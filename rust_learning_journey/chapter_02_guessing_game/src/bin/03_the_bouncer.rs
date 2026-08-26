//03_the_bouncer
use std::cmp::Ordering;
fn main() {
    println!("Enter your age");
    let mut age = String::new();
    std::io::stdin().read_line(&mut age).expect("Input error");
    let age: u32 = match age.trim().parse::<u32>() {
        Ok(age) => age,
        Err(_) => {
            println!("Invalid age!");
            return;
        }
    };
    match &age.cmp(&18) {
        Ordering::Less => {
            println!("You can not come inside becouse you are below 18 years of age!");
        }
        Ordering::Greater => {
            println!("Welcome please come inside!");
        }
        Ordering::Equal => {
            println!("Welcome please come inside!")
        }
    }
}
