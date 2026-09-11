struct Point<T, U> {
    x: T,
    y: U,
}
fn main() {
    let p1 = Point { x: 100, y: 25.5 };
    let p2 = Point { x: 'x', y: "Edge" };

    println!(
        "Point 1 x={} y={}\nPoint 2 x={} y={}",
        p1.x, p1.y, p2.x, p2.y
    );
}
