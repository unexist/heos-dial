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

use anyhow::{Result};

#[derive(Debug, Clone)]
pub struct HeosDevice {
    pub(crate) base_url: String,
    player_id: String,
}

impl HeosDevice {
    pub fn new(url: &str) -> Result<Self> {
        Ok(Self {
            base_url: url.into(),
            player_id: "".into(),
        })
    }

    pub(crate) fn attributes_from(attributes: Vec<(&str, &str)>) -> String {
        if attributes.is_empty() {
            "".into()
        } else {
            match attributes.iter()
                .map(|kv| { format!("{}={}", kv.0, kv.1) })
                .reduce(|prev, next| { format!("{}&{}", prev, next) })
            {
                Some(result) => format!("?{}", result),
                None => "".into()
            }
        }
    }

    pub(crate) fn command_from(command_group: &str, command_string: &str,
                               attributes: Vec<(&str, &str)>) -> String
    {
        format!("{}{}/{}{}{}", crate::heos::PREFIX, command_group, command_string,
                Self::attributes_from(attributes), crate::heos::POSTFIX)
    }
}


