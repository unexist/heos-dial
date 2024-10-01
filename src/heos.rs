///
/// @package heos-dial
///
/// @file HEOS protocol helper
/// @copyright 2024-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use std::net::UdpSocket;

const TargetUrn: &'static str = "urn:schemas-denon-com:device:ACT-Denon:1";
const DiscoverMessage: &'static str = "M-SEARCH * HTTP/1.1\r\n \
    HOST: 239.255.255.250:1900\r\n \
    ST: {urn}\r\n \
    MX: 5\r\n \
    MAN: \"ssdp:discover\"\r\n \
    \r\n";

pub struct Heos<'d> {
    socket: UdpSocket,
}

impl<'d> Heos<'d> {
    pub fn new(socket: UdpSocket) -> Self {
        Self { socket }
    }

    pub fn discovery() -> Self {
        let mut buf = [0; 10];
        buf.reverse();

        socket.send_to(buf, &src)?;

        let buf = &mut buf[..amt];
        let (amt, src) = socket.recv_from(&mut buf)?;
    }
}