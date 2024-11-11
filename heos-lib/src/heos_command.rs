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

use crate::{Heos, HeosDevice};
use anyhow::Result;

#[derive(Default)]
pub struct HeosCommand<'a> {
    cmd_group_string: &'a str,
    cmd_string: &'a str,
    attrs: Vec<(&'a str, &'a str)>,
}

impl HeosCommand {
    pub fn from<T:HeosCommandHandler>(handler: T, cmd_group_string: &str, cmd_string: &str,
                                      attrs: Vec<(&str, &str)>) -> Self
    {
        let cmd = Self {
            cmd_group_string,
            cmd_string,
            attrs,
        };

        handler.update_command(cmd)
    }

    pub fn add_pair(&mut self, key: &str, value: &str) {
        self.attrs.push((key, value));
    }

    fn create_command(&self) -> String {
        format!("{}{}/{}{}{}", crate::constants::CMD_PREFIX,
                self.cmd_group_string, self.cmd_string, self.attrs,
                crate::constants::CMD_POSTFIX)
    }
}

impl Into<String> for HeosCommand {
    fn into(self) -> String {
        self.create_command()
    }
}

pub(crate) trait HeosCommandHandler {
    fn update_command(cmd: &HeosCommand) -> Result<String>;
}

impl HeosCommandHandler for HeosDevice {
    fn update_command(cmd: &mut HeosCommand) -> Result<HeosCommand> {
        cmd.add_pair("pid", self.player_id.as_str());

        Ok(cmd)
    }
}

impl HeosCommandHandler for Heos {
    fn update_command(cmd: &HeosCommand) -> Result<HeosCommand> {
        Ok(cmd)
    }
}
