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
use anyhow::Result;

pub(crate) trait HeosCommands {
    fn command_from(&self, command_group: &str, command_string: &str,
                    attributes: Vec<(&str, &str)>) -> Result<String>;
}

fn create_command(command_group: &str, command_string: &str, attributes_string: &str) -> String {
    format!("{}{}/{}{}{}", crate::constants::PREFIX, command_group, command_string,
            attributes_string, crate::constants::POSTFIX)
}

impl HeosCommands for HeosDevice {
    fn command_from(&self, command_group: &str, command_string: &str,
                    attributes: Vec<(&str, &str)>) -> Result<String>
    {
        let mut attrs = attributes.clone();
        attrs.push(("pid", self.player_id.as_str()));

        let attributes_string = attrs.to_heos_attrs()?;

        Ok(create_command(command_group, command_string, attributes_string.as_str()))
    }
}

impl HeosCommands for Heos {
    fn command_from(&self, command_group: &str, command_string: &str,
                    attributes: Vec<(&str, &str)>) -> Result<String>
    {
        let attributes_string = attributes.to_heos_attrs()?;

        Ok(create_command(command_group, command_string, attributes_string.as_str()))
    }
}
