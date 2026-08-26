fn main() {
    let s: String = String::from("Education makes you pefect");
    let slice = first_word(&s);
    println!("Full string ={s}\nSlice of this string ={slice}");
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
