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

use tinyjson::JsonValue;
use anyhow::Result;

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
        let parsed: JsonValue = response_str.parse()?;

        if let Some(heos) = parsed.get().get("heos") {
            return match heos.get()?.get("command").unwrap() {
                Some("player/get_players") => Self {
                    kind: HeosReplyKind::GetPlayers,
                },
                _ => None,
            }
        }

        Err
    }

    pub fn kind(&self) -> HeosReplyKind {
        self.kind.to_owned()
    }
}