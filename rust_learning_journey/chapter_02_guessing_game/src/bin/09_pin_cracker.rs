//09_pin_cracker
use rand::RngExt;
fn main() {
    let pin = rand::rng().random_range(1000..=9999);
    loop {
        println!("Enter the PIN between 1000 to 9999");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Input error!");
        let input: u32 = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };
        if input == pin {
            println!("PIN matched loop breaked!");
            break;
        }
    }
}
