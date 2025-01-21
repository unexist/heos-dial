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

use std::fmt::Display;
use anyhow::Result;
use crate::heos_reply::HeosReply;

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

    pub fn attr(mut self, key: &'a str, value: &'a str) -> Self {
        if let Some(attrs) = self.attrs.as_mut() {
            attrs.push((key, value));
        } else {
            self.attrs = Some(vec![(key, value)]);
        }

        self
    }
}

fn format_attributes(attrs: Option<&Vec<(&str, &str)>>) -> String {
    if attrs.is_none() || attrs.unwrap().is_empty() {
        "".into()
    } else {
        match attrs.unwrap().iter()
            .map(|kv| { format!("{}={}", kv.0, kv.1) })
            .reduce(|prev, next| { format!("{}&{}", prev, next) })
        {
            Some(result) => format!("?{}", result),
            None => "".into()
        }
    }
}

impl<'a> Display for HeosCommand<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}/{}{}{}", crate::constants::CMD_PREFIX,
               self.group.expect("Group missing"),
               self.cmd.expect("Cmd missing"),
               format_attributes(self.attrs.as_ref()),
               crate::constants::CMD_POSTFIX)
    }
}

pub trait HeosCommandHandler {
    #[allow(async_fn_in_trait)]
    async fn send_command<'a>(&self, cmd: &HeosCommand<'a>) -> Result<HeosReply>;
}
