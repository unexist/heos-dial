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
    use crate::constants::TEST_LOCATION;
    use crate::heos_command::HeosCommand;
    use crate::HeosDevice;

    fn should_create_valid_client() {
        let dev = HeosDevice::new(TEST_LOCATION, "1");

        assert!(dev.is_ok());
    }

    #[ignore]
    #[tokio::test]
    async fn should_connect_and_get_result() {
        let dev = HeosDevice::new(TEST_LOCATION, "1")
            .expect("Failed to create client");

        let cmd = dev.command_from("player", "get_players", vec![])
            .expect("Failed to create command");

        let result = dev.send(cmd.as_ref()).await;

        assert!(result.is_ok());
    }
}
