///
/// @package heos-dial
///
/// @file HEOS tui
/// @copyright 2024-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use futures::pin_mut;
use arc_swap::ArcSwap;
use heos_lib::{Heos, HeosDevice};
use futures::StreamExt;

pub(crate) async fn discover_devices(arc_list: ArcSwap<Vec<HeosDevice>>) {
    let devices = Heos::new().discover().await
        .expect("To discover devices");
    pin_mut!(devices);

    match devices.next().await {
        Some(dev) => {
            let mut dev_list: Vec<HeosDevice> = arc_list.load().to_vec();

            dev_list.push(dev);
        },
        None => println!("Failed to discover devices"),
    }
}
