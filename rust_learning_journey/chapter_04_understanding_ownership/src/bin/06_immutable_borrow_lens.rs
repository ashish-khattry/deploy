//06_immutable_borrow_lens
fn main() {
    let s1: String = String::from("Rust development architect!");
    let i1: usize = calculate_len(&s1);
    println!("Main string is ={s1} and its length is ={i1}");
}
fn calculate_len(s1: &String) -> usize {
    s1.len()
}
