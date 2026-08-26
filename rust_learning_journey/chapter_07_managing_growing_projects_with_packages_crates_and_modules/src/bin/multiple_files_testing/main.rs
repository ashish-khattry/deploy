mod math;
use std::io::Write;
fn main() {
    println!("Enter a number :-  ");
    std::io::stdout().flush().expect("Flushing error!");
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Input error!");
    let n: i32 = match n.trim().parse::<i32>() {
        Ok(number) => number,
        Err(_) => {
            println!("This is invalid value !");
            return;
        }
    };
    println!("After is_prime_number function calling");
    math::is_prime::is_prime_number(n);
    println!("After find_even_odd function calling");
    math::even_odd::find_even_odd(n);
    println!("After calculate_factorial function calling");
    math::factorial::calculate_factorial(n);
    println!("After calculate_table function calling");
    math::table::calculate_table(n);
}
