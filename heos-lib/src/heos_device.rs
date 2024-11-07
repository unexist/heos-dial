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
use surf::Url;

#[derive(Debug)]
pub struct HeosDevice {
    pub(crate) base_url: Url,
    pub(crate) player_id: String,
}

impl HeosDevice {
    pub fn new(url: &str, pid: &str) -> Result<Self> {
        Ok(Self {
            base_url: Url::parse(url)?,
            player_id: pid.into(),
        })
    }

    pub async fn send(&self, heos_cmd_str: &str) -> Result<String> {
        let response = surf::post(self.base_url.as_ref())
            .body(heos_cmd_str)
            .recv_string()
            .await;

        match response {
            Ok(body) => Ok(body),
            Err(err) => Err(err.into_inner())
        }
    }
}
