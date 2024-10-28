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
mod heos_attributes_test {
    use crate::HeosDevice;
    use crate::heos_attributes::HeosAttributes;

    #[test]
    fn should_generate_heos_attribute_string() {
        assert_eq!(vec![].to_heos_attrs().expect("To attributes failed"), "");
        assert_eq!(vec![("pid", "5")].to_heos_attrs().expect("To attributes failed"), "?pid=5");
        assert_eq!(vec![("pid", "5"), ("v", "1")].to_heos_attrs().expect("To attributes failed"), "?pid=5&v=1");
    }

    #[test]
    fn should_generate_valid_heos_command() {
        const COMMAND: &'static str = "heos://player/get_volume?pid=5\r\n";

        assert_eq!(HeosDevice::command_from("player",
                                            "get_volume", vec![("pid", "5")]), COMMAND);
    }
}
