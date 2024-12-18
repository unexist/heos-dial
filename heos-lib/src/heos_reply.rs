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

#[derive(Clone, PartialEq, Debug)]
pub enum HeosReply {
    Players(Vec<String>),
    PlayState(bool, String),
    PlayAction(bool, String),
    Volume(bool, String),
}

impl HeosReply {
    pub(crate) fn parse(response_str: &str) -> Result<HeosReply> {
        let json = gjson::parse(response_str);

        match json.get("heos.command").str() {
            "player/get_players" => Ok(HeosReply::Players(vec![])),

            "player/get_play_state" | "player_set_state" => Ok(HeosReply::PlayState(
                "success" == json.get("heos.result").str(),
                json.get("heos.message").str().to_string()
            )),

            "player/set_volume" | "player/get_colume" => Ok(HeosReply::Volume(
                "success" == json.get("heos.result").str(),
                json.get("heos.message").str().to_string()
            )),

            _ => Err(anyhow!("Command type unknown")),
        }
    }
}
