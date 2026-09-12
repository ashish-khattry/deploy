struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    let novel = String::from("Rust is hard, But ashish is CTO.");
    let first_sentense = novel.split(",").next().unwrap();
    let i = ImportantExcerpt {
        part: &first_sentense,
    };
    println!("Excerpt={}", i.part);
}
