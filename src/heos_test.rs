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

use crate::heos::Heos;
use super::*;
use std::assert_matches::assert_matches;

#[test]
fn should_parse_discovery_response() -> std::io::Result<()> {
    const RAW_REPLY: &'static str = "HTTP/1.1 200 OK\r\nCACHE-CONTROL: max-age=180\r\nEXT:\r\nLOCATION: http://10.0.8.87:60006/upnp/desc/aios_device/aios_device.xml\r\nVERSIONS.UPNP.HEOS.COM: 10,668205267,801394619,363364703,1840750642,105553199,-316033077,1711326982,-838802320,-170053632,363364703\r\nNETWORKID.UPNP.HEOS.COM: d424dda645d7\r\nBOOTID.UPNP.ORG: 1947595085\r\nIPCACHE.URL.UPNP.HEOS.COM: /ajax/upnp/get_device_info\r\nSERVER: LINUX UPnP/1.0 Denon-Heos/316763a47eba7769d9be106fb4f3617c5393a2b7\r\nST: urn:schemas-denon-com:device:ACT-Denon:1\r\nUSN: uuid:60f346a0-9018-49e7-b77e-4a14ad25b96f::urn:schemas-denon-com:device:ACT-Denon:1\r\n\r\n";

    assert_matches!(Heos::parse_discovery_response(RAW_REPLY), Ok(_));
}