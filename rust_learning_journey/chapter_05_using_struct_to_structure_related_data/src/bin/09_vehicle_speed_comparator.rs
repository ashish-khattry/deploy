//09_vehicle_speed_comparator
struct Car {
    model: String,
    top_speed: u32,
}
impl Car {
    fn is_faster(&self, c2: &Car) -> bool {
        self.top_speed > c2.top_speed
    }
}
fn main() {
    let c1 = Car {
        model: String::from("Safary"),
        top_speed: 255,
    };
    let c2 = Car {
        model: String::from("Sadan"),
        top_speed: 300,
    };
    println!("Car 1 detail model={} speed={}", c1.model, c1.top_speed);
    println!("Car 2 detail model={} speed={}", c2.model, c2.top_speed);
    let mut fast = String::new();
    fast = if c1.is_faster(&c2) {
        "Car 1".to_string()
    } else {
        "Car2".to_string()
    };
    println!("Which car is faster={}", fast);
}
