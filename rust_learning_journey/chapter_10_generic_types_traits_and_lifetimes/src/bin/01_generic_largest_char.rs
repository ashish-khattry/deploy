fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut large = list[0];

    for i in 1..list.len() {
        if large < list[i] {
            large = list[i];
        }
    }
    large
}
fn main() {
    let l1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let large = largest(&l1);
    println!("Largest={large}")
}
