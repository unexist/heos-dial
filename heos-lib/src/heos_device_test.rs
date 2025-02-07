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
    use crate::heos_command::{HeosCommand, HeosCommandHandler};
    use crate::heos_reply::HeosReply;
    use crate::HeosDevice;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_create_valid_client() {
        let dev = HeosDevice::new(
            env!("TEST_DEVICE_NAME"), env!("TEST_DEVICE_IP"), "");

        assert!(dev.is_ok());
    }

    #[tokio::test]
    async fn should_update_own_info() {
        let mut dev = HeosDevice::new(
            "", env!("TEST_DEVICE_IP"), "")
            .expect("Failed to create client");

        assert_eq!(dev.name, "");
        assert_eq!(dev.player_id, "");

        dev.update().await
            .expect("Failed to update client");

        assert_eq!(dev.name, env!("TEST_DEVICE_NAME"));
    }


    #[tokio::test]
    async fn should_connect_and_get_players() {
        let mut dev = HeosDevice::new(
            env!("TEST_DEVICE_NAME"), env!("TEST_DEVICE_IP"), "")
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
        let mut dev = HeosDevice::new(
            env!("TEST_DEVICE_NAME"), env!("TEST_DEVICE_IP"), "")
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
            assert!(payload.get("artist").is_some());
            assert!(payload.get("song").is_some());
        }
    }


    #[tokio::test]
    async fn should_connect_and_get_volume() {
        let mut dev = HeosDevice::new(
            env!("TEST_DEVICE_NAME"), env!("TEST_DEVICE_IP"), "")
            .expect("Failed to create client");

        dev.connect().await
            .expect("Failed to connect to client");

        let cmd = HeosCommand::new()
            .group("player")
            .cmd("get_volume");

        let reply = dev.send_command(&cmd).await
            .expect("Failed to send command");

        println!("{:?}", reply);

        assert!(matches!(reply, HeosReply::Volume { .. }));

        if let HeosReply::Volume(_success, payload) = reply {
            assert!(payload.get("level").is_some());
        }
    }
}
