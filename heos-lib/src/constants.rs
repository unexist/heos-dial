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

pub(crate) const PREFIX: &'static str = "heos://";
pub(crate) const POSTFIX: &'static str = "\r\n";
pub(crate) const TARGET_URN: &'static str = "urn:schemas-denon-com:device:ACT-Denon:1";
pub (crate) const LOCATION: &'static str = "http://10.0.8.87:60006/upnp/desc/aios_device/aios_device.xml";