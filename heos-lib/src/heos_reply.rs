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
use std::collections::HashMap;
use crate::heos_group::HeosGroup;
use crate::HeosDevice;

#[derive(Clone, PartialEq, Debug)]
pub enum HeosReply {
    Players(bool, Vec<HeosDevice>),
    Groups(bool, Vec<HeosGroup>),
    PlayerInfo(bool, HeosDevice),
    GroupInfo(bool, HeosGroup),
    PlayState(bool, HashMap<String, String>),
    PlayAction(bool, HashMap<String, String>),
    PlayingMedia(bool, HashMap<String, String>),
    Volume(bool, HashMap<String, String>),
    Mute(bool, HashMap<String, String>),
    Error(bool, String, HashMap<String, String>),
}

impl HeosReply {
    pub fn parse(response_str: &str) -> Result<HeosReply> {
        let json = gjson::parse(response_str);

        /* Check for error */
        if "fail".eq(json.get("heos.result").str()) {
            return Ok(HeosReply::Error(false, json.get("heos.command").to_string(),
                Self::parse_message(&json, "heos.message")))
        }

        match json.get("heos.command").str() {
            "player/get_players" => Ok(HeosReply::Players(
                "success" == json.get("heos.result").str(),
                Self::parse_players_payload(&json, "payload", "")
            )),

            "player/get_groups" => Ok(HeosReply::Groups(
                "success" == json.get("heos.result").str(),
                Self::parse_groups_payload(&json, "payload")
            )),

            "player/get_player_info" => Ok(HeosReply::PlayerInfo(
                "success" == json.get("heos.result").str(),
                Self::parse_player(&json.get("payload"))
            )),

            "player/get_group_info" => Ok(HeosReply::GroupInfo(
                "success" == json.get("heos.result").str(),
                Self::parse_group(&json.get("payload"))
            )),

            "player/get_play_state" | "player/set_play_state" => Ok(HeosReply::PlayState(
                "success" == json.get("heos.result").str(),
                Self::parse_message(&json, "heos.message")
            )),

            "player/play_next" | "player/play_previous" => Ok(HeosReply::PlayAction(
                "success" == json.get("heos.result").str(),
                Self::parse_message(&json, "heos.message")
            )),

            "player/get_now_playing_media" => Ok(HeosReply::PlayingMedia(
                "success" == json.get("heos.result").str(),
                Self::parse_generic_payload(&json, "payload")
            )),

            "player/set_volume" | "player/get_volume"
            | "group/set_volume" | "group/get_volume" => Ok(HeosReply::Volume(
                "success" == json.get("heos.result").str(),
                Self::parse_message(&json, "heos.message")
            )),

            "player/set_mute" | "player/get_mute" | "player/toggle_mute"
            | "group/set_mute" | "group/get_mute" | "group/toggle_mute" => Ok(HeosReply::Mute(
                "success" == json.get("heos.result").str(),
                Self::parse_message(&json, "heos.message")
            )),

            cmd => Err(anyhow!("Command type `{:?}` unknown", cmd)),
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

    pub fn parse_player(json: &Value) -> HeosDevice {
        let mut player = HeosDevice::new(json.get("name").str(),
                                         json.get("ip").str(),
                                         json.get("pid").str()).unwrap();

        player.model = json.get("model").str().into();

        player
    }

    pub fn parse_group(json: &Value) -> HeosGroup {
        HeosGroup::new(json.get("name").str(),
                       json.get("gid").str())
    }


    pub(crate) fn parse_generic_payload(json: &Value, path: &str) -> HashMap<String, String> {
        let mut payload: HashMap<String, String> = HashMap::new();

        json.get(path).each(|key, value| {
            payload.insert(key.to_string(), value.to_string()).is_none()
        });

        payload
    }

    pub(crate) fn parse_players_payload(json: &Value, path: &str, group_id: &str) -> Vec<HeosDevice> {
        json.get(path).array().iter()
            .map(|v| {
                let mut player = Self::parse_player(v);

                player.group_id = group_id.to_string();

                player
            })
            .collect()
    }

    pub(crate) fn parse_groups_payload(json: &Value, path: &str) -> Vec<HeosGroup> {
        json.get(path).array().iter()
            .map(|v| {
                let mut group = Self::parse_group(v);

                group.players = Some(Self::parse_players_payload(v, "players", &*group.group_id));

                v.get("players").array().iter().for_each(|player| {
                    if "leader".eq(player.get("role").str()) {
                        group.leader = Some(Self::parse_player(player));

                        return;
                    }
                });

                group
            })
            .collect()
    }
}
