# heos-lib

![License](https://img.shields.io/badge/License-GNU%20GPL3-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/heos-lib.svg)](https://crates.io/crates/heos-lib)
[![Protocol Spec](https://img.shields.io/badge/Protocol_version-1.14-blue)](https://assets.denon.com/documentmaster/us/heos_cli_protocolspecification-version_04062020.pdf)

These bindings are my own take on providing a small library to communicate and control with HEOS devices.

## Getting Started

```rust
use heos::{Heos, HeosReply};
use heos::heos_command::HeosCommand;
use futures::pin_mut;;
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let devices = Heos::discover().await
        expect("To discover devices");
    pin_mut!(devices);

    let mut cmd = HeosCommand::new()
        .group("player")
        .cmd("get_players");

    while let Some(dev) = devices.next().await {
        dev.send_command(&cmd).await
            .expect("To send command");

        if let HeosReply::Players(success, mut players) = reply {
            if success {
                for player in players {
                    println!("player={}", player);
                }
            }
        }
    }
}
```

## Documentation

There will soon be docs available, for now please have a look at the test suite
or the demo TUI app:

* https://github.com/unexist/heos-dial/tree/master/heos-lib/src
* https://github.com/unexist/heos-dial/tree/master/heos-tui

## Roadmap

* Improve device discovery
* Make the lib ready to be used by Embassy
* Maybe no_std?
