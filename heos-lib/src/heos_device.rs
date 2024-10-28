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



    pub(crate) fn command_from(command_group: &str, command_string: &str,
                               attributes: Vec<(&str, &str)>) -> String
    {
        format!("{}{}/{}{}{}", crate::heos::PREFIX, command_group, command_string,
            attributes.to_heos_attrs().expect("Parsing attributes failed"), crate::heos::POSTFIX)
    }
}


