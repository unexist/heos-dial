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

pub(crate) trait HeosAttributes {
    fn to_attrs_str(&self) -> Result<String>;
}

fn attributes_string_from(attrs: Vec<(&str, &str)>) -> String {
    if attrs.is_empty() {
        "".into()
    } else {
        match attrs.iter()
            .map(|kv| { format!("{}={}", kv.0, kv.1) })
            .reduce(|prev, next| { format!("{}&{}", prev, next) })
        {
            Some(result) => format!("?{}", result),
            None => "".into()
        }
    }
}

impl HeosAttributes for [(&str, &str)] {
    fn to_attrs_str(&self) -> Result<String> {
        Ok(attributes_string_from(self.to_vec()))
    }
}
