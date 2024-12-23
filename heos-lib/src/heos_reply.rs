use std::collections::HashMap;
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
use gjson::Value;
use crate::HeosDevice;

#[derive(Clone, PartialEq, Debug)]
pub enum HeosReply {
    Players(bool, Vec<HeosDevice>),
    PlayState(bool, HashMap<String, String>),
    PlayAction(bool, HashMap<String, String>),
    Volume(bool, HashMap<String, String>),
}

impl HeosReply {
    pub(crate) fn parse(response_str: &str) -> Result<HeosReply> {
        let json = gjson::parse(response_str);

        match json.get("heos.command").str() {
            "player/get_players" => Ok(HeosReply::Players(
                "success" == json.get("heos.result").str(),
                Self::parse_players_payload(&json, "heos.payload")
            )),

            "player/get_play_state" | "player/set_play_state" => Ok(HeosReply::PlayState(
                "success" == json.get("heos.result").str(),
                Self::parse_message(&json, "heos.message")
            )),

            "player/set_volume" | "player/get_volume" => Ok(HeosReply::Volume(
                "success" == json.get("heos.result").str(),
                Self::parse_message(&json, "heos.message")
            )),

            "player/play_next" | "player/play_previous" => Ok(HeosReply::PlayAction(
                "success" == json.get("heos.result").str(),
                Self::parse_message(&json, "heos.message")
            )),

            _ => Err(anyhow!("Command type unknown")),
        }
    }

    pub(crate) fn parse_message(json: &Value, path: &str) -> HashMap<String, String> {
        json.get(path).str()
            .split("&")
            .filter_map(|s| {
                s.split_once("=")
                    .and_then(|t| Some((t.0.to_owned(), t.1.to_owned())))
            })
            .collect()
    }

    pub(crate) fn parse_players_payload(json: &Value, path: &str) -> Vec<HeosDevice> {
        json.get(path).array().iter()
            .map(|v| {
                print!("{:?}", json.get("ip").str());
                HeosDevice::new(json.get("ip").str(),
                                json.get("player_id").str()).unwrap()
            })
            .collect()
    }
}
