//05_array_vault
fn main() {
    let salary: [u32; 5] = [25_00_000, 30_00_000, 45_00_000, 60_00_000, 120_000_000];
    println!("Enter any index number from 0 to 4 to know the salary package 🤑");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Input error!");
    let input: usize = match input.trim().parse::<usize>() {
        Ok(value) if value <= 4 => value,
        _ => {
            println!("Friend this index number is invalid 😒");
            return;
        }
    };
    println!("Salary package is {} INR", salary[input]);
}
