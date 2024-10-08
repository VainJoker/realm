use std::{
    fs,
    path::{self, Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossbeam::channel::{Receiver, Sender, bounded};

use crate::{Realme, RealmeError, RealmeResult};

// /// A watcher for the `Realme` configuration.
// ///
// /// This struct provides a mechanism to watch for changes to the `Realme`
// /// configuration and reload it if necessary. It uses a channel to
// communicate /// with the main thread and a thread to watch for changes.
// pub struct RealmeWatcher {
//     /// The channel to communicate with the main thread.
//     sender: Sender<()>,
//     /// The thread to watch for changes.
//     thread: thread::JoinHandle<()>,
// }

// impl RealmeWatcher {}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Changed,
    Stopped,
}

pub type Channel = (Sender<Event>, Receiver<Event>);

// pub fn watch(
//     tx: Sender<Event>,
//     rx: Receiver<Event>,
//     interval: Duration,
//     path: PathBuf,
// ) {
//     thread::spawn(move || {
//         loop {
//             thread::sleep(interval);

//             match rx.recv() {
//                 Ok(Event::Stopped) => break,
//                 Ok(e) => {
//                     RealmeError::Unknown(format!("{:?}", e));
//                     break;
//                 }
//                 Err(e) => {
//                     RealmeError::Unknown(format!("{:?}", e));
//                     break;
//                 }
//             }
//         }
//     });

//     thread::spawn(move || {
//         let mut last_modified =
//             fs::metadata(&path).unwrap().modified().unwrap();
//         loop {
//             thread::sleep(interval);
//             let modified = fs::metadata(&path).unwrap().modified().unwrap();
//             if modified != last_modified {
//                 last_modified = modified;
//                 tx.send(Event::Changed).unwrap();
//             }
//         }
//     });
// }
