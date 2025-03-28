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

use std::{error, fmt};
use std::fmt::{Display, Formatter};
use heos_lib::{HeosDevice, HeosGroup, HeosReply};
use ratatui::widgets::ListState;
use std::sync::{Arc, RwLock};
use heos_lib::heos_command::{HeosCommand, HeosCommandHandler};
use log::{error, info};

pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub(crate) enum PlayerState {
    Play,
    Stop,
}

impl Display for PlayerState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default)]
pub(crate) enum Focus {
    #[default]
    Devices,

    Groups,
}

impl Display for Focus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct App {
    pub(crate) dev_list: Arc<RwLock<Vec<HeosDevice>>>,
    pub(crate) group_list: Arc<RwLock<Vec<HeosGroup>>>,
    pub(crate) dev_list_state: ListState,
    pub(crate) group_list_state: ListState,
    pub(crate) focus_state: Focus,
    pub running: bool,
}

impl App {
    pub(crate) fn new(dev_list: Arc<RwLock<Vec<HeosDevice>>>, group_list: Arc<RwLock<Vec<HeosGroup>>>) -> App {
        Self {
            running: true,
            dev_list,
            group_list,
            dev_list_state: ListState::default(),
            group_list_state: ListState::default(),
            focus_state: Focus::default(),
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub(crate) fn reset(&mut self) {
        match self.focus_state {
            Focus::Devices => self.group_list_state.select(None),
            Focus::Groups => self.dev_list_state.select(None),
        }
    }

    pub(crate) fn select_none(&mut self) {
        match self.focus_state {
            Focus::Devices => self.dev_list_state.select(None),
            Focus::Groups => self.group_list_state.select(None),
        }
        self.reset()
    }

    pub(crate) fn select_next(&mut self) {
        match self.focus_state {
            Focus::Devices => self.dev_list_state.select_next(),
            Focus::Groups => self.group_list_state.select_next(),
        }
        self.reset()
    }

    pub(crate) fn select_previous(&mut self) {
        match self.focus_state {
            Focus::Devices => self.dev_list_state.select_previous(),
            Focus::Groups => self.group_list_state.select_previous(),
        }
        self.reset()
    }

    pub(crate) fn select_first(&mut self) {
        match self.focus_state {
            Focus::Devices => self.dev_list_state.select_first(),
            Focus::Groups => self.group_list_state.select_first(),
        }
        self.reset()
    }

    pub(crate) fn select_last(&mut self) {
        match self.focus_state {
            Focus::Devices => self.dev_list_state.select_last(),
            Focus::Groups => self.group_list_state.select_last(),
        }
        self.reset()
    }

    pub(crate) fn select_list(&mut self, focus_state: Focus) {
        self.focus_state = focus_state;
    }

    pub(crate) fn set_volume(&mut self, step: i16) {
        if let Some(i) = self.dev_list_state.selected() {
            let dev_list = Arc::clone(&self.dev_list);
            let read_list = dev_list.read().unwrap();

            let mut dev = read_list.get(i).unwrap().clone();

            drop(read_list);

            tokio::spawn(async move {

                /* Calculate and normalize new volume level */
                let new_level = dev.volume as i16 + step;

                let level : u16 = if 0 > new_level {
                    0
                } else {
                    new_level as u16
                };

                let level_str = level.to_string();

                info!("set_volume: level={}", level_str);

                let cmd = HeosCommand::new()
                    .group("player")
                    .cmd("set_volume")
                    .attr("level", &*level_str);

                let reply = dev.send_command(&cmd).await.unwrap();

                if let HeosReply::Volume(success, _) = reply {
                    info!("set_volume: success={}, level={}", success, level);

                    if success {
                        let mut write_list = dev_list.write().unwrap();

                        #[allow(unused_mut)]
                        let mut dev = write_list.get_mut(i).unwrap();

                        dev.volume = level;
                    }
                } else if let HeosReply::Error(success, command, message) = reply {
                    error!("set_state: success={}, command={:?}, message={:?}",
                        success, command, message);
                }
            });
        }
    }

    pub(crate) fn set_play_state(&mut self, state: PlayerState) {
        if let Some(i) = self.dev_list_state.selected() {
            let dev_list = Arc::clone(&self.dev_list);
            let read_list = dev_list.read().unwrap();

            let mut dev = read_list.get(i).unwrap().clone();

            drop(read_list);

            tokio::spawn(async move {
                let state_str = state.to_string().to_lowercase();

                info!("set_play_state: state={}", state_str);

                let cmd = HeosCommand::new()
                    .group("player")
                    .cmd("set_play_state")
                    .attr("state", &*state_str);

                let reply = dev.send_command(&cmd).await.unwrap();

                if let HeosReply::PlayState(success, _) = reply {
                    info!("set_play_state: success={}, state={}", success, state_str);
                } else if let HeosReply::Error(success, command, message) = reply {
                    error!("set_play_state: success={}, command={:?}, message={:?}",
                        success, command, message);
                }
            });
        }
    }

    pub(crate) fn toggle_mute(&self) {
        if let Some(i) = self.dev_list_state.selected() {
            let dev_list = Arc::clone(&self.dev_list);
            let read_list = dev_list.read().unwrap();

            let mut dev = read_list.get(i).unwrap().clone();

            drop(read_list);

            tokio::spawn(async move {
                info!("toggle_mute");

                let cmd = HeosCommand::new()
                    .group("player")
                    .cmd("toggle_mute");

                let reply = dev.send_command(&cmd).await.unwrap();

                if let HeosReply::Mute(success, _) = reply {
                    info!("toggle_mute: success={}", success);
                } else if let HeosReply::Error(success, command, message) = reply {
                    error!("toggle_mute: success={}, command={:?}, message={:?}",
                        success, command, message);
                }
            });
        }
    }
}
