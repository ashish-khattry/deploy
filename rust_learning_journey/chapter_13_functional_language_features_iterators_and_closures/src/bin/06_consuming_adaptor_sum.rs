fn main() {
    let number = vec![1, 2, 3, 4, 5];
    let total: i32 = number.iter().sum();
    println!("{:?}", total);
}
