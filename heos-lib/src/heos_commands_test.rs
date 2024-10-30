///
/// @package heos-dial
///
/// @file HEOS protocol helper tests
/// @copyright 2024-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

#[cfg(test)]
mod heos_commands_test {
    use crate::{Heos, HeosDevice};
    use crate::constants::LOCATION;
    use crate::heos_commands::HeosCommands;

    #[test]
    fn should_generate_valid_heos_command() {
        const COMMAND: &'static str = "heos://player/get_players\r\n";

        let heos = Heos::new();

        assert!(heos.command_from("player", "get_players",
                                  vec![])
            .is_ok_and(|cmd| COMMAND == cmd));
    }

    #[test]
    fn should_generate_valid_heos_device_command() {
        const COMMAND: &'static str = "heos://player/set_play_state?state=play&pid=5\r\n";

        let dev = HeosDevice::new(LOCATION, "5")
            .expect("Location is not a valid url");

        assert!(dev.command_from("player", "set_play_state",
                                 vec![("state", "play")])
            .is_ok_and(|cmd| COMMAND == cmd));
    }
}
