///
/// @package heos-dial
///
/// @file HEOS tui
/// @copyright 2024-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use std::error;
use arc_swap::ArcSwap;
use heos_lib::{HeosDevice, HeosReply};
use ratatui::widgets::ListState;
use std::sync::Arc;
use heos_lib::heos_command::{HeosCommand, HeosCommandHandler};

pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub(crate) dev_list: Arc<ArcSwap<Vec<HeosDevice>>>,
    pub(crate) dev_list_state: ListState,
    pub(crate) group_list_state: ListState,
    pub running: bool,
}

impl App {
    pub(crate) fn new(dev_list: Arc<ArcSwap<Vec<HeosDevice>>>) -> App {
        Self {
            running: true,
            dev_list,
            dev_list_state: ListState::default(),
            group_list_state: ListState::default(),
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub(crate) fn select_none(&mut self) {
        self.dev_list_state.select(None);
    }

    pub(crate) fn select_next(&mut self) {
        self.dev_list_state.select_next();
    }

    pub(crate) fn select_previous(&mut self) {
        self.dev_list_state.select_previous();
    }

    pub(crate) fn select_first(&mut self) {
        self.dev_list_state.select_first();
    }

    pub(crate) fn select_last(&mut self) {
        self.dev_list_state.select_last();
    }

    pub(crate) fn set_volume(&mut self, step: i16) {
        if let Some(i) = self.dev_list_state.selected() {
            let dev_list = Arc::clone(&self.dev_list);

            tokio::spawn(async move {
                let swap_list = dev_list.swap(Arc::from(Vec::default()));
                let mut vec_list = swap_list.to_vec();

                let mut dev = vec_list.get_mut(i).unwrap();
                let new_level = i16::try_from(dev.volume).unwrap() + step;

                let level : u16 = if 0 > new_level {
                    0
                } else {
                    u16::try_from(new_level).unwrap()
                };

                let level_str = level.to_string();

                let cmd = HeosCommand::new()
                    .group("player")
                    .cmd("set_volume")
                    .attr("level", &*level_str);

                let reply = dev.send_command(&cmd).await.unwrap();

                if let HeosReply::Volume(success, _) = reply {
                    eprintln!("success={}, level={}", success, level);
                    if success {
                        dev.volume = level;

                        dev_list.swap(Arc::from(vec_list));
                    }
                }
            });
        }
    }

    pub(crate) fn toggle_status(&mut self) {
        if let Some(dev) = self.get_selected_device() {
            eprintln!("Selected status: {}", dev.stream.is_some());
        }
    }

    pub(crate) fn get_selected_device(&self) -> Option<HeosDevice> {
        if let Some(i) = self.dev_list_state.selected() {
            return self.dev_list.load().get(i).cloned();
        }

        None
    }
}
