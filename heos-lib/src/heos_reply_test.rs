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
    use std::collections::HashMap;
    use crate::heos_reply::HeosReply;

    const JSON_GET_PLAYERS_REPLY: &'static str = r###"{"heos": {"command": "player/get_players",\
"result": "success", "message": ""},\
"payload": [\
{"name": "Living Room (AVR)", "pid": -474905601, "model": "Denon AVR-S660H", "version": "3.34.410", "ip": "10.0.8.37", "network": "wired", "lineout": 0, "serial": "DBNM052317669"},\
{"name": "Studio1", "pid": 844263156, "gid": -622728288, "model": "Denon Home 350", "version": "3.34.425", "ip": "10.0.8.24", "network": "wifi", "lineout": 0, "serial": "BME27220818140"}\
]}\r\n"###;

    const JSON_SET_PLAY_STATE_REPLY: &'static str = r###"{"heos": {\
"command": "player/get_play_state",\
"result": "success",\
"message": "pid='player_id'&state='play_state'"\
}\r\n"###;

    const JSON_GET_PLAY_STATE_REPLY: &'static str = r###"{"heos": {\
"command": "player/set_play_state", \
"result": "success", \
"message": "pid='player_id'&state='play_state'" \
}\r\n"###;

    const JSON_SET_VOLUME_REPLY: &'static str = r###"{"heos": {\
"command": "player/set_volume",\
"result": "success",\
"message": "pid='player_id'&level='vol_level'"
}\r\n"###;

    const JSON_GET_VOLUME_REPLY: &'static str = r###"{"heos": {\
"command": "player/get_volume",\
"result": "success",\
"message": "pid='player_id'&level='vol_level'"
}\r\n"###;

    const JSON_PLAY_NEXT_REPLY: &'static str = r###"{"heos": {\
"command": "player/play_next", \
"result": "success", \
"message": "pid=player_id" \
}\r\n"###;

    const JSON_PLAY_PREVIOUS_REPLY: &'static str = r###"{"heos": {\
"command": "player/play_previous", \
"result": "success", \
"message": "pid=player_id" \
}\r\n"###;

    const JSON_MESSAGE: &'static str = r###"{"message": "pid='player_id'&repeat=on_all_or_on_one_or_off&shuffle=on_or_off"}"###;

    #[test]
    fn should_parse_get_players_reply() {
        let reply = HeosReply::parse(JSON_GET_PLAYERS_REPLY)
            .expect("Failed to parse");

        assert!(matches!(reply, HeosReply::Players { .. }));
    }

    #[test]
    fn should_parse_set_play_state_reply() {
        let reply = HeosReply::parse(JSON_SET_PLAY_STATE_REPLY)
            .expect("Failed to parse");

        assert!(matches!(reply, HeosReply::PlayState { .. }));
    }

    #[test]
    fn should_parse_get_play_state_reply() {
        let reply = HeosReply::parse(JSON_GET_PLAY_STATE_REPLY)
            .expect("Failed to parse");

        assert!(matches!(reply, HeosReply::PlayState { .. }));
    }

    #[test]
    fn should_parse_set_volume_reply() {
        let reply = HeosReply::parse(JSON_SET_VOLUME_REPLY)
            .expect("Failed to parse");

        assert!(matches!(reply, HeosReply::Volume { .. }));
    }

    #[test]
    fn should_parse_get_volume_reply() {
        let reply = HeosReply::parse(JSON_GET_VOLUME_REPLY)
            .expect("Failed to parse");

        assert!(matches!(reply, HeosReply::Volume { .. }));
    }

    #[test]
    fn should_parse_play_next_reply() {
        let reply = HeosReply::parse(JSON_PLAY_NEXT_REPLY)
            .expect("Failed to parse");

        assert!(matches!(reply, HeosReply::PlayAction { .. }));
    }

    #[test]
    fn should_parse_play_previous_reply() {
        let reply = HeosReply::parse(JSON_PLAY_PREVIOUS_REPLY)
            .expect("Failed to parse");

        assert!(matches!(reply, HeosReply::PlayAction { .. }));
    }

    #[test]
    fn should_parse_message() {
        let message: HashMap<_, _> = HeosReply::parse_message(
            &gjson::parse(JSON_MESSAGE), "message");

        assert_eq!(message.get("pid").expect("Parsing pid failed"), "'player_id'");
        assert_eq!(message.get("repeat").expect("Parsing repeat_on failed"), "on_all_or_on_one_or_off");
        assert_eq!(message.get("shuffle").expect("Parsing shuffle failed"), "on_or_off");
    }

    #[test]
    fn should_parse_players_payload() {
        let payload = gjson::parse(JSON_GET_PLAYERS_REPLY);
        let devices = HeosReply::parse_players_payload(&payload, "heos.payload");

        assert_eq!(devices.len(), 2);
        assert_eq!(devices[0].player_id, "-474905601");
        assert_eq!(devices[0].base_url.host_str(), Some("10.0.8.37"));
    }
}
