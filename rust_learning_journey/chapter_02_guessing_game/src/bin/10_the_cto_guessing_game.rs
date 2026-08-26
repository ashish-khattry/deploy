//10_the_cto_guessing_game
use rand::RngExt;
use std::cmp::Ordering;
fn main() {
    let secret_number = rand::rng().random_range(1..100);
    println!("Welcome 🚀 to number guessing game 🎮 !");
    println!("Machine 📠 has a random number 🔢 from range 1 to 100! ");
    loop {
        println!("\nEnter Your Guess [1..=100]");
        let mut number = String::new();
        std::io::stdin()
            .read_line(&mut number)
            .expect("Input error!");
        let number: u32 = match number.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input 👎!");
                continue;
            }
        };
        match number.cmp(&secret_number) {
            Ordering::Less => {
                println!("\nThis is small 🤏🏻!");
            }
            Ordering::Greater => {
                println!("\nThis is big ⭕!");
            }
            Ordering::Equal => {
                println!("\n------------->\n🔥Perfect👍🏻 you WIN 💯!\n<-------------\n\n");
                break;
            }
        }
    }
}
