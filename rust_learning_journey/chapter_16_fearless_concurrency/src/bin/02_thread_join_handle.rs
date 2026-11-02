use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            thread::sleep(Duration::from_millis(1));
            println!("Spawn thread {i}");
        }
    });
    for i in 1..=5 {
        thread::sleep(Duration::from_millis(1));
        println!("Main thread {i}");
    }
    handle.join().unwrap();
}
