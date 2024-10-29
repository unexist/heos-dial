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

#[derive(Debug, Clone)]
pub struct HeosDevice {
    pub (crate) base_url: String,
    pub(crate) player_id: String,
}

impl HeosDevice {
    pub fn new(url: &str, pid: &str) -> Result<Self> {
        Ok(Self {
            base_url: url.into(),
            player_id: pid.into(),
        })
    }
}


