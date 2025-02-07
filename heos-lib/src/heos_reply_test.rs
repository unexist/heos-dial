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
    use pretty_assertions::assert_eq;

    macro_rules! test_asset {
        ($s:expr) => {
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/test_assets/", $s))
        }
    }

    #[test]
    fn should_parse_get_players_reply() {
        let input = test_asset!("get_players.json");
        let reply = HeosReply::parse(input).expect("Failed to parse");

        if let HeosReply::Players(success, devices) = reply {
            assert!(success);
            assert_eq!(devices.len(), 2);
        } else {
            panic!("Wrong reply type");
        }
    }

    #[test]
    fn should_parse_set_play_state_reply() {
        let input = test_asset!("set_play_state.json");
        let reply = HeosReply::parse(input)
            .expect("Failed to parse");

        assert!(matches!(reply, HeosReply::PlayState { .. }));
    }

    #[test]
    fn should_parse_get_play_state_reply() {
        let input = test_asset!("get_play_state.json");
        let reply = HeosReply::parse(input)
            .expect("Failed to parse");

        assert!(matches!(reply, HeosReply::PlayState { .. }));
    }

    #[test]
    fn should_parse_play_next_reply() {
        let input = test_asset!("play_next.json");
        let reply = HeosReply::parse(input)
            .expect("Failed to parse");

        assert!(matches!(reply, HeosReply::PlayAction { .. }));
    }

    #[test]
    fn should_parse_play_previous_reply() {
        let input = test_asset!("play_previous.json");
        let reply = HeosReply::parse(input)
            .expect("Failed to parse");

        assert!(matches!(reply, HeosReply::PlayAction { .. }));
    }

    #[test]
    fn should_parse_get_now_playing_media_reply() {
        let input = test_asset!("get_now_playing_media.json");
        let reply = HeosReply::parse(input)
            .expect("Failed to parse");

        assert!(matches!(reply, HeosReply::PlayingMedia { .. }));
    }

    #[test]
    fn should_parse_set_volume_reply() {
        let input = test_asset!("set_volume.json");
        let reply = HeosReply::parse(input)
            .expect("Failed to parse");

        assert!(matches!(reply, HeosReply::Volume { .. }));
    }

    #[test]
    fn should_parse_get_volume_reply() {
        let input = test_asset!("get_volume.json");
        let reply = HeosReply::parse(input)
            .expect("Failed to parse");

        assert!(matches!(reply, HeosReply::Volume { .. }));
    }

    #[test]
    fn should_parse_message() {
        let input = test_asset!("message.json");
        let json = gjson::parse(input);
        let attrs: HashMap<_, _> = HeosReply::parse_message(&json, "message");

        assert_eq!(attrs.get("pid").expect("Parsing pid failed"), "'player_id'");
        assert_eq!(attrs.get("repeat").expect("Parsing repeat_on failed"),
                   "on_all_or_on_one_or_off");
        assert_eq!(attrs.get("shuffle").expect("Parsing shuffle failed"), "on_or_off");
    }

    #[test]
    fn should_parse_generic_payload() {
        let input = test_asset!("get_now_playing_media.json");
        let json = gjson::parse(input);
        let payload: HashMap<_, _> = HeosReply::parse_generic_payload(&json, "payload");

        assert_eq!(payload.get("artist").expect("Parsing artist failed"), "'artist name'");
        assert_eq!(payload.get("album").expect("Parsing album failed"), "'album name'");
    }

    #[test]
    fn should_parse_players_payload() {
        let input = test_asset!("get_players.json");
        let json = gjson::parse(input);
        let devices = HeosReply::parse_players_payload(&json, "payload");

        assert_eq!(devices.len(), 2);
        assert_eq!(devices[0].base_url, env!("TEST_DEVICE_IP"));
    }
}
