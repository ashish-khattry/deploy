use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(300));
        tx.send("Completion").unwrap();
    });
    loop {
        match rx.try_recv() {
            Ok(data) => {
                println!("{data}");
            }
            Err(mpsc::TryRecvError::Empty) => {
                println!("Main thread doing work while waiting..");
                thread::sleep(Duration::from_millis(300));
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                break;
            }
        }
    }
}
