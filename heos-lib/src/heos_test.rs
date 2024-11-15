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
    use crate::constants::TEST_LOCATION;

    #[test]
    fn should_parse_discovery_response() {
        const RAW_REPLY: &'static str = formatcp!("HTTP/1.1 200 OK\r\n\
CACHE-CONTROL: max-age=180\r\n\
EXT:\r\n\
LOCATION: {location}\r\n\
VERSIONS.UPNP.HEOS.COM: 10,668205267,801394619,363364703,1840750642,105553199,-316033077,1711326982,-838802320,-170053632,363364703\r\n\
NETWORKID.UPNP.HEOS.COM: d424dda645d7\r\n\
BOOTID.UPNP.ORG: 1947595085\r\n\
IPCACHE.URL.UPNP.HEOS.COM: /ajax/upnp/get_device_info\r\n\
SERVER: LINUX UPnP/1.0 Denon-Heos/316763a47eba7769d9be106fb4f3617c5393a2b7\r\n\
ST: urn:schemas-denon-com:device:ACT-Denon:1\r\n\
USN: uuid:60f346a0-9018-49e7-b77e-4a14ad25b96f::urn:schemas-denon-com:device:ACT-Denon:1\r\n\r\n", location = TEST_LOCATION);

        assert!(Heos::parse_discovery_response(RAW_REPLY)
            .is_ok_and(|dev| dev.base_url.to_string() == TEST_LOCATION));
    }

    #[ignore]
    #[tokio::test]
    async fn should_discover_anything() {
        let heos = Heos::new();

        let devices = heos.discover().await
            .expect("To discover devices");
        pin_mut!(devices);

        if let Some(device) = devices.next().await {
            assert!(true);
        } else {
            assert!(false, "Failed to discover devices");
        }
    }
}
