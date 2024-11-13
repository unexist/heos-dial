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
    fn should_generate_valid_heos_commands() {
        const COMMAND1: &'static str = "heos://player/get_players\r\n";
        const COMMAND2: &'static str = "heos://player/set_play_state?state=play\r\n";
        const COMMAND3: &'static str = "heos://player/set_play_state?state=play&pid=5\r\n";

        let cmd1 = HeosCommand::new()
            .group("player")
            .cmd("get_players")
            .build();

        assert_eq!(COMMAND1, cmd1);

        let cmd2 = HeosCommand::new()
            .group("player")
            .cmd("set_play_state")
            .attrs(vec![("state", "play")])
            .build();

        assert_eq!(COMMAND2, cmd2);

        let cmd3 = HeosCommand::new()
            .group("player")
            .cmd("set_play_state")
            .attr("state", "play")
            .attr("pid", "5")
            .build();

        assert_eq!(COMMAND3, cmd3);
    }
}
