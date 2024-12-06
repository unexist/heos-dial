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

use anyhow::{anyhow, Result};

#[derive(Default, Clone, PartialEq, Debug)]
pub enum HeosReplyKind {
    #[default]
    GetPlayers
}

#[derive(Debug)]
pub struct HeosReply {
    kind: HeosReplyKind,
}

impl HeosReply {
    pub(crate) fn parse(response_str: &str) -> Result<HeosReply> {
        let value = gjson::get(response_str, "heos.command");

        match value.str() {
            "player/get_players" => Ok(Self {
                kind: HeosReplyKind::GetPlayers,
            }),
            _ => Err(anyhow!("Command type unknown"))
        }
    }

    pub fn kind(&self) -> HeosReplyKind {
        self.kind.to_owned()
    }
}