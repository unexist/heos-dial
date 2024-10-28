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

pub(crate) trait HeosAttributes {
    fn to_heos_attrs(&self) -> anyhow::Result<String>;
}

fn attributes_from(attributes: Vec<(&str, &str)>) -> String {
    if attributes.is_empty() {
        "".into()
    } else {
        match attributes.iter()
            .map(|kv| { format!("{}={}", kv.0, kv.1) })
            .reduce(|prev, next| { format!("{}&{}", prev, next) })
        {
            Some(result) => format!("?{}", result),
            None => "".into()
        }
    }
}

impl HeosAttributes for [(&str, &str)] {
    fn to_heos_attrs(&self) -> anyhow::Result<String> {
        Ok(attributes_from(self.to_vec()))
    }
}