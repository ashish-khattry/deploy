use std::ops::Drop;
struct MyStruct {
    x: String,
}
impl Drop for MyStruct {
    fn drop(&mut self) {
        print!("Dropping {}! ", self.x);
    }
}
fn main() {
    let a = MyStruct {
        x: String::from("A"),
    };
    print!("{} created. ", a.x);
    let b = MyStruct {
        x: String::from("B"),
    };
    print!("{} created. ", b.x);
}
