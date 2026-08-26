//03_anonymous_struct_message
#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Write(String),
    Move { x: i32, y: i32 },
}
fn main() {
    let m1 = Message::Write(String::from("Hi buddy i'm vadanta!"));
    let m2 = Message::Move { x: 277, y: 315 };
    println!("m1={:?} and m2={:#?}", m1, m2);
}
