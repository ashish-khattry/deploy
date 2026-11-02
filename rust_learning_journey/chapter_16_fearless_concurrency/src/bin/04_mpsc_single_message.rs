use std::sync::mpsc;
use std::thread;
fn main() {
    let (tx, rx) = mpsc::channel();
    let msg = String::from("Java vs Rust");
    let handle = thread::spawn(move || {
        tx.send(msg).unwrap();
    });
    handle.join().unwrap();
    match rx.recv() {
        Ok(msg) => {
            println!("{msg}");
        }
        Err(_) => {}
    }
}
