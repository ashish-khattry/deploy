use std::thread;
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let handle = thread::spawn(move || {
        println!("Vector is {:?}", v);
    });
    match handle.join() {
        Ok(()) => {}
        Err(_) => {}
    }
}
