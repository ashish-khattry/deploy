struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
    let p1 = Point { x: 100, y: 'X' };
    let p2 = Point { x: 200, y: 'U' };
    let p3 = p1.mixup(p2);
    println!("Point 3 x={},y={}", p3.x, p3.y);
}
