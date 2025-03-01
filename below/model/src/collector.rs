// Copyright (c) Facebook, Inc. and its affiliates.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::{Arc, Mutex};

use super::*;
use regex::Regex;
use slog::{self, error};
use std::path::{Path, PathBuf};

/// Collects data samples and maintains the latest data
pub struct Collector {
    cgroup_root: PathBuf,
    last: Option<(Sample, Instant)>,
    exit_data: Arc<Mutex<procfs::PidMap>>,
}

impl Collector {
    pub fn new(exit_data: Arc<Mutex<procfs::PidMap>>) -> Collector {
        Collector::new_with_cgroup_root(
            Path::new(cgroupfs::DEFAULT_CG_ROOT).to_path_buf(),
            exit_data,
        )
    }

    pub fn new_with_cgroup_root(
        cgroup_root: PathBuf,
        exit_data: Arc<Mutex<procfs::PidMap>>,
    ) -> Collector {
        Collector {
            cgroup_root,
            last: None,
            exit_data,
        }
    }

    /// Collect a new `Sample`, returning an updated Model
    pub fn update_model(&mut self, logger: &slog::Logger) -> Result<Model> {
        let now = Instant::now();
        let sample = collect_sample(
            &self.cgroup_root,
            &self.exit_data,
            true,
            logger,
            false,
            &None,
        )?;
        let last = self.last.replace((sample, now));
        let model = Model::new(
            SystemTime::now(),
            &self.last.as_ref().unwrap().0,
            last.as_ref().map(|(s, i)| (s, now.duration_since(*i))),
        );
        Ok(model)
    }
}

pub fn opt_add<T: std::ops::Add<T, Output = T>>(a: Option<T>, b: Option<T>) -> Option<T> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a + b),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        _ => None,
    }
}

pub fn opt_multiply<S: Sized + std::ops::Mul<T, Output = S>, T: Sized>(
    a: Option<S>,
    b: Option<T>,
) -> Option<S> {
    a.and_then(|x| b.map(|y| x * y))
}

pub fn get_hostname() -> Result<String> {
    if let Ok(h) = hostname::get() {
        if let Ok(s) = h.into_string() {
            return Ok(s);
        }
    }
    Err(anyhow!("Could not get hostname"))
}

#[cfg(fbcode_build)]
pub fn get_os_release() -> Result<String> {
    std::fs::read_to_string("/etc/centos-release")
        .context("Fail to get centos release")
        .map(|o| o.trim_matches('\n').trim().into())
}

use os_info as _; // So RUSTFIXDEPS doesn't complain.
#[cfg(not(fbcode_build))]
pub fn get_os_release() -> Result<String> {
    let info = os_info::get();
    Ok(format!(
        "{} {} {}",
        info.os_type(),
        info.version(),
        info.bitness()
    ))
}

fn merge_procfs_and_exit_data(
    mut procfs_data: procfs::PidMap,
    exit_data: procfs::PidMap,
) -> procfs::PidMap {
    exit_data
        .iter()
        // If `procfs_data` already has the pid, then we use the procfs data because the time delta
        // between the two collection points is negligible and procfs collected data is more
        // complete.
        .for_each(|entry| {
            if !procfs_data.contains_key(entry.0) {
                procfs_data.insert(*entry.0, entry.1.clone());
            }
        });

    procfs_data
}

/// This function will test if all field of DiskStat are zero, if so we will need to skip
/// this sample inside collector.
fn is_all_zero_disk_stats(disk_stats: &procfs::DiskStat) -> bool {
    disk_stats.read_completed == Some(0)
        && disk_stats.write_completed == Some(0)
        && disk_stats.discard_completed == Some(0)
        && disk_stats.read_merged == Some(0)
        && disk_stats.read_sectors == Some(0)
        && disk_stats.time_spend_read_ms == Some(0)
        && disk_stats.write_merged == Some(0)
        && disk_stats.write_sectors == Some(0)
        && disk_stats.time_spend_write_ms == Some(0)
        && disk_stats.discard_merged == Some(0)
        && disk_stats.discard_sectors == Some(0)
        && disk_stats.time_spend_discard_ms == Some(0)
}

