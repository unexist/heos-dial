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
use heos_lib::HeosDevice;
use std::sync::Arc;
use tokio::sync::Mutex;

mod app;
mod heos;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let dev_list = Arc::new(Mutex::new(Vec::<HeosDevice>::new()));

    heos::discover_devices(Arc::clone(&dev_list)).await;

    let terminal = ratatui::init();
    let result = App::new(Arc::clone(&dev_list)).run(terminal).await;

    ratatui::restore();

    result
}
