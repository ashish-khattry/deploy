use colored::Colorize;
#[allow(dead_code)]
use std::io;
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}
struct Calculator {
    num1: f64,
    num2: f64,
    op: Operation,
}
fn calculate(c: Calculator) -> f64 {
    match c.op {
        Operation::Add => c.num1 + c.num2,
        Operation::Sub => c.num1 - c.num2,
        Operation::Mul => c.num1 * c.num2,
        Operation::Div => {
            if c.num2 == 0.0 {
                println!("\n{}", "Error: Second Number is 0".red().bold());
                return 0.0;
            } else {
                c.num1 / c.num2
            }
        }
        Operation::Rem => {
            if c.num2 == 0.0 {
                println!("\n{}", "Error: Second Number is 0".red().bold());
                return 0.0;
            } else {
                c.num1 % c.num2
            }
        }
    }
}
fn main() {
    println!(
        "\n               [ {} ]",
        "🚀 Welcome To CLI Calculator 🧮 ".green().bold()
    );
    println!(
        "\n                              [ {} ]",
        "Enter 1️⃣  Number".green().bold()
    );
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let number1: f64 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("\n{}", "Error: Invalid First Number 👎🏻".red().bold());
            return;
        }
    };
    println!(
        "\n                              [ {}]",
        "Enter 2️⃣  Number ".green().bold()
    );
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let number2: f64 = match b.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("\n{}", "Error: Invalid Second Number 👎🏻".red().bold());
            return;
        }
    };
    println!(
        "\n                                 [ {} ]",
        "OPERATION".green().bold()
    );
    println!(
        "\n                               [ {} ]",
        "[➕] Add (➕)".red().bold()
    );
    println!(
        "\n                               [ {} ]",
        "[➖] Sub (➖)".red().bold()
    );
    println!(
        "\n                               [ {} ]",
        "[✖️ ] Mul (✖️ )".red().bold()
    );
    println!(
        "\n                               [ {} ]",
        "[➗] Div (➗)".red().bold()
    );
    println!(
        "\n                               [ {}]",
        "[％] Rema (％)".red().bold()
    );
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let string1 = choice.trim();
    let operation = match string1 {
        "Add" | "+" => Operation::Add,
        "Sub" | "-" => Operation::Sub,
        "Mul" | "*" => Operation::Mul,
        "Div" | "/" => Operation::Div,
        "Rem" | "%" => Operation::Rem,
        _ => {
            println!("\n{}", "Error: Invalid OPERATION 👎🏻".red().bold());
            return;
        }
    };
    let c1 = Calculator {
        num1: number1,
        num2: number2,
        op: operation,
    };
    let result = calculate(c1);
    let result: String = result.to_string();
    println!(
        "\n                               [    🟰  {}      ]",
        result.green().bold()
    );
}
