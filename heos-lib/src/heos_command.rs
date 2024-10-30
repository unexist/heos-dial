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

pub(crate) trait HeosCommand {
    fn command_from(&self, cmd_group_string: &str, cmd_string: &str,
                    attrs: Vec<(&str, &str)>) -> Result<String>;
}

fn create_command(cmd_group_string: &str, cmd_string: &str,
                  attrs_string: &str) -> String
{
    format!("{}{}/{}{}{}", crate::constants::PREFIX, cmd_group_string, cmd_string, attrs_string,
            crate::constants::POSTFIX)
}

impl HeosCommand for HeosDevice {
    fn command_from(&self, cmd_group_string: &str, cmd_string: &str,
                    attrs: Vec<(&str, &str)>) -> Result<String>
    {
        let mut attrs = attrs.clone();
        attrs.push(("pid", self.player_id.as_str()));

        let attrs_string = attrs.to_heos_attrs()?;

        Ok(create_command(cmd_group_string, cmd_string, attrs_string.as_str()))
    }
}

impl HeosCommand for Heos {
    fn command_from(&self, cmd_group_string: &str, cmd_string: &str,
                    attrs: Vec<(&str, &str)>) -> Result<String>
    {
        let attrs_string = attrs.to_heos_attrs()?;

        Ok(create_command(cmd_group_string, cmd_string, attrs_string.as_str()))
    }
}
