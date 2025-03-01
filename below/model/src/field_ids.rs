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

/// All available field id representations of the base `Model` struct.
///
/// Most can be parsed as `ModelFieldId` and used to query a Model. Some are
/// parameterized field ids (with angle bracketed variable like <idx>) that must
/// have the variable substituted with specific values to be parsed correctly.
///
/// For example, <idx> must be some zero-indexed Vec index, <cgroup_path> must
/// be a path and <key> could be pid, disk name, iface name etc.
///
/// This list also servers as documentation for available field ids that could
/// be used in other below crates. A test ensures that this list is up-to-date.
pub const MODEL_FIELD_IDS: &[&'static str] = &[
    "system.hostname",
    "system.kernel_version",
    "system.os_release",
    "system.stat.total_interrupt_ct",
    "system.stat.context_switches",
    "system.stat.boot_time_epoch_secs",
    "system.stat.total_processes",
    "system.stat.running_processes",
    "system.stat.blocked_processes",
    "system.cpu.idx",
    "system.cpu.usage_pct",
    "system.cpu.user_pct",
    "system.cpu.system_pct",
    "system.cpu.idle_pct",
    "system.cpu.nice_pct",
    "system.cpu.iowait_pct",
    "system.cpu.irq_pct",
    "system.cpu.softirq_pct",
    "system.cpu.stolen_pct",
    "system.cpu.guest_pct",
    "system.cpu.guest_nice_pct",
    "system.cpus.<idx>.idx",
    "system.cpus.<idx>.usage_pct",
    "system.cpus.<idx>.user_pct",
    "system.cpus.<idx>.system_pct",
    "system.cpus.<idx>.idle_pct",
    "system.cpus.<idx>.nice_pct",
    "system.cpus.<idx>.iowait_pct",
    "system.cpus.<idx>.irq_pct",
    "system.cpus.<idx>.softirq_pct",
    "system.cpus.<idx>.stolen_pct",
    "system.cpus.<idx>.guest_pct",
    "system.cpus.<idx>.guest_nice_pct",
    "system.mem.total",
    "system.mem.free",
    "system.mem.available",
    "system.mem.buffers",
    "system.mem.cached",
    "system.mem.swap_cached",
    "system.mem.active",
    "system.mem.inactive",
    "system.mem.anon",
    "system.mem.file",
    "system.mem.unevictable",
    "system.mem.mlocked",
    "system.mem.swap_total",
    "system.mem.swap_free",
    "system.mem.dirty",
    "system.mem.writeback",
    "system.mem.anon_pages",
    "system.mem.mapped",
    "system.mem.shmem",
    "system.mem.kreclaimable",
    "system.mem.slab",
    "system.mem.slab_reclaimable",
    "system.mem.slab_unreclaimable",
    "system.mem.kernel_stack",
    "system.mem.page_tables",
    "system.mem.anon_huge_pages_bytes",
    "system.mem.shmem_huge_pages_bytes",
    "system.mem.file_huge_pages_bytes",
    "system.mem.hugetlb",
    "system.mem.cma_total",
    "system.mem.cma_free",
    "system.mem.vmalloc_total",
    "system.mem.vmalloc_used",
    "system.mem.vmalloc_chunk",
    "system.mem.direct_map4k",
    "system.mem.direct_map2m",
    "system.mem.direct_map1g",
    "system.vm.pgpgin_per_sec",
    "system.vm.pgpgout_per_sec",
    "system.vm.pswpin_per_sec",
    "system.vm.pswpout_per_sec",
    "system.vm.pgsteal_kswapd",
    "system.vm.pgsteal_direct",
    "system.vm.pgscan_kswapd",
    "system.vm.pgscan_direct",
    "system.vm.oom_kill",
    "system.disks.<key>.name",
    "system.disks.<key>.read_bytes_per_sec",
    "system.disks.<key>.write_bytes_per_sec",
    "system.disks.<key>.discard_bytes_per_sec",
    "system.disks.<key>.disk_total_bytes_per_sec",
    "system.disks.<key>.read_completed",
    "system.disks.<key>.read_merged",
    "system.disks.<key>.read_sectors",
    "system.disks.<key>.time_spend_read_ms",
    "system.disks.<key>.write_completed",
    "system.disks.<key>.write_merged",
    "system.disks.<key>.write_sectors",
    "system.disks.<key>.time_spend_write_ms",
    "system.disks.<key>.discard_completed",
    "system.disks.<key>.discard_merged",
    "system.disks.<key>.discard_sectors",
    "system.disks.<key>.time_spend_discard_ms",
    "system.disks.<key>.major",
    "system.disks.<key>.minor",
    "cgroup.[path:/<cgroup_path>/.]name",
    "cgroup.[path:/<cgroup_path>/.]full_path",
    "cgroup.[path:/<cgroup_path>/.]inode_number",
    "cgroup.[path:/<cgroup_path>/.]cpu.usage_pct",
    "cgroup.[path:/<cgroup_path>/.]cpu.user_pct",
    "cgroup.[path:/<cgroup_path>/.]cpu.system_pct",
    "cgroup.[path:/<cgroup_path>/.]cpu.nr_periods_per_sec",
    "cgroup.[path:/<cgroup_path>/.]cpu.nr_throttled_per_sec",
    "cgroup.[path:/<cgroup_path>/.]cpu.throttled_pct",
    "cgroup.[path:/<cgroup_path>/.]mem.total",
    "cgroup.[path:/<cgroup_path>/.]mem.swap",
    "cgroup.[path:/<cgroup_path>/.]mem.anon",
    "cgroup.[path:/<cgroup_path>/.]mem.file",
    "cgroup.[path:/<cgroup_path>/.]mem.kernel_stack",
    "cgroup.[path:/<cgroup_path>/.]mem.slab",
    "cgroup.[path:/<cgroup_path>/.]mem.sock",
    "cgroup.[path:/<cgroup_path>/.]mem.shmem",
    "cgroup.[path:/<cgroup_path>/.]mem.file_mapped",
    "cgroup.[path:/<cgroup_path>/.]mem.file_dirty",
    "cgroup.[path:/<cgroup_path>/.]mem.file_writeback",
    "cgroup.[path:/<cgroup_path>/.]mem.anon_thp",
    "cgroup.[path:/<cgroup_path>/.]mem.inactive_anon",
    "cgroup.[path:/<cgroup_path>/.]mem.active_anon",
    "cgroup.[path:/<cgroup_path>/.]mem.inactive_file",
    "cgroup.[path:/<cgroup_path>/.]mem.active_file",
    "cgroup.[path:/<cgroup_path>/.]mem.unevictable",
    "cgroup.[path:/<cgroup_path>/.]mem.slab_reclaimable",
    "cgroup.[path:/<cgroup_path>/.]mem.slab_unreclaimable",
    "cgroup.[path:/<cgroup_path>/.]mem.pgfault",
    "cgroup.[path:/<cgroup_path>/.]mem.pgmajfault",
    "cgroup.[path:/<cgroup_path>/.]mem.workingset_refault",
    "cgroup.[path:/<cgroup_path>/.]mem.workingset_activate",
    "cgroup.[path:/<cgroup_path>/.]mem.workingset_nodereclaim",
    "cgroup.[path:/<cgroup_path>/.]mem.pgrefill",
    "cgroup.[path:/<cgroup_path>/.]mem.pgscan",
    "cgroup.[path:/<cgroup_path>/.]mem.pgsteal",
    "cgroup.[path:/<cgroup_path>/.]mem.pgactivate",
    "cgroup.[path:/<cgroup_path>/.]mem.pgdeactivate",
    "cgroup.[path:/<cgroup_path>/.]mem.pglazyfree",
    "cgroup.[path:/<cgroup_path>/.]mem.pglazyfreed",
    "cgroup.[path:/<cgroup_path>/.]mem.thp_fault_alloc",
    "cgroup.[path:/<cgroup_path>/.]mem.thp_collapse_alloc",
    "cgroup.[path:/<cgroup_path>/.]mem.memory_high",
    "cgroup.[path:/<cgroup_path>/.]mem.events_low",
    "cgroup.[path:/<cgroup_path>/.]mem.events_high",
    "cgroup.[path:/<cgroup_path>/.]mem.events_max",
    "cgroup.[path:/<cgroup_path>/.]mem.events_oom",
    "cgroup.[path:/<cgroup_path>/.]mem.events_oom_kill",
    "cgroup.[path:/<cgroup_path>/.]io_details.<key>.rbytes_per_sec",
    "cgroup.[path:/<cgroup_path>/.]io_details.<key>.wbytes_per_sec",
    "cgroup.[path:/<cgroup_path>/.]io_details.<key>.rios_per_sec",
    "cgroup.[path:/<cgroup_path>/.]io_details.<key>.wios_per_sec",
    "cgroup.[path:/<cgroup_path>/.]io_details.<key>.dbytes_per_sec",
    "cgroup.[path:/<cgroup_path>/.]io_details.<key>.dios_per_sec",
    "cgroup.[path:/<cgroup_path>/.]io_details.<key>.rwbytes_per_sec",
    "cgroup.[path:/<cgroup_path>/.]io.rbytes_per_sec",
    "cgroup.[path:/<cgroup_path>/.]io.wbytes_per_sec",
    "cgroup.[path:/<cgroup_path>/.]io.rios_per_sec",
    "cgroup.[path:/<cgroup_path>/.]io.wios_per_sec",
    "cgroup.[path:/<cgroup_path>/.]io.dbytes_per_sec",
    "cgroup.[path:/<cgroup_path>/.]io.dios_per_sec",
    "cgroup.[path:/<cgroup_path>/.]io.rwbytes_per_sec",
    "cgroup.[path:/<cgroup_path>/.]pressure.cpu_some_pct",
    "cgroup.[path:/<cgroup_path>/.]pressure.io_some_pct",
    "cgroup.[path:/<cgroup_path>/.]pressure.io_full_pct",
    "cgroup.[path:/<cgroup_path>/.]pressure.memory_some_pct",
    "cgroup.[path:/<cgroup_path>/.]pressure.memory_full_pct",
    "process.processes.<key>.pid",
    "process.processes.<key>.ppid",
    "process.processes.<key>.comm",
    "process.processes.<key>.state",
    "process.processes.<key>.uptime_secs",
    "process.processes.<key>.cgroup",
    "process.processes.<key>.io.rbytes_per_sec",
    "process.processes.<key>.io.wbytes_per_sec",
    "process.processes.<key>.io.rwbytes_per_sec",
    "process.processes.<key>.mem.minorfaults_per_sec",
    "process.processes.<key>.mem.majorfaults_per_sec",
    "process.processes.<key>.mem.rss_bytes",
    "process.processes.<key>.mem.vm_size",
    "process.processes.<key>.mem.lock",
    "process.processes.<key>.mem.pin",
    "process.processes.<key>.mem.anon",
    "process.processes.<key>.mem.file",
    "process.processes.<key>.mem.shmem",
    "process.processes.<key>.mem.pte",
    "process.processes.<key>.mem.swap",
    "process.processes.<key>.mem.huge_tlb",
    "process.processes.<key>.cpu.usage_pct",
    "process.processes.<key>.cpu.user_pct",
    "process.processes.<key>.cpu.system_pct",
    "process.processes.<key>.cpu.num_threads",
    "process.processes.<key>.cmdline",
    "process.processes.<key>.exe_path",
    "network.interfaces.<key>.interface",
    "network.interfaces.<key>.rx_bytes_per_sec",
    "network.interfaces.<key>.tx_bytes_per_sec",
    "network.interfaces.<key>.throughput_per_sec",
    "network.interfaces.<key>.rx_packets_per_sec",
    "network.interfaces.<key>.tx_packets_per_sec",
    "network.interfaces.<key>.collisions",
    "network.interfaces.<key>.multicast",
    "network.interfaces.<key>.rx_bytes",
    "network.interfaces.<key>.rx_compressed",
    "network.interfaces.<key>.rx_crc_errors",
    "network.interfaces.<key>.rx_dropped",
    "network.interfaces.<key>.rx_errors",
    "network.interfaces.<key>.rx_fifo_errors",
    "network.interfaces.<key>.rx_frame_errors",
    "network.interfaces.<key>.rx_length_errors",
    "network.interfaces.<key>.rx_missed_errors",
    "network.interfaces.<key>.rx_nohandler",
    "network.interfaces.<key>.rx_over_errors",
    "network.interfaces.<key>.rx_packets",
    "network.interfaces.<key>.tx_aborted_errors",
    "network.interfaces.<key>.tx_bytes",
    "network.interfaces.<key>.tx_carrier_errors",
    "network.interfaces.<key>.tx_compressed",
    "network.interfaces.<key>.tx_dropped",
    "network.interfaces.<key>.tx_errors",
    "network.interfaces.<key>.tx_fifo_errors",
    "network.interfaces.<key>.tx_heartbeat_errors",
    "network.interfaces.<key>.tx_packets",
    "network.interfaces.<key>.tx_window_errors",
    "network.tcp.active_opens_per_sec",
    "network.tcp.passive_opens_per_sec",
    "network.tcp.attempt_fails_per_sec",
    "network.tcp.estab_resets_per_sec",
    "network.tcp.curr_estab_conn",
    "network.tcp.in_segs_per_sec",
    "network.tcp.out_segs_per_sec",
    "network.tcp.retrans_segs_per_sec",
    "network.tcp.retrans_segs",
    "network.tcp.in_errs",
    "network.tcp.out_rsts_per_sec",
    "network.tcp.in_csum_errors",
    "network.ip.forwarding_pkts_per_sec",
    "network.ip.in_receives_pkts_per_sec",
    "network.ip.forw_datagrams_per_sec",
    "network.ip.in_discards_pkts_per_sec",
    "network.ip.in_delivers_pkts_per_sec",
    "network.ip.out_requests_per_sec",
    "network.ip.out_discards_pkts_per_sec",
    "network.ip.out_no_routes_pkts_per_sec",
    "network.ip.in_mcast_pkts_per_sec",
    "network.ip.out_mcast_pkts_per_sec",
    "network.ip.in_bcast_pkts_per_sec",
    "network.ip.out_bcast_pkts_per_sec",
    "network.ip.in_octets_per_sec",
    "network.ip.out_octets_per_sec",
    "network.ip.in_mcast_octets_per_sec",
    "network.ip.out_mcast_octets_per_sec",
    "network.ip.in_bcast_octets_per_sec",
    "network.ip.out_bcast_octets_per_sec",
    "network.ip.in_no_ect_pkts_per_sec",
    "network.ip6.in_receives_pkts_per_sec",
    "network.ip6.in_hdr_errors",
    "network.ip6.in_no_routes_pkts_per_sec",
    "network.ip6.in_addr_errors",
    "network.ip6.in_discards_pkts_per_sec",
    "network.ip6.in_delivers_pkts_per_sec",
    "network.ip6.out_forw_datagrams_per_sec",
    "network.ip6.out_requests_per_sec",
    "network.ip6.out_no_routes_pkts_per_sec",
    "network.ip6.in_mcast_pkts_per_sec",
    "network.ip6.out_mcast_pkts_per_sec",
    "network.ip6.in_octets_per_sec",
    "network.ip6.out_octets_per_sec",
    "network.ip6.in_mcast_octets_per_sec",
    "network.ip6.out_mcast_octets_per_sec",
    "network.ip6.in_bcast_octets_per_sec",
    "network.ip6.out_bcast_octets_per_sec",
    "network.icmp.in_msgs_per_sec",
    "network.icmp.in_errors",
    "network.icmp.in_dest_unreachs",
    "network.icmp.out_msgs_per_sec",
    "network.icmp.out_errors",
    "network.icmp.out_dest_unreachs",
    "network.icmp6.in_msgs_per_sec",
    "network.icmp6.in_errors",
    "network.icmp6.in_dest_unreachs",
    "network.icmp6.out_msgs_per_sec",
    "network.icmp6.out_errors",
    "network.icmp6.out_dest_unreachs",
    "network.udp.in_datagrams_pkts_per_sec",
    "network.udp.no_ports",
    "network.udp.in_errors",
    "network.udp.out_datagrams_pkts_per_sec",
    "network.udp.rcvbuf_errors",
    "network.udp.sndbuf_errors",
    "network.udp.ignored_multi",
    "network.udp6.in_datagrams_pkts_per_sec",
    "network.udp6.no_ports",
    "network.udp6.in_errors",
    "network.udp6.out_datagrams_pkts_per_sec",
    "network.udp6.rcvbuf_errors",
    "network.udp6.sndbuf_errors",
    "network.udp6.in_csum_errors",
    "network.udp6.ignored_multi",
];
