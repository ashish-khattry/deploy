//03_basic_result_match
fn main() {
    let is_file = std::fs::File::open("ghost");
    let file = match is_file {
        Ok(file) => file,
        Err(_) => {
            println!("File is not exit in this computer!");
            return;
        }
    };
    println!("{:?}", file);
}
