//06_the_shield
fn main() {
    loop {
        println!("\n\n-->>>This is a infinit loop that takes only numerical values!");
        println!("Enter a numerical value!");
        let mut value = String::new();
        std::io::stdin()
            .read_line(&mut value)
            .expect("Input error!");
        let value: i32 = match value.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: this is not a numerical value !");
                continue;
            }
        };
        println!("Numerical value={}", value);
    }
}
