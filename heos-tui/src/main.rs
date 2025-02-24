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
use arc_swap::ArcSwap;
use futures::pin_mut;
use futures_util::StreamExt;
use heos_lib::heos_command::{HeosCommand, HeosCommandHandler};
use heos_lib::{Heos, HeosDevice, HeosReply};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::sync::Arc;

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

    let arc_list = Arc::new(ArcSwap::from_pointee(Vec::<HeosDevice>::new()));
    let dev_list = Arc::clone(&arc_list);

    let mut app = App::new(Arc::clone(&arc_list));

    /* Start discovery */
    tokio::spawn(async move {
        let devices = Heos::discover().await
            .expect("To discover devices");
        pin_mut!(devices);

        let cmd = HeosCommand::new()
            .group("player")
            .cmd("get_players");

        while let Some(mut dev) = devices.next().await {

            /* Ask first device for other known devices */
            let reply = dev.send_command(&cmd).await
                .expect("To send command");

            if let HeosReply::Players(success, devices) = reply {
                if success {
                    dev_list.swap(Arc::from(devices));

                    break;
                }
            }
        }
    });

    /* Kick off main loop */
    while app.running {
        tui.draw(&mut app)?;

        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;

    Ok(())
}
