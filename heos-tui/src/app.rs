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
use heos_lib::HeosDevice;
use ratatui::widgets::ListState;
use std::sync::Arc;

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

    pub(crate) fn increase_volume(&mut self) {
        eprintln!("Increase volume");
    }

    pub(crate) fn decrease_volume(&mut self) {
        eprintln!("Decrease volume");
    }

    pub(crate) fn toggle_status(&mut self) {
        if let Some(i) = self.dev_list_state.selected() {
            if let Some(item) = self.dev_list.load().get(i) {
                eprintln!("Selected status: {}", item.stream.is_some());
            }
        }
    }
}
