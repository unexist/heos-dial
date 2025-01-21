//
/// @package heos-dial
///
/// @file HEOS tui
/// @copyright 2024-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

pub use app::App;

pub mod app;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = App::default().run(terminal);

    ratatui::restore();

    result
}
