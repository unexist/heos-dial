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
use heos_lib::{Heos, HeosDevice};
use arc_swap::ArcSwap;
use std::sync::Arc;
use futures::pin_mut;
use futures_util::StreamExt;

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

        while let Some(dev) = devices.next().await {
            let mut swap_list = dev_list.load().to_vec();

            swap_list.push(dev);

            dev_list.swap(Arc::from(swap_list));
        }
    });

    let terminal = ratatui::init();
    let result = App::new(Arc::clone(&arc_list)).run(terminal);

    ratatui::restore();

    result
}