pub fn collect_sample(
    cgroup_root: &PathBuf,
    exit_data: &Arc<Mutex<procfs::PidMap>>,
    collect_io_stat: bool,
    logger: &slog::Logger,
    disable_disk_stat: bool,
    cgroup_re: &Option<Regex>,
) -> Result<Sample> {
    let mut reader = procfs::ProcReader::new();

    // Take mutex, then take all values out of shared map and replace with default map
    //
    // NB: unconditionally drain the exit buffer otherwise we can leak the entries
    let exit_pidmap =
        std::mem::take(&mut *exit_data.lock().expect("tried to acquire poisoned lock"));

    Ok(Sample {
        cgroup: collect_cgroup_sample(
            &cgroupfs::CgroupReader::new(cgroup_root.to_owned())?,
            collect_io_stat,
            logger,
            &cgroup_re,
        )?,
        processes: merge_procfs_and_exit_data(
            reader
                .read_all_pids()?
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            exit_pidmap,
        ),
        netstats: match procfs::NetReader::new().and_then(|v| v.read_netstat()) {
            Ok(ns) => ns.into(),
            Err(e) => {
                error!(logger, "{:#}", e);
                Default::default()
            }
        },
        system: SystemSample {
            stat: reader.read_stat()?.into(),
            meminfo: reader.read_meminfo()?.into(),
            vmstat: reader.read_vmstat()?.into(),
            hostname: get_hostname()?,
            kernel_version: match reader.read_kernel_version() {
                Ok(k) => Some(k),
                Err(e) => {
                    error!(logger, "{:#}", e);
                    None
                }
            },
            os_release: match get_os_release() {
                Ok(o) => Some(o),
                Err(e) => {
                    error!(logger, "{:#}", e);
                    None
                }
            },
            disks: match (disable_disk_stat, reader.read_disk_stats()) {
                (false, Ok(disks)) => disks
                    .into_iter()
                    .map(|(disk_name, disk_stat)| (disk_name, disk_stat.into()))
                    .filter(|(disk_name, disk_stat)| {
                        if disk_name.starts_with("ram") || disk_name.starts_with("loop") {
                            return false;
                        }

                        !is_all_zero_disk_stats(&disk_stat)
                    })
                    .collect(),
                (false, Err(e)) => {
                    error!(logger, "{:#}", e);
                    Default::default()
                }
                (true, _) => Default::default(),
            },
        },
    })
}

/// cgroupfs can give us a NotFound error if the cgroup doesn't have
/// the relevant stat file (e.g. if it is the root cgroup). We
/// translate that into `None` so that other errors are propagated,
/// but omitted data is allowed.
///
/// This method just does that translation for us.
fn wrap<S: Sized>(
    v: std::result::Result<S, cgroupfs::Error>,
) -> std::result::Result<Option<S>, cgroupfs::Error> {
    if let Err(cgroupfs::Error::IoError(_, ref e)) = v {
        if e.kind() == std::io::ErrorKind::NotFound {
            return Ok(None);
        }
        if e.kind() == std::io::ErrorKind::Other {
            if let Some(errno) = e.raw_os_error() {
                if errno == /* ENODEV */ 19 {
                    // If the cgroup is removed after a control file is opened,
                    // ENODEV is returned. Ignore it.
                    return Ok(None);
                }
            }
        }
    }
    v.map(Some)
}

/// As above, but in addition, io.stat can have broken formatting due
/// to a kernel bug which will not output more than one page. In such
/// cases we should not fail all data collection, but just omit the io
/// data.
fn io_stat_wrap<S: Sized>(
    v: std::result::Result<S, cgroupfs::Error>,
) -> std::result::Result<Option<S>, cgroupfs::Error> {
    match wrap(v) {
        Err(cgroupfs::Error::InvalidFileFormat(_)) => Ok(None),
        Err(cgroupfs::Error::UnexpectedLine(_, _)) => Ok(None),
        wrapped => wrapped,
    }
}

