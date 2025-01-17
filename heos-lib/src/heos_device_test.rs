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
mod heos_device_test {
    use crate::constants::{TEST_DEVICE_IP, TEST_DEVICE_NAME, TEST_DEVICE_PLAYER_ID};
    use crate::heos_command::{HeosCommand, HeosCommandHandler};
    use crate::heos_reply::HeosReply;
    use crate::HeosDevice;

    #[test]
    fn should_create_valid_client() {
        let dev = HeosDevice::new(TEST_DEVICE_NAME,
                                  TEST_DEVICE_IP, TEST_DEVICE_PLAYER_ID);

        assert!(dev.is_ok());
    }

    #[tokio::test]
    async fn should_connect_and_get_players() {
        let mut dev = HeosDevice::new(TEST_DEVICE_NAME,
                                      TEST_DEVICE_IP, TEST_DEVICE_PLAYER_ID)
            .expect("Failed to create client");

        dev.connect().await
            .expect("Failed to connect to client");

        let cmd = HeosCommand::new()
            .group("player")
            .cmd("get_players");

        let reply = dev.send_command(&cmd).await
            .expect("Failed to send command");

        println!("{:?}", reply);

        assert!(matches!(reply, HeosReply::Players { .. }));
    }

    #[tokio::test]
    async fn should_connect_and_get_playing_media() {
        let mut dev = HeosDevice::new(TEST_DEVICE_NAME,
                                      TEST_DEVICE_IP, TEST_DEVICE_PLAYER_ID)
            .expect("Failed to create client");

        dev.connect().await
            .expect("Failed to connect to client");

        let cmd = HeosCommand::new()
            .group("player")
            .cmd("get_now_playing_media");

        let reply = dev.send_command(&cmd).await
            .expect("Failed to send command");

        println!("{:?}", reply);

        assert!(matches!(reply, HeosReply::PlayingMedia { .. }));

        if let HeosReply::PlayingMedia(_success, payload) = reply {
            assert_eq!(payload.get("artist").expect("Failed to parse artist"), "Lekkerfaces");
            assert_eq!(payload.get("song").expect("Failed to parse song"), "Break Down");
        }
    }
}
