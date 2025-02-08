///
/// @package heos-dial
///
/// @file HEOS protocol helper tests
/// @copyright 2024-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

#[cfg(test)]
#[macro_export]
macro_rules! test_asset {
    ($s:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/test_assets/", $s))
    }
}