fn collect_cgroup_sample(
    reader: &cgroupfs::CgroupReader,
    collect_io_stat: bool,
    logger: &slog::Logger,
    cgroup_re: &Option<Regex>,
) -> Result<CgroupSample> {
    let io_stat = if collect_io_stat {
        io_stat_wrap(reader.read_io_stat())?
    } else {
        None
    };
    Ok(CgroupSample {
        cpu_stat: wrap(reader.read_cpu_stat())?.map(Into::into),
        io_stat: io_stat.map(|m| m.into_iter().map(|(k, v)| (k, v.into())).collect()),
        memory_current: wrap(reader.read_memory_current().map(|v| v as i64))?,
        memory_stat: wrap(reader.read_memory_stat())?.map(Into::into),
        pressure: wrap(reader.read_pressure())?.map(Into::into),
        // We transpose at the end here to convert the
        // Option<Result<BTreeMap... into Result<Option<BTreeMap and
        // then bail any errors with `?` - leaving us with the
        // Option<BTreeMap...
        //
        // The only case this can be None is if the cgroup no longer
        // exists - this is consistent with the above members
        children: wrap(reader.child_cgroup_iter())
            .context("Failed to get iterator over cgroup children")?
            .map(|child_iter| {
                child_iter
                    .filter(|child| {
                        if let Some(cgroup_re) = cgroup_re.as_ref() {
                            !cgroup_re.is_match(&child.name().to_string_lossy())
                        } else {
                            true
                        }
                    })
                    .map(|child| {
                        collect_cgroup_sample(&child, collect_io_stat, logger, cgroup_re).map(
                            |child_sample| {
                                (
                                    child
                                        .name()
                                        .file_name()
                                        .expect("Unexpected .. in cgroup path")
                                        .to_string_lossy()
                                        .to_string(),
                                    child_sample,
                                )
                            },
                        )
                    })
                    .collect::<Result<BTreeMap<String, CgroupSample>>>()
            })
            .transpose()?,
        memory_swap_current: wrap(reader.read_memory_swap_current().map(|v| v as i64))?,
        memory_high: reader.read_memory_high()?.map(Into::into),
        memory_events: wrap(reader.read_memory_events())?.map(Into::into),
        inode_number: match reader.read_inode_number() {
            Ok(st_ino) => Some(st_ino as i64),
            Err(e) => {
                error!(logger, "Fail to collect inode number: {:#}", e);
                None
            }
        },
    })
}

macro_rules! usec_pct {
    ($a_opt:expr, $b_opt:expr, $delta:expr) => {{
        let mut ret = None;
        if let (Some(a), Some(b)) = ($a_opt, $b_opt) {
            if a <= b {
                ret = Some((b - a) as f64 * 100.0 / $delta.as_micros() as f64);
            }
        }
        ret
    }};
}

macro_rules! count_per_sec {
    ($a_opt:expr, $b_opt:expr, $delta:expr) => {{
        let mut ret = None;
        if let (Some(a), Some(b)) = ($a_opt, $b_opt) {
            if a <= b {
                ret = Some((b - a) as f64 / $delta.as_secs_f64());
            }
        }
        ret
    }};
    ($a_opt:expr, $b_opt:expr, $delta:expr, $target_type:ty) => {{
        let mut ret = None;
        if let (Some(a), Some(b)) = ($a_opt, $b_opt) {
            if a <= b {
                ret = Some(((b - a) as f64 / $delta.as_secs_f64()).ceil() as $target_type);
            }
        }
        ret
    }};
}

#[allow(unused)]
macro_rules! get_option_rate {
    ($key:ident, $sample:ident, $last:ident) => {
        $last
            .map(|(l, d)| {
                count_per_sec!(l.$key.map(|s| s as u64), $sample.$key.map(|s| s as u64), d)
            })
            .unwrap_or_default()
            .map(|s| s as u64)
    };
}
