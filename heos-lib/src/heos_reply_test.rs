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

    const RAW_GET_PLAYERS_REPLY: &'static str = r###"{"heos": {"command": "player/get_players",\
"result": "success", "message": ""},\
"payload": [\
{"name": "Living Room (AVR)", "pid": -474905601, "model": "Denon AVR-S660H", "version": "3.34.410", "ip": "10.0.8.37", "network": "wired", "lineout": 0, "serial": "DBNM052317669"},\
{"name": "Studio1", "pid": 844263156, "gid": -622728288, "model": "Denon Home 350", "version": "3.34.425", "ip": "10.0.8.24", "network": "wifi", "lineout": 0, "serial": "BME27220818140"}\
]}\r\n"###;

    const RAW_SET_PLAY_STATE_REPLY: &'static str = r###"{"heos": {\
"command": "player/get_play_state",\
"result": "success",\
"message": "pid='player_id'&state='play_state'"\
}\r\n"###;

    const RAW_GET_PLAY_STATE_REPLY: &'static str = r###"{"heos": {\
"command": "player/set_play_state", \
"result": "success", \
"message": "message": "pid='player_id'&state='play_state'" \
}\r\n"###;

    const RAW_SET_VOLUME_REPLY: &'static str = r###"{"heos": {\
"command": "player/set_volume",\
"result": "success",\
"message": "pid='player_id'&level='vol_level'"
}\r\n"###;

    const RAW_GET_VOLUME_REPLY: &'static str = r###"{"heos": {\
"command": "player/get_volume",\
"result": "success",\
"message": "pid='player_id'&level='vol_level'"
}\r\n"###;

    const RAW_PLAY_NEXT_REPLY: &'static str = r###"{"heos": {\
"command": " player/play_next", \
"result": "success", \
"message": "pid=player_id" \
}\r\n"###;

    const RAW_PLAY_PREVIOUS_REPLY: &'static str = r###"{"heos": {\
"command": " player/play_next", \
"result": "success", \
"message": "pid=player_id" \
}\r\n"###;

    #[test]
    fn should_parse_get_players_reply() {
        let reply = HeosReply::parse(RAW_GET_PLAYERS_REPLY).expect("Failed to parse");

        assert!(matches!(reply, HeosReply::Players { .. }));
    }

    #[test]
    fn should_parse_set_play_state_reply() {
        let reply = HeosReply::parse(RAW_SET_PLAY_STATE_REPLY).expect("Failed to parse");

        assert!(matches!(reply, HeosReply::PlayState { .. }));
    }

    #[test]
    fn should_parse_get_play_state_reply() {
        let reply = HeosReply::parse(RAW_SET_PLAY_STATE_REPLY).expect("Failed to parse");

        assert!(matches!(reply, HeosReply::PlayState { .. }));
    }

    #[test]
    fn should_parse_set_volume_reply() {
        let reply = HeosReply::parse(RAW_SET_VOLUME_REPLY).expect("Failed to parse");

        assert!(matches!(reply, HeosReply::Volume { .. }));
    }
}
