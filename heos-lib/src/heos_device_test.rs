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
    use crate::constants::LOCATION;
    use crate::HeosDevice;

    fn should_create_valid_client() {
        let dev = HeosDevice::new(LOCATION, "1");

        assert!(dev.is_ok());
    }
}
