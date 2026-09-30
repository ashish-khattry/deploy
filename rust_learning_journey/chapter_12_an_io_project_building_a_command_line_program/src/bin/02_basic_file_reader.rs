fn main() {
    let content =
        std::fs::read_to_string("data.txt").expect("Error while reading the data.txt file!");
    println!("File text:\n{content}");
}
