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

use crate::HeosDevice;

pub(crate) trait HeosAttributes {
    fn to_heos_attrs(&self) -> anyhow::Result<String>;
}

impl HeosAttributes for [(&str, &str)] {
    fn to_heos_attrs(&self) -> anyhow::Result<String> {
        Ok(HeosDevice::attributes_from(self.to_vec()))
    }
}