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

use crossterm::event::{Event as CrosstermEvent, KeyEvent};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

use crate::app::AppResult;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Redraw,
    Key(KeyEvent),
    Resize(u16, u16),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    pub sender: mpsc::UnboundedSender<Event>,
    receiver: mpsc::UnboundedReceiver<Event>,
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let cloned_sender = sender.clone();

        let handler = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();

            loop {
                let crossterm_event = reader.next().fuse();

                tokio::select! {
                  _ = cloned_sender.closed() => {
                        break;
                  }

                  Some(Ok(evt)) = crossterm_event => {
                        match evt {
                            CrosstermEvent::Key(key) => {
                                if key.kind == crossterm::event::KeyEventKind::Press {
                                    cloned_sender.send(Event::Key(key)).unwrap();
                                }
                            },
                            CrosstermEvent::Resize(x, y) => {
                                cloned_sender.send(Event::Resize(x, y)).unwrap();
                            },
                            _ => {}
                        }
                    }
                };
            }
        });

        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub async fn next(&mut self) -> AppResult<Event> {
        self.receiver
            .recv()
            .await
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other, "This is an IO error",
            )))
    }
}