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

use crate::HeosDevice;

#[derive(Debug)]
pub struct HeosGroup {
    pub name: String,
    pub group_id: String,
    pub leader: Option<HeosDevice>,
    pub players: Option<Vec<HeosDevice>>,
}

impl HeosGroup {
    pub fn new(name: &str, group_id: &str) -> Self {
        Self {
            name: name.into(),
            group_id: group_id.into(),
            leader: None,
            players: None,
        }
    }
}

impl Clone for HeosGroup {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            group_id: self.group_id.clone(),
            leader: None,
            players: None,
        }
    }
}

impl PartialEq for HeosGroup {
    fn eq(&self, other: &Self) -> bool {
        self.group_id == other.group_id
    }
}
