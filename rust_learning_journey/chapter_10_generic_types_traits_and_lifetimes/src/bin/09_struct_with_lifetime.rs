struct ImportantExcerpt<'a> {
    part: &'a String,
}
fn main() {
    let novel = String::from("Rust is hard, But ashish is CTO.");
    let first_sentense = novel.split(",").next().unwrap();
    let first_sentense = &first_sentense.to_string();
    let i = ImportantExcerpt {
        part: &first_sentense,
    };
    println!("Excerpt={}", i.part);
}
