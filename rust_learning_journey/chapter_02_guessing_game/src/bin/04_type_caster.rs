//04_type_caster
fn main() {
    println!("Enter your salary package");
    let mut salary = String::new();
    std::io::stdin()
        .read_line(&mut salary)
        .expect("Input error!");
    let salary: u32 = match salary.trim().parse::<u32>() {
        Ok(salary) => salary,
        Err(_) => {
            println!("Invalid!");
            return;
        }
    };
    println!("Package {} LPA", salary);
}
