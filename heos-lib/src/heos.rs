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
use anyhow::{anyhow, Result};
use async_stream::stream;
use const_format::formatcp;
use futures_util::Stream;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::UdpSocket;
use crate::constants::TARGET_URN;
use crate::heos_device::HeosDevice;

const DISCOVERY_REQUEST: &'static str = formatcp!("M-SEARCH * HTTP/1.1\r\n\
HOST: 239.255.255.250:1900\r\n\
ST: {urn}\r\n\
MX: 5\r\n\
MAN: \"ssdp:discover\"\r\n\r\n" , urn = TARGET_URN);

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

    pub async fn discover(&self) -> Result<impl Stream<Item = HeosDevice>>  {
        let any: SocketAddr = ([0, 0, 0, 0], 0).into();
        let socket = UdpSocket::bind(any).await?;
        socket.join_multicast_v4(Ipv4Addr::new(239, 255, 255, 250),
                                 Ipv4Addr::new(0, 0, 0, 0))?;

        // Set the socket address to the multicast IP and port for UPnP device discovery
        let socket_addr: SocketAddr = ([239, 255, 255, 250], 1900).into();

        // Send the discovery request
        socket.send_to(DISCOVERY_REQUEST.as_bytes(), &socket_addr).await?;

        Ok(stream! {
            loop {
                async fn get_next(socket: &UdpSocket) -> Result<String> {
                     // Receive the discovery response
                    let mut buf = [0; 2048];
                    let (size, _) = socket.recv_from(&mut buf).await?;

                    // Convert the response to a string
                    let response = str::from_utf8(&buf[..size])?;

                    if response.contains(TARGET_URN) {
                        Ok(response.to_string())
                    } else {
                        Err(anyhow!("Target urn not found"))
                    }
                }

                if let Ok(response) = get_next(&socket).await {
                    match Self::parse_discovery_response(&response) {
                        Ok(location) => {
                            let url = Heos::parse_location(location.as_ref())?;

                            yield HeosDevice::new(&*url, "0")?;
                        },
                        Err(err) => println!("{:#?}", err),
                    }
                }
            }
        })
    }

    pub(crate) fn parse_discovery_response(response_str: &str) -> Result<String> {
        match response_str.split("\r\n\r\n").next() {
            Some(header_str) => {
                for header_line in header_str.split("\r\n") {
                    if header_line.contains("LOCATION") {
                        if let Some(idx) = header_line.find(":") {
                            let location = header_line[idx + 1..].trim();

                            return Ok(String::from(location));
                        }
                    }
                }
            },
            None => {}
        }

        Err(anyhow!("Invalid response"))
    }

    pub(crate) fn parse_location(location_str: &str) -> Result<String> {
        let mut start: usize = 0;
        let mut stop: usize = 0;

        for c in location_str.char_indices() {
            if '/' == c.1 && ((0 == start && 0 == stop) || (0 <= start && 0 == stop)) {
                start = c.0;
            } else if (':' == c.1 && 0 < start && 0 == stop) || ('/' == c.1 && 0 < start) {
                stop = c.0;

                break;
            }
        }

        if stop > start {
            Ok(location_str[start..stop].to_string())
        } else {
            Err(anyhow!("Invalid location"))
        }
    }
}
