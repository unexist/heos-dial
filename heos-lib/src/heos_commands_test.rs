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

    const URL: &'static str = "http://10.0.8.87:60006/upnp/desc/aios_device/aios_device.xml";

    #[test]
    fn should_generate_valid_heos_command() {
        const COMMAND: &'static str = "heos://player/get_players\r\n";

        let heos = Heos::new();

        assert_eq!(heos.command_from("player", "get_volume", vec![()]), COMMAND);
    }

    #[test]
    fn should_generate_valid_heos_device_command() {
        const COMMAND: &'static str = "heos://player/set_play_state?pid=5&state=play\r\n";

        let dev = HeosDevice::new(URL, "5");

        assert_eq!(dev.command_from("player", "set_play_state",
                                    vec![("state", "play")]), COMMAND);
    }
}
