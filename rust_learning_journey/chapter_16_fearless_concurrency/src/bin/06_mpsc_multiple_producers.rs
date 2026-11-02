use std::sync::mpsc;
use std::thread;
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let v1 = vec!["hii", "welcome", "in rust"];
    let v2 = vec!["keep", "coding", "everyday"];
    thread::spawn(move || {
        for data in v1 {
            tx1.send(data).unwrap();
        }
    });
    thread::spawn(move || {
        for data in v2 {
            tx.send(data).unwrap();
        }
    });
    for data in rx {
        print!("{data} ");
    }
}
