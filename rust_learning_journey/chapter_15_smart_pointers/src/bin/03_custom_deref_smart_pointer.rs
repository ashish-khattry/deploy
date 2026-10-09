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
fn main() {
    let x = 10;
    let y = MyBox::new(x);
    assert_eq!(*y, 100, "Custom deref operator does not work successfully!");
    println!("Custom deref operator works successfully!");
}
