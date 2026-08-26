//01_basic_traffic_light
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;
    println!("Traffic light are ={:?} {:?} {:?}", red, yellow, green);
}
