fn large_vec(vec: &[i32], large: i32) -> Vec<&i32> {
    vec.into_iter().filter(|x| *x >= &large).collect()
}
fn main() {
    let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let vec2 = large_vec(&vec1, 4);
    println!("{:?}", vec2);
}
