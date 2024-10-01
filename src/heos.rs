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

use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use anyhow::anyhow;
use std::collections::HashMap;
use esp_idf_svc::sys::const_format::formatcp;

const TARGET_URN: &'static str = "urn:schemas-denon-com:device:ACT-Denon:1";
const DISCOVERY_REQUEST: &'static str = formatcp!(#"M-SEARCH * HTTP/1.1\r\n
HOST: 239.255.255.250:1900\r\n
MAN: \"ssdp:discover\"\r\n
MX: 5\r\n
ST: {urn}\r\n
\r\n"#, urn = TARGET_URN);

#[derive(Debug)]
pub struct HeosDevice<'d> {
}

#[derive(Default)]
pub struct Heos<'d> {
}

impl<'d> Heos<'d> {
    pub fn new() -> Self {
        Self { }
    }

    pub async fn discover(&self) -> Self {
        let any: SocketAddr = ([0, 0, 0, 0], 0).into();
        let socket = UdpSocket::bind(any).await?;
        socket.join_multicast_v4(Ipv4Addr::new(239, 255, 255, 250),
                                 Ipv4Addr::new(0, 0, 0, 0))?;

        // Set the socket address to the multicast IP and port for UPnP device discovery
        let socket_addr: SocketAddr = ([239, 255, 255, 250], 1900).into();

        // Send the discovery request
        socket.send_to(DISCOVERY_REQUEST.as_bytes(), &socket_addr).await?;

        loop {
            // Receive the discovery response
            let mut buf = [0; 2048];
            let (size, _) = socket.recv_from(&mut buf).await?;

            // Convert the response to a string
            let response =
                str::from_utf8(unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, size) })?;

            let headers = Self::parse_discovery_response(response)
                .ok_or_else(|| anyhow!("Couldn't parse response"))?
                .to_string();
        }
    }

    pub(crate) fn parse_discovery_response(response_str: &str) -> Result<HeosDevice, Err> {
        Ok(HeosDevice{})
    }
}
