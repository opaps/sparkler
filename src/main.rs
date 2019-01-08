extern crate git2;
extern crate notify;

use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() { 
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    let path = "../tmp";

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    println!("watching {}", path);

    loop {
        match rx.recv() {
            Ok(event) => {
                println!("{:?}", event);
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}