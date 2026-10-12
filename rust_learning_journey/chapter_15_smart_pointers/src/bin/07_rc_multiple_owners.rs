use std::rc::Rc;
fn main() {
    let string = String::from("RUST IS MY LIFE");
    let r1 = Rc::new(string);
    print!("Count: {} -> ", Rc::strong_count(&r1));
    let r2 = Rc::clone(&r1);
    print!("Count: {} -> ", Rc::strong_count(&r2));
    {
        let r3 = Rc::clone(&r1);
        print!("Inner Count: {} -> ", Rc::strong_count(&r3));
        print!("Inner ends,");
    }
    print!("Count: {} .", Rc::strong_count(&r2));
}
