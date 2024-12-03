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
use jsonpath_rust::JsonPath;
use std::str::FromStr;

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
        let parsed = serde_json::from_str(response_str)?;
        let path = JsonPath::from_str("$.heos.command")?;

        match path.find_slice(parsed) {
            "player/get_players" => Ok(Self {
                kind: HeosReplyKind::GetPlayers,
            }),
            _ => Err,
        }

        Err
    }

    pub fn kind(&self) -> HeosReplyKind {
        self.kind.to_owned()
    }
}