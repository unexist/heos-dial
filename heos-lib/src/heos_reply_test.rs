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
mod heos_reply_test {
    use crate::heos_reply::HeosReply;

    const RAW_PLAYERS_REPLY: &'static str = r###"{"heos": {"command": "player/get_players",\
"result": "success", "message": ""},\
"payload": [\
{"name": "Living Room (AVR)", "pid": -474905601, "model": "Denon AVR-S660H", "version": "3.34.410", "ip": "10.0.8.37", "network": "wired", "lineout": 0, "serial": "DBNM052317669"},\
{"name": "Studio1", "pid": 844263156, "gid": -622728288, "model": "Denon Home 350", "version": "3.34.425", "ip": "10.0.8.24", "network": "wifi", "lineout": 0, "serial": "BME27220818140"}\
]}\r\n"###;

    const RAW_PLAY_REPLY: &'static str = r###"heos": {\
"command": "player/get_play_state",\
"result": "success",\
"message": "pid='player_id'&state='play_state'"\
}\r\n"###;

    const RAW_SET_VOL_REPLY: &'static str = r###""heos": {\
 "command": "player/set_volume ",\
 "result": "success",\
 "message": "pid='player_id'&level='vol_level'"
 }\r\n"###;

    #[test]
    fn should_parse_players_reply() {
        let reply = HeosReply::parse(RAW_PLAYERS_REPLY).expect("Failed to parse");

        assert!(matches!(reply, HeosReply::Players { .. }));
    }

    #[test]
    fn should_parse_play_reply() {
        let reply = HeosReply::parse(RAW_PLAY_REPLY).expect("Failed to parse");

        assert!(matches!(reply, HeosReply::PlayState { .. }));
    }

    #[test]
    fn should_parse_set_vol_reply() {
        let reply = HeosReply::parse(RAW_SET_VOL_REPLY).expect("Failed to parse");

        assert!(matches!(reply, HeosReply::SetVol { .. }));
    }
}
