fn largest<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a > b {
        a
    } else {
        b
    }
}
fn main() {
    let a = String::from("Rust");
    let large: &String;
    {
        let b = String::from("Java");
        large = largest(&a, &b);
        println!("Largest={large}");
    }
}
