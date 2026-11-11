trait Shape {
    fn area(&self) -> f64;
}
struct Circle {
    radius: f64,
}
struct Square {
    side: f64,
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius.powf(2.0) * std::f64::consts::PI
    }
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 420.15 }),
        Box::new(Square { side: 14.55 }),
    ];
    for shape in shapes.iter() {
        let calculated_area = shape.area();
        println!("Area is ={}", calculated_area);
    }
}
