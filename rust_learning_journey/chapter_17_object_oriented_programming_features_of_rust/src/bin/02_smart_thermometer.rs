pub struct Thermometer {
    temperature: i32,
}
impl Thermometer {
    pub fn set_value(&mut self, value: i32) {
        if value < 0 || value > 150 {
            println!("This {} temprature value is invalid in thermometer!", value);
        } else {
            self.temperature = value;
        }
    }
    pub fn new() -> Self {
        Self { temperature: 0 }
    }
}
fn main() {
    let mut t1 = Thermometer::new();
    t1.set_value(60);
    t1.set_value(90);
    t1.set_value(190);
    t1.set_value(-12);
}
