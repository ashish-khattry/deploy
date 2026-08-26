//04_tuple_destructor
fn main() {
    let tup: (i32, f64, u8) = (120_00_000, 34.7655, 244);
    let (x, y, z) = tup;
    println!("x={x} y={y} and z={z}");
    println!("Index 0={} 1={} and 2={}", tup.0, tup.1, tup.2);
}
