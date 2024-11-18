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
use std::collections::HashMap;

#[derive(Default, Clone, PartialEq, Debug)]
pub enum HeosReplyKind {
    #[default]
    GetPlayers
}

pub struct HeosReply {
    kind: HeosReplyKind,
}

impl HeosReply {
    pub(crate) fn parse(response_str: &str) -> Option<HeosReply> {
        let parsed: JsonValue = response_str.parse().ok()?;
        let object: &HashMap<_, _> = parsed.get()?;

        if let Some(heos) = object.get("heos") {
        }

        Some(Self)
    }

    pub fn kind(&self) -> HeosReplyKind {
        self.kind.to_owned()
    }
}