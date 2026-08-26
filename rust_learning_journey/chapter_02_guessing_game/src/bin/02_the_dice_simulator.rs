//02_the_dice_simulator
use rand::RngExt;
fn main() {
    let first = rand::rng().random_range(1..=6);
    let second = rand::rng().random_range(1..=6);
    println!("First ramdom number ={}", first);
    println!("Second ramdom number ={}", second);
    println!("Addition of both number={}", first + second);
}
