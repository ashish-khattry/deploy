//01_the_manual_panic
fn self_test(input: String) {
    if input.to_lowercase() == "yes" {
        panic!("\nCTO Initiated Self-Destruct\n")
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter some input ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();
    self_test(input);
    Ok(())
}
