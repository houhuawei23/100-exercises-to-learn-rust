use std::thread::spawn;
use std::sync::Mutex;
use std::sync::mpsc::sync_channel;

fn main() {
    let lock = Mutex::new(0);
    let (sender, receiver) = sync_channel(1);
    let guard = lock.lock().unwrap();

    spawn(move || {
        receiver.recv().unwrap();
    });

    // Try to send the guard over the channel
    // to another thread
    sender.send(guard);
}