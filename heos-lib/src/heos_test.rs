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
mod heos_test {
    use crate::heos::Heos;
    use crate::test_asset;
    use const_format::formatcp;
    use futures_util::{pin_mut, StreamExt};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_discovery_response() {
        let location = formatcp!("http://{ip}:60006/upnp/desc/aios_device/aios_device.xml",
                ip = env!("TEST_DEVICE_IP"));

        let parsed  = Heos::parse_discovery_response(test_asset!("discovery_response.txt"))
            .expect("Failed to parse location");

        assert_eq!(parsed, location);
    }

    #[test]
    fn should_parse_location() {
        let input = formatcp!("http://{ip}:60006/upnp/desc/aios_device/aios_device.xml",
                ip = env!("TEST_DEVICE_IP"));
        let parsed = Heos::parse_location(input)
            .expect("Failed to parse location");

        assert_eq!(parsed, *env!("TEST_DEVICE_IP"));
    }

    #[ignore]
    #[tokio::test]
    async fn should_discover_at_least_one() {
        let devices = Heos::discover().await
            .expect("To discover devices");
        pin_mut!(devices);

        match devices.next().await {
            Some(_) => assert!(true),
            None => panic!("Failed to discover devices"),
        }
    }
}
