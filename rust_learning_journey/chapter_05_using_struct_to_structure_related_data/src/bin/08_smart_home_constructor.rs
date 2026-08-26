//08_smart_home_constructor
#[derive(Debug)]
#[allow(dead_code)]
struct SmartBulb {
    color: String,
    brightness: u32,
}
impl SmartBulb {
    fn new() -> Self {
        Self {
            color: String::from("Warm white"),
            brightness: 25,
        }
    }
}
fn main() {
    let b = SmartBulb::new();
    println!("{:?}", b);
}
