//08_number_analyzer
use rand::RngExt;
use std::cmp::Ordering;
fn main() {
    let secret_number = rand::rng().random_range(1..=10);
    loop {
        println!("\nMachine has a ramdom number from 1 to 10");
        println!("Enter your number");
        let mut number = String::new();
        std::io::stdin()
            .read_line(&mut number)
            .expect("Input error!");
        let number: u32 = match number.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                continue;
            }
        };
        match number.cmp(&secret_number) {
            Ordering::Less => {
                println!("This is small!");
            }
            Ordering::Greater => {
                println!("This is big!");
            }
            Ordering::Equal => {
                println!("Perfect this is same!");
                break;
            }
        }
    }
}
