use std::fmt::format;
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
    pub(crate) base_url: String,
    pub(crate) player_id: String,
    stream: Option<TcpStream>,
}

impl HeosDevice {
    pub fn new(url: &str, pid: &str) -> Result<Self> {
        Ok(Self {
            base_url: url.parse()?,
            player_id: pid.into(),
            stream: None,
        })
    }

    pub async fn connect(&mut self) -> Result<()> {
        self.stream = Some(TcpStream::connect(
            format!("{}:{}", self.base_url, DEFAULT_PORT)).await?);

        Ok(())
    }
}

impl HeosCommandHandler for HeosDevice {
    async fn send_command<'a>(&self, cmd: &HeosCommand<'a>) -> Result<HeosReply> {
        /* Append player id */
        let mut dev_cmd = cmd.clone().attr("pid", self.player_id.as_str());

        match self.stream.as_ref() {
            Some(stream) => {
                stream.try_write(dev_cmd.to_string().as_bytes())?;

                let mut buf = Vec::with_capacity(2048);

                loop {
                    stream.readable().await?;

                    match stream.try_read_buf(&mut buf) {
                        Ok(0) => break,
                        #[cfg(test)]
                        Ok(n) => {
                            println!("Read {} bytes", n);
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            if '\r' == char::from(buf[buf.len() - 2]) && '\n' == char::from(buf[buf.len() - 1]) {
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
            base_url: self.base_url.clone(),
            player_id: self.player_id.clone(),
            stream: None,
        }
    }
}

impl PartialEq for HeosDevice {
    fn eq(&self, other: &Self) -> bool {
        self.player_id == other.player_id
    }
}
