fn point<T, U>(x: T, y: U)
where
    T: std::fmt::Display + Clone,
    U: std::fmt::Display + Clone,
{
    println!("Point x={} y={}", x, y);
}
fn main() {
    point(100, 200.50);
    point('X', "North");
}
