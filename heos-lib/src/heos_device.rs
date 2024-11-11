use std::io::BufReader;
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
use url::Url;
use crate::constants::DEFAULT_PORT;
use crate::heos_command::HeosCommand;

#[derive(Debug)]
pub struct HeosDevice {
    pub(crate) base_url: Url,
    pub(crate) player_id: String,
    stream: TcpStream,
}

impl HeosDevice {
    pub fn new(url: &str, pid: &str) -> Result<Self> {
        Ok(Self {
            base_url: Url::parse(url)?,
            player_id: pid.into(),
        })
    }

    pub async fn connect(&mut self) -> Result<()> {
        self.stream = TcpStream::connect(
            format!("{}:{}", self.base_url.host(), DEFAULT_PORT)).await?;

        Ok(())
    }

    pub async fn send_cmd(&mut self, cmd: HeosCommand) -> Result<()> {
        self.stream.try_write(cmd.into()?.as_bytes())?;

        loop {
            self.stream.readable().await?;

            let mut buf = [0; 4096];

            match self.stream.try_read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    println!("read {} bytes", n);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }
}
