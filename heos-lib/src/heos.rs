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

use std::str;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::collections::HashMap;
use anyhow::anyhow;
use const_format::formatcp;

const PREFIX: &'static str = "heos://";
const POSTFIX: &'static str = "\r\n";
const TARGET_URN: &'static str = "urn:schemas-denon-com:device:ACT-Denon:1";
const DISCOVERY_REQUEST: &'static str = formatcp!(r#"M-SEARCH * HTTP/1.1\r\n
HOST: 239.255.255.250:1900\r\n
MAN: "ssdp:discover"\r\n
MX: 5\r\n
ST: {urn}\r\n
\r\n"#, urn = TARGET_URN);

#[derive(Default, Debug)]
pub struct HeosDevice {
    pub(crate) _url: String,
}

#[derive(Default)]
pub struct Heos {
    pub(crate) _devices: Vec<HeosDevice>,
}

impl Heos {
    pub fn new() -> Self {
        Self {
            _devices: vec![]
        }
    }

    pub fn discover(&self) -> anyhow::Result<()> {
        let any: SocketAddr = ([0, 0, 0, 0], 0).into();
        let socket = UdpSocket::bind(any)?;
        socket.join_multicast_v4(&Ipv4Addr::new(239, 255, 255, 250),
                                 &Ipv4Addr::new(0, 0, 0, 0))?;

        // Set the socket address to the multicast IP and port for UPnP device discovery
        let socket_addr: SocketAddr = ([239, 255, 255, 250], 1900).into();

        // Send the discovery request
        socket.send_to(DISCOVERY_REQUEST.as_bytes(), &socket_addr)?;

        loop {
            // Receive the discovery response
            let mut buf = [0; 2048];
            let (size, _) = socket.recv_from(&mut buf)?;

            // Convert the response to a string
            let response =
                str::from_utf8(unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, size) })?;

            match Self::parse_discovery_response(response) {
                Ok(device) => println!("{:#?}", device),
                Err(err) => println!("{:#?}", err),
            }
        }
    }

    pub(crate) fn generate_heos_attributes(attributes: &HashMap<&str, &str>) -> String {
        if attributes.is_empty() {
            "".to_string()
        } else {
            match attributes.into_iter()
                .map(|kv| { format!("{}={}", kv.0, kv.1) })
                .reduce(|prev, next| { format!("{}&{}", prev, next) })
            {
                Some(result) => format!("?{}", result),
                None => "".to_string()
            }
        }

    }

    pub(crate) fn generate_heos_command(command_group: &str, command_string: &str,
                                        attributes: &HashMap<&str, &str>) -> String
    {
        format!("{}{}/{}{}{}", PREFIX, command_group, command_string,
                Self::generate_heos_attributes(attributes), POSTFIX)
    }

    pub(crate) fn parse_discovery_response(response_str: &str) -> anyhow::Result<HeosDevice> {
        match response_str.split("\r\n\r\n").next() {
            Some(header_str) => {
                for header_line in header_str.split("\r\n") {
                    if header_line.starts_with("LOCATION") {
                        if let Some(idx) = header_str.find(':') {
                            let url = header_line[idx + 1..].trim();

                            return Ok(HeosDevice { _url: url.to_string() });
                        }
                    }
                }
                Err(anyhow::anyhow!("Invalid response"))
            }
            None => Err(anyhow::anyhow!("Invalid response")),
        }
    }
}
