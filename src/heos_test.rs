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

use std::net::UdpSocket;
use crate::heos::Heos;
use super::*;

#[test]
fn discover_test() -> std::io::Result<()> {
    let socket = UdpSocket::bind("239.255.255.250:1900")?;

    let _heos = Heos.new(socket);
}