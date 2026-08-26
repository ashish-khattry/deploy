use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let _file = match File::open("secret.txt") {
        Ok(file) => file,
        Err(file_error) => match file_error.kind() {
            ErrorKind::NotFound => match File::create("secret.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    println!("File creation failed: {:?}", e);
                    panic!("Crashing program!");
                }
            },

            other_error => {
                println!("Other error: {:?}", other_error);
                panic!("Crashing program!");
            }
        },
    };
}
