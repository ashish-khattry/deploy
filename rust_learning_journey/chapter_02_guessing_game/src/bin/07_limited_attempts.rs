//07_limited_attempts
fn main() {
    let mut count = 1;
    loop {
        println!("\nEnter {} input", count);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Input error");
        count += 1;
        let input = input.trim();
        println!("Input={}", input);
        if count == 6 {
            println!("\nBreaked!\n\n");
            break;
        }
    }
}
