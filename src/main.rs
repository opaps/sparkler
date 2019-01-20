extern crate git2;
extern crate notify;
extern crate sparkler;

use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

use sparkler::repo;

fn main() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    let path = "../tmp";

    let the_repo = repo::Repo::new(path);

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
                use notify::DebouncedEvent::*;
                match event {
                    Create(path) | Write(path) | Chmod(path) | Remove(path) | Rename(_, path) => {
                        println!("event: {:?}", path);
                        // exclude .git
                        if !path.iter().any(|p| p.to_str().unwrap() == ".git") {
                            let _ = the_repo.get_status();
                        }
                    },
                    _ => {}

                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
