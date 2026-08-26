//07_mutable_borrow_modifier
fn main() {
    let mut s1: String = String::from("Ashish");
    println!("Before add_surname function={s1}");
    add_surname(&mut s1);
    println!("After add_surname function={s1}");
}
fn add_surname(s1: &mut String) {
    s1.push_str(" Khattry ..");
}
