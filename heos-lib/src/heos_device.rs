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
use tokio::io;
use tokio::net::TcpStream;
use url::Url;
use crate::constants::DEFAULT_PORT;
use crate::heos_command::{HeosCommand, HeosCommandHandler};

#[derive(Debug)]
pub struct HeosDevice {
    pub(crate) base_url: Url,
    pub(crate) player_id: String,
    stream: Option<TcpStream>,
}

impl HeosDevice {
    pub fn new(url: &str, pid: &str) -> Result<Self> {
        Ok(Self {
            base_url: Url::parse(url)?,
            player_id: pid.into(),
            stream: None,
        })
    }

    pub async fn connect(&mut self) -> Result<()> {
        self.stream = Some(TcpStream::connect(
            format!("{}:{}", self.base_url.host().expect("Host failed"),
                    DEFAULT_PORT)).await?);

        Ok(())
    }
}

impl HeosCommandHandler for HeosDevice {
    async fn send_command<'a>(&self, cmd: &HeosCommand<'a>) -> Result<String> {
        /* Append player id */
        let mut dev_cmd = cmd.clone();

        dev_cmd = dev_cmd.attr("pid", self.player_id.as_str());

        match self.stream.as_ref() {
            Some(stream) => {
                stream.try_write(dev_cmd.to_string().as_bytes())?;

                let mut buf = Vec::with_capacity(2048);

                loop {
                    stream.readable().await?;

                    match stream.try_read_buf(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            println!("Read {} bytes", n);
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                }

                return Ok(String::from_utf8(buf)?)
            }
            _ => {}
        }

        Ok("".to_string())
    }
}

