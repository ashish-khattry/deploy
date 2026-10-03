fn main() {
    let number = vec![1, 2, 3, 4];
    let mut iter_number = number.iter();
    println!("Vec num={:?}", iter_number.next());
    println!("Vec num={:?}", iter_number.next());
    println!("Vec num={:?}", iter_number.next());
    println!("Vec num={:?}", iter_number.next());
    println!("Vec num={:?}", iter_number.next());
}
