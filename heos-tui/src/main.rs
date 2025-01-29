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
use arc_swap::ArcSwap;

mod app;
mod heos;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let arc_list = ArcSwap::from_pointee(Vec::<HeosDevice>::new());

    heos::discover_devices(arc_list).await;

    let terminal = ratatui::init();
    let result = App::new(arc_list).run(terminal).await;

    ratatui::restore();

    result
}
