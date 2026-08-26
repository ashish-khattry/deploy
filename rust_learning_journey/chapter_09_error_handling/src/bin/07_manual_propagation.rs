//07_manual_propagation
use std::fs::File;
use std::io::Error;
fn read_config() -> Result<File, Error> {
    let file = match File::open("my_file.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    Ok(file)
}

fn main() {
    let file_result = read_config();
    println!("File result is = {:?}", file_result);
}
