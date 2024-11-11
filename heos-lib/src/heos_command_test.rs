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
    use crate::constants::TEST_LOCATION;
    use crate::heos_command::{HeosCommand, HeosCommandHandler};

    #[test]
    fn should_generate_valid_heos_command() {
        const COMMAND: &'static str = "heos://player/get_players\r\n";

        let heos = Heos::new();
        let cmd = HeosCommand::from(heos, "player", "get_players", vec![]);

        assert!(cmd.into().is_ok_and(|cmd| COMMAND == cmd));
    }

    #[test]
    fn should_generate_valid_heos_device_command() {
        const COMMAND: &'static str = "heos://player/set_play_state?state=play&pid=5\r\n";

        let dev = HeosDevice::new(TEST_LOCATION, "5")
            .expect("Location is not a valid url");
        let cmd = HeosCommand::from(dev, "player", "set_play_state",
                                    vec![("state", "play")]);

        assert!(cmd.into().is_ok_and(|cmd| COMMAND == cmd));
    }
}
