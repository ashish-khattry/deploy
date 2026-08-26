//09_extreme_chaining
fn read_my_file() -> Result<String, std::io::Error> {
    std::fs::read_to_string("data.txt")
}
fn main() {
    let _file = read_my_file();
}
