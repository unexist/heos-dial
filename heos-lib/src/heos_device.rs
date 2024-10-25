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

use surf::Url;
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct HeosDevice {
    pub(crate) base_url: Url,
}

impl HeosDevice {
    pub fn new(url: &str) -> Result<Self> {
        Ok(Self {
            base_url: Url::parse(url)?,
        })
    }

    pub fn connect() -> Result<Self> {
       Ok(Self)
    }

    pub fn ip(&self) -> String {
        self.base_url.host_str().unwrap().to_string()
    }
}
