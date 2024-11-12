use std::fmt::Display;
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

use anyhow::Result;
use crate::heos_attributes::HeosAttributes;

#[derive(Default, Clone)]
pub struct HeosCommand<'a> {
    group: Option<&'a str>,
    cmd: Option<&'a str>,
    attrs: Option<Vec<(&'a str, &'a str)>>,
}

impl<'a> HeosCommand<'a> {
    pub fn new() -> Self {
        Self {
            group: None,
            cmd: None,
            attrs: None,
        }
    }

    pub fn group(mut self, cmd_group_string: &'a str) -> Self {
        self.group = Some(cmd_group_string);

        self
    }

    pub fn cmd(mut self, cmd_string: &'a str) -> Self {
        self.cmd = Some(cmd_string);

        self
    }

    pub fn attrs(mut self, attrs: Vec<(&'a str, &'a str)>) -> Self {
        self.attrs = Some(attrs);

        self
    }

    pub fn attr(mut self, key: &'a str, value: &'a str) {
        if self.attrs.is_none() {
            self.attrs = Some(vec![]);
        }

        self.attrs.unwrap().push((key, value));
    }

    pub fn build(self) -> String {
        self.to_string()
    }
}

impl<'a> Display for HeosCommand<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}/{}{:?}{}", crate::constants::CMD_PREFIX,
               self.group.expect("Group missing"), self.cmd.expect("Cmd missing"),
               self.attrs.to_attrs_str(), crate::constants::CMD_POSTFIX)
    }
}

pub(crate) trait HeosCommandHandler {
    async fn send_command<'a>(&self, cmd: &HeosCommand<'a>) -> Result<String>;
}
