trait Vehicle {
    fn drive(&self);
}
struct Car {}
struct Bike {}
impl Vehicle for Car {
    fn drive(&self) {
        println!("Car is has been drived!");
    }
}
impl Vehicle for Bike {
    fn drive(&self) {
        println!("Bike has been drived!");
    }
}
fn main() {
    let vehicles: Vec<Box<dyn Vehicle>> = vec![Box::new(Car {}), Box::new(Bike {})];
    for vehicle in vehicles.iter() {
        vehicle.drive();
    }
}
