#![feature(let_chains)]
#![feature(assert_matches)]

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

mod heos;

#[cfg(test)]
mod heos_test;

pub use heos::{Heos, HeosDevice};