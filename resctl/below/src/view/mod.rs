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

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use ::cursive::event::{Event, EventResult, EventTrigger};
use ::cursive::theme::{BaseColor, Color, PaletteColor};
use ::cursive::view::Identifiable;
use ::cursive::views::{LinearLayout, NamedView, OnEventView, Panel, ResizedView, StackView};
use ::cursive::Cursive;
use anyhow::Result;

use crate::store::Direction;
use crate::Advance;

mod cgroup_view;
mod filter_popup;
mod help_menu;
mod process_view;
mod status_bar;
mod system_view;

pub struct View {
    inner: Cursive,
}

// Invoked either when the data view was explicitly advanced, or
// periodically (during live mode)
fn refresh(c: &mut Cursive) {
    status_bar::refresh(c);
    system_view::refresh(c);
    process_view::refresh(c);
    cgroup_view::refresh(c);
}

macro_rules! advance {
    ($c:ident, $adv:ident, $dir:expr) => {
        match $adv.advance($dir) {
            Some(data) => {
                $c.user_data::<ViewState>().expect("No user data set").model = data;
                refresh($c);
            }
            None => (),
        }
    };
}

fn update_sort_order(c: &mut Cursive, sort_order: SortOrder) {
    let vs = &mut c.user_data::<ViewState>().expect("No user data");
    if vs.sort_order != sort_order {
        vs.sort_order = sort_order;
        refresh(c);
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortOrder {
    PID,
    Name,
    CPU,
    Memory,
    Disk,
}

#[derive(Clone)]
pub enum MainViewState {
    Cgroup,
    Process,
    ProcessZoomedIntoCgroup(String),
}

pub struct ViewState {
    pub model: crate::model::Model,
    pub sort_order: SortOrder,
    pub collapsed_cgroups: HashSet<String>,
    pub current_selected_cgroup: String,
    pub main_view_state: MainViewState,
    pub cgroup_filter: Option<String>,
    pub process_filter: Option<String>,
}

impl View {
    pub fn new(model: crate::model::Model) -> View {
        let mut inner = Cursive::default();
        inner.set_user_data(ViewState {
            model,
            sort_order: SortOrder::PID,
            collapsed_cgroups: HashSet::new(),
            current_selected_cgroup: "<root>".to_string(),
            main_view_state: MainViewState::Cgroup,
            cgroup_filter: None,
            process_filter: None,
        });
        View { inner }
    }

    pub fn cb_sink(&mut self) -> &::cursive::CbSink {
        self.inner.set_fps(4);
        self.inner.cb_sink()
    }

    pub fn register_advance(&mut self, advance: Advance) {
        let rc = Rc::new(RefCell::new(advance));

        let forward_rc = rc.clone();
        self.inner.add_global_callback('t', move |c| {
            let mut adv = forward_rc.borrow_mut();
            advance!(c, adv, Direction::Forward);
        });

        let reverse_rc = rc.clone();
        self.inner.add_global_callback('T', move |c| {
            let mut adv = reverse_rc.borrow_mut();
            advance!(c, adv, Direction::Reverse);
        });
    }

    pub fn run(&mut self) -> Result<()> {
        let mut theme = self.inner.current_theme().clone();
        theme.palette[PaletteColor::Background] = Color::TerminalDefault;
        theme.palette[PaletteColor::View] = Color::TerminalDefault;
        theme.palette[PaletteColor::Primary] = Color::TerminalDefault;
        theme.palette[PaletteColor::Highlight] = Color::Dark(BaseColor::Cyan);
        theme.palette[PaletteColor::HighlightText] = Color::Dark(BaseColor::Black);
        theme.shadow = false;

        self.inner.set_theme(theme);

        self.inner.add_global_callback('q', Cursive::quit);
        self.inner.add_global_callback('?', |s| {
            s.add_fullscreen_layer(ResizedView::with_full_screen(
                OnEventView::new(help_menu::new()).on_event(EventTrigger::from('q').or('?'), |s| {
                    s.pop_layer();
                }),
            ))
        });
        self.inner
            .add_global_callback(Event::CtrlChar('z'), |_| unsafe {
                if libc::raise(libc::SIGTSTP) != 0 {
                    panic!("failed to SIGTSTP self");
                }
            });
        self.inner.add_global_callback(Event::Refresh, |c| {
            refresh(c);
        });
        self.inner
            .add_global_callback('P', |c| update_sort_order(c, SortOrder::PID));
        self.inner
            .add_global_callback('C', |c| update_sort_order(c, SortOrder::CPU));
        self.inner
            .add_global_callback('N', |c| update_sort_order(c, SortOrder::Name));
        self.inner
            .add_global_callback('M', |c| update_sort_order(c, SortOrder::Memory));
        self.inner
            .add_global_callback('D', |c| update_sort_order(c, SortOrder::Disk));

        let status_bar = status_bar::new(&mut self.inner);
        let system_view = system_view::new(&mut self.inner);
        let process_view = process_view::new(&mut self.inner);
        let cgroup_view = cgroup_view::new(&mut self.inner);
        self.inner.add_fullscreen_layer(
            StackView::new().fullscreen_layer(ResizedView::with_full_screen(
                LinearLayout::vertical()
                    .child(Panel::new(status_bar))
                    .child(Panel::new(system_view))
                    .child(
                        OnEventView::new(
                            StackView::new()
                                .fullscreen_layer(ResizedView::with_full_screen(
                                    Panel::new(process_view).with_name("process_view_panel"),
                                ))
                                .fullscreen_layer(ResizedView::with_full_screen(
                                    Panel::new(cgroup_view).with_name("cgroup_view_panel"),
                                ))
                                .with_name("main_view_stack"),
                        )
                        .on_pre_event_inner('p', |stack, _| {
                            let position = (*stack.get_mut())
                                .find_layer_from_name("process_view_panel")
                                .expect("Failed to find process view");
                            (*stack.get_mut()).move_to_front(position);

                            Some(EventResult::with_cb(|c| {
                                let view_state = c
                                    .user_data::<ViewState>()
                                    .expect("No data stored in Cursive object!");
                                view_state.main_view_state = MainViewState::Process;
                            }))
                        })
                        .on_pre_event_inner('c', |stack, _| {
                            let position = (*stack.get_mut())
                                .find_layer_from_name("cgroup_view_panel")
                                .expect("Failed to find cgroup view");
                            (*stack.get_mut()).move_to_front(position);

                            Some(EventResult::with_cb(|c| {
                                let view_state = c
                                    .user_data::<ViewState>()
                                    .expect("No data stored in Cursive object!");
                                view_state.main_view_state = MainViewState::Cgroup;
                            }))
                        })
                        .on_pre_event('z', |c| {
                            let current_selection = c
                                .user_data::<ViewState>()
                                .expect("No data stored in Cursive object!")
                                .current_selected_cgroup
                                .clone();

                            let current_state = c
                                .user_data::<ViewState>()
                                .expect("No data stored in Cursive object!")
                                .main_view_state
                                .clone();

                            let next_state = match current_state {
                                // Pressing 'z' in zoomed view should remove zoom
                                // and bring user back to cgroup view
                                MainViewState::ProcessZoomedIntoCgroup(_) => MainViewState::Cgroup,
                                MainViewState::Cgroup => MainViewState::ProcessZoomedIntoCgroup(
                                    current_selection.clone(),
                                ),
                                // Pressing 'z' in process view should do nothing
                                MainViewState::Process => MainViewState::Process,
                            };

                            c.call_on_name(
                                "main_view_stack",
                                |stack: &mut NamedView<StackView>| {
                                    match &next_state {
                                        MainViewState::Process
                                        | MainViewState::ProcessZoomedIntoCgroup(_) => {
                                            // Bring process_view to front
                                            let process_pos = (*stack.get_mut())
                                                .find_layer_from_name("process_view_panel")
                                                .expect("Failed to find process view");
                                            (*stack.get_mut()).move_to_front(process_pos);
                                        }
                                        MainViewState::Cgroup => {
                                            // Bring cgroup_view to front
                                            let cgroup_pos = (*stack.get_mut())
                                                .find_layer_from_name("cgroup_view_panel")
                                                .expect("Failed to find cgroup view");
                                            (*stack.get_mut()).move_to_front(cgroup_pos);
                                        }
                                    }
                                },
                            )
                            .expect("failed to find main_view_stack");

                            // Set next state
                            c.user_data::<ViewState>()
                                .expect("No data stored in Cursive object!")
                                .main_view_state = next_state;

                            // Redraw screen now so we don't have to wait until next tick
                            // to get the zoomed view
                            refresh(c);
                        })
                        .with_name("dynamic_view"),
                    ),
            )),
        );

        self.inner
            .focus_name("dynamic_view")
            .expect("Could not set focus at initialization!");

        self.inner.run();

        Ok(())
    }
}
