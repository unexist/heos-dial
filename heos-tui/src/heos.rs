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
use std::sync::Arc;
use heos_lib::{Heos, HeosDevice};
use futures::StreamExt;

pub(crate) async fn discover_devices(dev_list: Arc<ArcSwap<Vec<HeosDevice>>>) {
    let devices = Heos::new().discover().await
        .expect("To discover devices");
    pin_mut!(devices);

    match devices.next().await {
        Some(dev) => {
            let mut swap_list = dev_list.load().to_vec();

            swap_list.push(dev);

            dev_list.swap(Arc::from(swap_list));
        },
        None => println!("Failed to discover devices"),
    }
}
