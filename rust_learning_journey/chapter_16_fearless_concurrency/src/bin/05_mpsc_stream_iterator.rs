use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel();
    let status = vec!["ok", "happy", "focus", "sad"];
    thread::spawn(move || {
        for msg in status {
            thread::sleep(Duration::from_millis(10));
            tx.send(msg).unwrap();
        }
    });
    for data in rx {
        println!("{data}");
    }
}
