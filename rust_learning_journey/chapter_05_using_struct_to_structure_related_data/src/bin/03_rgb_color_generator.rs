//03_rgb_color_generator
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let red = Color(255, 0, 0);
    let p = Point(0, 1, 2);
    println!("Red color index 0 {} 1 {} and 2 {}", red.0, red.1, red.2);
    println!("Point index 0 {} 1 {} and 2 {}", p.0, p.1, p.2);
}
