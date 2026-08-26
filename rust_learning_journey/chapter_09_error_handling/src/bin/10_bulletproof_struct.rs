//10_bulletproof_struct
#[derive(Debug)]
pub struct Age {
    value: i32,
}
impl Age {
    fn new(value: i32) -> Age {
        if value > 0 && value < 150 {
            Age { value }
        } else {
            panic!("Invalid age!");
        }
    }
}
fn main() {
    let my_age = Age::new(25);
    println!("My age is ={:?}", my_age);
}
