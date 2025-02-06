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

pub(crate) const DEFAULT_PORT: u16 = 1255;
pub(crate) const CMD_PREFIX: &'static str = "heos://";
pub(crate) const CMD_POSTFIX: &'static str = "\r\n";
pub(crate) const TARGET_URN: &'static str = "urn:schemas-denon-com:device:ACT-Denon:1";
