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
    use rstest::{fixture, rstest};

    #[fixture]
    fn heos_device() -> HeosDevice {
        HeosDevice::new(env!("TEST_DEVICE_NAME"),
                        env!("TEST_DEVICE_IP"), "")
            .expect("Failed to create device")
    }

    #[rstest]
    fn should_clone_and_compare_with_itself(heos_device: HeosDevice) {
        let cloned = heos_device.clone();

        assert_eq!(heos_device, cloned);
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn should_update_own_info(mut heos_device: HeosDevice) {
        heos_device.update().await
            .expect("Failed to update client");

        assert_eq!(heos_device.name, env!("TEST_DEVICE_NAME"));
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn should_connect_and_get_players(mut heos_device: HeosDevice) {
        heos_device.connect().await
            .expect("Failed to connect to client");

        let cmd = HeosCommand::new()
            .group("player")
            .cmd("get_players");

        let reply = heos_device.send_command(&cmd).await
            .expect("Failed to send command");

        println!("{:?}", reply);

        assert!(matches!(reply, HeosReply::Players { .. }));
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn should_connect_and_get_playing_media(mut heos_device: HeosDevice) {
        heos_device.connect().await
            .expect("Failed to connect to client");

        let cmd = HeosCommand::new()
            .group("player")
            .cmd("get_now_playing_media");

        let reply = heos_device.send_command(&cmd).await
            .expect("Failed to send command");

        println!("{:?}", reply);

        assert!(matches!(reply, HeosReply::PlayingMedia { .. }));

        if let HeosReply::PlayingMedia(_success, payload) = reply {
            assert!(payload.get("artist").is_some());
            assert!(payload.get("song").is_some());
        }
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn should_connect_and_get_volume(mut heos_device: HeosDevice) {
        heos_device.connect().await
            .expect("Failed to connect to client");

        let cmd = HeosCommand::new()
            .group("player")
            .cmd("get_volume");

        let reply = heos_device.send_command(&cmd).await
            .expect("Failed to send command");

        println!("{:?}", reply);

        assert!(matches!(reply, HeosReply::Volume { .. }));

        if let HeosReply::Volume(_success, payload) = reply {
            assert!(payload.get("level").is_some());
        }
    }
}
