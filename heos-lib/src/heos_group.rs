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

use crate::heos_command::{HeosCommand, HeosCommandHandler};
use crate::{HeosDevice, HeosReply};
use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct HeosGroup {
    pub name: String,
    pub group_id: String,
    pub leader: Option<HeosDevice>,
    pub players: Option<Vec<HeosDevice>>,
    pub volume: u16,
}

impl HeosGroup {
    pub fn new(name: &str, group_id: &str) -> Self {
        Self {
            name: name.into(),
            group_id: group_id.into(),
            leader: None,
            players: None,
            volume: 0,
        }
    }

    pub async fn update_volume(&mut self) -> Result<()> {
        let cmd = HeosCommand::new()
            .group("group")
            .cmd("get_volume");

        let reply = self.send_command(&cmd).await?;

        if let HeosReply::Volume(success, attrs) = reply {
            if success {
                self.volume = attrs.get("level").unwrap().parse::<u16>()?;
            }
        } else if let HeosReply::Error(_, _, message) = reply {
            return Err(anyhow!(message.get("text")
                .expect("Expected error test to be set").to_string()));
        }

        Ok(())
    }
}

impl HeosCommandHandler for HeosGroup {
    async fn send_command<'a>(&mut self, cmd: &HeosCommand<'a>) -> Result<HeosReply> {
        match self.leader {
            Some(ref mut leader) => {
                Ok(leader.send_command(cmd).await?)
            },
            None => Err(anyhow!("No leader found")),
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
            volume: self.volume,
        }
    }
}

impl PartialEq for HeosGroup {
    fn eq(&self, other: &Self) -> bool {
        self.group_id == other.group_id
    }
}

impl Display for HeosGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (leader: {:?})", self.name, self.leader)
    }
}