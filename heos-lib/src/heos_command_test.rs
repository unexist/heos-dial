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
    use crate::heos_command::HeosCommand;

    #[test]
    fn should_generate_valid_heos_command() {
        const COMMAND1: &'static str = "heos://player/get_players\r\n";
        const COMMAND2: &'static str = "heos://player/set_play_state?state=play&pid=5\r\n";

        let cmd1 = HeosCommand::new()
            .group("player")
            .cmd("get_players");

        assert_eq!(COMMAND1, cmd1.to_string());

        let cmd2 = HeosCommand::new()
            .group("player")
            .cmd("set_play_state")
            .attrs(vec![("state", "play")]);

        assert_eq!(COMMAND2, cmd2.to_string());
    }
}
