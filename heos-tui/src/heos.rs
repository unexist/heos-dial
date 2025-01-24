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
use tokio::sync::Mutex;
use std::sync::Arc;
use heos_lib::{Heos, HeosDevice};
use futures::StreamExt;

pub(crate) async fn discover_devices(list: Arc<Mutex<Vec<HeosDevice>>>) {
    let arc_list = Arc::clone(&list);

    let devices = Heos::new().discover().await
        .expect("To discover devices");
    pin_mut!(devices);

    match devices.next().await {
        Some(dev) => {
            let mut dev_list: Vec<HeosDevice> = arc_list.lock().await.to_vec();

            dev_list.push(dev);
        },
        None => println!("Failed to discover devices"),
    }
}
