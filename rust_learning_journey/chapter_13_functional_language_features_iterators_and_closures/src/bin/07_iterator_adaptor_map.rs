fn main() {
    let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{:?}", vec1);
    let vec2: Vec<_> = vec1.iter().map(|x| x * 10).collect();
    println!("{:?}", vec2);
}
