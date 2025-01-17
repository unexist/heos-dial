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

        let result = dev.send_command(&cmd).await
            .expect("Failed to send command");

        println!("{:?}", result);

        assert!(matches!(result, HeosReply::Players { .. }));
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

        let result = dev.send_command(&cmd).await
            .expect("Failed to send command");

        println!("{:?}", result);

        assert!(matches!(result, HeosReply::PlayingMedia { .. }));
    }
}
