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
    use const_format::formatcp;
    use futures_util::{pin_mut, StreamExt};
    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_discovery_response() {
        let reply = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/test_assets/discovery_response.txt"));
        let location = formatcp!("http://{ip}:60006/upnp/desc/aios_device/aios_device.xml",
                ip = env!("TEST_DEVICE_IP"));

        assert!(Heos::parse_discovery_response(reply)
            .is_ok_and(|parsed_location| parsed_location == location));
    }

    #[test]
    fn should_parse_location() {
        let input = formatcp!("http://{ip}:60006/upnp/desc/aios_device/aios_device.xml",
                ip = env!("TEST_DEVICE_IP"));
        let location = Heos::parse_location(input)
            .expect("Failed to parse location");

        assert_eq!(location, *env!("TEST_DEVICE_IP"));
    }

    #[test]
    fn should_parse_device_description() {
        assert!(true)
    }

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
