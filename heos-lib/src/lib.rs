//
/// @package heos-dial
///
/// @file HEOS lib
/// @copyright 2024-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

mod heos;
mod heos_device;

mod heos_test;
mod heos_device_test;

pub use heos::Heos;
pub use heos_device::HeosDevice;
