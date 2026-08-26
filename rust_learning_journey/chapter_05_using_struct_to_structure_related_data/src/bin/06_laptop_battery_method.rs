//06_laptop_battery_method
struct Laptop {
    brand: String,
    battery: u32,
}
impl Laptop {
    fn show_battery(&self) {
        println!("Brand {} has {}% battery", self.brand, self.battery);
    }
}
fn main() {
    let l = Laptop {
        brand: String::from("Acer"),
        battery: 75,
    };
    l.show_battery();
}
