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

use crate::app::AppResult;
use crate::events::{Event, EventHandler};
use crate::tui::Tui;
use app::App;
use futures::pin_mut;
use futures_util::StreamExt;
use heos_lib::heos_command::{HeosCommand, HeosCommandHandler};
use heos_lib::{Heos, HeosDevice, HeosGroup, HeosReply};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::sync::{Arc, RwLock};
use log::{debug, error, info};
use tokio::sync::mpsc::UnboundedSender;

mod app;
mod ui;
mod events;
mod tui;

#[tokio::main]
async fn main() -> AppResult<()> {
    /* Initialize the terminal user interface */
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;

    let mut events = EventHandler::new();
    let mut tui = Tui::new(terminal);

    tui.init()?;

    /* Create swap list */
    let dev_orig_list = Arc::new(RwLock::new(Vec::<HeosDevice>::new()));
    let group_orig_list = Arc::new(RwLock::new(Vec::<HeosGroup>::new()));

    let mut app = App::new(Arc::clone(&dev_orig_list), Arc::clone(&group_orig_list), events.sender.clone());

    tokio::spawn(start_discovery(Arc::clone(&dev_orig_list), Arc::clone(&group_orig_list), events.sender.clone()));

    /* Kick off main loop */
    while app.is_running {
        tui.draw(&mut app)?;

        match events.next().await? {
            Event::Redraw => tui.draw(&mut app)?,
            Event::Key(key_event) => app.handle_key_events(key_event)?,
            _ => {}
        }
    }

    tui.exit()?;

    Ok(())
}

async fn start_discovery(dev_list: Arc<RwLock<Vec<HeosDevice>>>, group_list: Arc<RwLock<Vec<HeosGroup>>>, cloned_sender: UnboundedSender<Event>) {
    let devices = Heos::discover().await
        .expect("To discover devices");
    pin_mut!(devices);

    info!("discovery: Start");

    let mut cmd = HeosCommand::new()
        .group("player")
        .cmd("get_players");

    cloned_sender.send(Event::Redraw).unwrap();

    while let Some(mut dev) = devices.next().await {
        info!("discovery: Requesting known devices from {}", dev);

        /* Ask first device for other known devices */
        let mut reply = dev.send_command(&cmd).await
            .expect("To send command");

        if let HeosReply::Players(success, mut devices) = reply {
            if success {
                debug!("discovery: Found ndevices={}", devices.len());

                for dev in &mut devices {
                    let res = dev.update_volume().await;

                    info!("discovery: Updated volume for {} ({:?})", dev, res);
                }

                for dev in &mut devices {
                    let res = dev.update_media().await;

                    info!("discovery: Updated media for {} ({:?})", dev, res);
                }

                /* Replace vec */
                let mut write_list = dev_list.write().unwrap();

                let _ = std::mem::replace(&mut *write_list, devices);

                cloned_sender.send(Event::Redraw).unwrap();
            }
        } else if let HeosReply::Error(success, command, message) = reply {
            error!("discovery: success={}, command={:?}, message={:?}",
                        success, command, message);
        }

        cloned_sender.send(Event::Redraw).unwrap();

        /* Ask first device for known groups */
        cmd = HeosCommand::new()
            .group("player")
            .cmd("get_groups");

        reply = dev.send_command(&cmd).await.expect("To send command");

        if let HeosReply::Groups(success, mut groups) = reply {
            if success {
                debug!("start_discovery: Found ngroups={}", groups.len());

                /* Find base url for leader if any */
                for group in &mut groups {
                    info!("start_discovery: Found group {}", group);

                    if let Some(leader) = group.leader.as_mut() {
                        for dev in dev_list.read().unwrap().iter() {
                            if dev == leader {
                                leader.base_url = dev.base_url.clone();
                            }
                        }
                    }
                }

                for group in &mut groups {
                    let res = group.update_volume().await;

                    info!("discovery: Updated volume for {} ({:?})", group, res);
                }

                /* Replace vec */
                let mut write_list = group_list.write().unwrap();

                let _ = std::mem::replace(&mut *write_list, groups);

                cloned_sender.send(Event::Redraw).unwrap();
            }
        } else if let HeosReply::Error(success, command, message) = reply {
            error!("start_discovery: success={}, command={:?}, message={:?}",
                        success, command, message);
        }

        break;
    }
}
