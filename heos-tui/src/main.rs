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
use crate::handlers::handle_key_events;
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

mod app;
mod ui;
mod handlers;
mod events;
mod tui;

#[tokio::main]
async fn main() -> AppResult<()> {
    /* Initialize the terminal user interface */
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    /* Create swap list */
    let dev_orig_list = Arc::new(RwLock::new(Vec::<HeosDevice>::new()));
    let group_orig_list = Arc::new(RwLock::new(Vec::<HeosGroup>::new()));

    let mut app = App::new(Arc::clone(&dev_orig_list), Arc::clone(&group_orig_list));

    tokio::spawn(start_discovery(Arc::clone(&dev_orig_list), Arc::clone(&group_orig_list)));

    /* Kick off main loop */
    while app.running {
        tui.draw(&mut app)?;

        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Redraw => tui.draw(&mut app)?,
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;

    Ok(())
}

async fn start_discovery(dev_list: Arc<RwLock<Vec<HeosDevice>>>, group_list: Arc<RwLock<Vec<HeosGroup>>>) {
    let devices = Heos::discover().await
        .expect("To discover devices");
    pin_mut!(devices);

    info!("discovery: Start");

    let mut cmd = HeosCommand::new()
        .group("player")
        .cmd("get_players");

    while let Some(mut dev) = devices.next().await {
        info!("start_discovery: Requesting known devices from {}", dev);

        /* Ask first device for other known devices */
        let mut reply = dev.send_command(&cmd).await
            .expect("To send command");

        if let HeosReply::Players(success, mut devices) = reply {
            if success {
                debug!("start_discovery: Found ndevices={}", devices.len());

                for dev in &mut devices {
                    dev.update_volume().await.expect("To update volume");

                    info!("start_discovery: Updated volume for {}", dev);
                }

                /* Replace vec */
                let mut write_list = dev_list.write().unwrap();

                let _ = std::mem::replace(&mut *write_list, devices);

                break;
            }
        } else if let HeosReply::Error(success, command, message) = reply {
            error!("start_discovery: success={}, command={:?}, message={:?}",
                        success, command, message);
        }

        /* Ask first device for known groups */
        cmd = HeosCommand::new()
            .group("player")
            .cmd("get_groups");

        reply = dev.send_command(&cmd).await.expect("To send command");

        if let HeosReply::Groups(success, groups) = reply {
            if success {
                debug!("start_discovery: Found ngroups={}", groups.len());

                for group in &groups {
                    info!("start_discovery: Found group {}", group);
                }

                /* Replace vec */
                let mut write_list = group_list.write().unwrap();

                let _ = std::mem::replace(&mut *write_list, groups);
            }
        } else if let HeosReply::Error(success, command, message) = reply {
            error!("start_discovery: success={}, command={:?}, message={:?}",
                        success, command, message);
        }
    }
}
