use std::fmt::{Display, Formatter};
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
use tokio::io;
use tokio::net::TcpStream;
use crate::constants::DEFAULT_PORT;
use crate::heos_command::{HeosCommand, HeosCommandHandler};
use crate::heos_reply::HeosReply;

#[derive(Debug)]
pub struct HeosDevice {
    pub name: String,
    pub model: String,
    pub base_url: String,
    pub player_id: String,
    pub group_id: String,
    pub volume: u16,
    pub stream: Option<TcpStream>,
}

impl HeosDevice {
    pub fn new(name: &str, url: &str, pid: &str) -> Result<Self> {
        Ok(Self {
            name: name.into(),
            model: Default::default(),
            base_url: url.parse()?,
            player_id: pid.into(),
            group_id: Default::default(),
            volume: 0,
            stream: None,
        })
    }

    pub async fn connect(&mut self) -> Result<()> {
        /* Sanity check to prevent re-connection */
        Ok(match self.stream {
            Some(_) => (),
            None => {
                self.stream = Some(TcpStream::connect(
                    format!("{}:{}", self.base_url, DEFAULT_PORT)).await?)
            }
        })
    }

    pub async fn update_info(&mut self) -> Result<()> {
        self.connect().await?;

        let cmd = HeosCommand::new()
            .group("player")
            .cmd("get_player_info");

        let reply = self.send_command(&cmd).await?;

        if let HeosReply::PlayerInfo(success, device) = reply {
            if success {
                self.name = device.name;
                self.player_id = device.player_id;
            }
        } else if let HeosReply::Error(_, _, message) = reply {
            return Err(anyhow!(message.get("text").expect("Expected error test to be set")));
        }

        Ok(())
    }

    pub async fn update_volume(&mut self) -> Result<()> {
        self.connect().await?;

        let cmd = HeosCommand::new()
            .group("player")
            .cmd("get_volume");

        let reply = self.send_command(&cmd).await?;

        if let HeosReply::Volume(success, attrs) = reply {
            if success {
                self.volume = attrs.get("level").unwrap().parse::<u16>()?;
            }
        } else if let HeosReply::Error(_, _, message) = reply {
            return Err(anyhow!(message.get("text").expect("Expected error test to be set")));
        }

        Ok(())
    }
}

impl HeosCommandHandler for HeosDevice {
    async fn send_command<'a>(&mut self, cmd: &HeosCommand<'a>) -> Result<HeosReply> {
        self.connect().await?;

        /* Append player id */
        let dev_cmd = cmd.clone().attr("pid", self.player_id.as_str());

        match self.stream.as_ref() {
            Some(stream) => {
                stream.try_write(dev_cmd.to_string().as_bytes())?;

                let mut buf = Vec::with_capacity(2048);

                loop {
                    stream.readable().await?;

                    match stream.try_read_buf(&mut buf) {
                        Ok(0) => break,
                        Ok(_n) => {
                            #[cfg(test)]
                            println!("Read {} bytes", _n);
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            if '\r' == char::from(buf[buf.len() - 2])
                                && '\n' == char::from(buf[buf.len() - 1]) {
                                break;
                            }
                        }
                        Err(e) => {
                            return Err(anyhow!(e));
                        }
                    }
                }

                return Ok(HeosReply::parse(String::from_utf8(buf)?.as_str())?)
            }
            _ => {}
        }

        Err(anyhow!("Failed to send command"))
    }
}

impl Clone for HeosDevice {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            model: self.model.clone(),
            base_url: self.base_url.clone(),
            player_id: self.player_id.clone(),
            group_id: self.group_id.clone(),
            volume: self.volume,
            stream: None,
        }
    }
}

impl PartialEq for HeosDevice {
    fn eq(&self, other: &Self) -> bool {
        self.player_id == other.player_id
    }
}

impl Display for HeosDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.base_url)
    }
}
