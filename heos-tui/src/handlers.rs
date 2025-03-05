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

use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::app;

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        /* Navigation */
        KeyCode::Char('h') | KeyCode::Left => app.set_volume(-1),
        KeyCode::Char('j') | KeyCode::Down => app.select_next(),
        KeyCode::Char('k') | KeyCode::Up => app.select_previous(),
        KeyCode::Char('l') | KeyCode::Right => app.set_volume(1),

        KeyCode::Char('g') | KeyCode::Home => app.select_first(),
        KeyCode::Char('G') | KeyCode::End => app.select_last(),

        /* Player */
        KeyCode::Char('s') => app.set_state(app::PlayerState::Play),
        KeyCode::Char('p') => app.set_state(app::PlayerState::Stop),

        KeyCode::Esc => app.select_none(),
        KeyCode::Enter => app.toggle_status(),

        /* Exit keys */
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        },

        _ => {}
    }
    Ok(())
}




