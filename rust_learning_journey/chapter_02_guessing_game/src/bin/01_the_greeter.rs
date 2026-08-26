//01_the_greeter
use std::io;
use std::io::Write;
fn main() {
    print!("Please enter you name :");
    io::stdout().flush().expect("Flushing error!");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Input error!");
    print!("Enter your age :");
    io::stdout().flush().expect("Flushing error!");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Input error!");
    println!("Welcome {} your age is {}", name, age);
}
