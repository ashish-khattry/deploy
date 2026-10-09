struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn hello(s: &str) {
    println!("Hello from inside custom box: {s}");
}
fn main() {
    let s = String::from("Rust Warrior");
    let m = MyBox::new(s);
    hello(&m);
}
