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

use app::App;
use heos_lib::{Heos, HeosDevice, HeosReply};
use arc_swap::ArcSwap;
use std::sync::Arc;
use futures::pin_mut;
use futures_util::StreamExt;
use heos_lib::heos_command::{HeosCommand, HeosCommandHandler};

mod app;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let arc_list = Arc::new(ArcSwap::from_pointee(Vec::<HeosDevice>::new()));
    let dev_list = Arc::clone(&arc_list);

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

    let terminal = ratatui::init();
    let result = App::new(Arc::clone(&arc_list)).run(terminal);

    ratatui::restore();

    result
}
