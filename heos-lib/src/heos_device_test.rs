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
    use crate::HeosDevice;

    #[test]
    fn should_parse_url() {
        const URL: &'static str = "http://10.0.8.87:60006/upnp/desc/aios_device/aios_device.xml";

        match HeosDevice::new(URL) {
            Ok(_) => assert!(true),
            Err(e) => panic!("{:?}", e),
        }
    }
}
