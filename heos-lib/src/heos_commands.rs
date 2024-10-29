///
/// @package heos-dial
///
/// @file HEOS lib
/// @copyright 2024-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use crate::heos_attributes::HeosAttributes;
use crate::{Heos, HeosDevice};

pub(crate) trait HeosCommands {
    fn command_from(command_group: &str, command_string: &str,
                    attributes: Vec<(&str, &str)>) -> String;
}

fn create_command(command_group: &str, command_string: &str, attributes_string: &str) -> String {
    format!("{}{}/{}{}{}", crate::constants::PREFIX, command_group, command_string,
            attributes_string, crate::constants::POSTFIX)
}

impl HeosCommands for HeosDevice {
    fn command_from(command_group: &str, command_string: &str,
                    attributes: Vec<(&str, &str)>) -> String
    {
        attributes.append(vec!["", Self.player_id.clone_into()]);

        let attributes_string = attributes.to_heos_attrs()
            .expect("Parsing attributes failed").as_str();

        create_command(command_group, command_string, attributes_string)
    }
}

impl HeosCommands for Heos {
    fn command_from(command_group: &str, command_string: &str,
                    attributes: Vec<(&str, &str)>) -> String
    {
        let attributes_string = attributes.to_heos_attrs()
            .expect("Parsing attributes failed").as_str();

        create_command(command_group, command_string, attributes_string)
    }
}
