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

mod constants;
mod macros;

pub mod heos;
pub mod heos_device;
pub mod heos_group;
pub mod heos_command;
pub mod heos_reply;

mod heos_test;
mod heos_device_test;
mod heos_group_test;
mod heos_command_test;
mod heos_reply_test;

pub use heos::Heos;
pub use heos_device::HeosDevice;
pub use heos_group::HeosGroup;
pub use heos_reply::HeosReply;
