use colored::Colorize;
use rand::prelude::*;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!(
        "\n                 [ {}   ]",
        "Welcome To CLI Guessing Game ".green().bold()
    );
    println!(
        "                 [ {}                  ]",
        "Select a Level".red().bold()
    );
    println!(
        "                 [ {}   ]",
        "Normal (1-50)       Press: 50".green().bold()
    );
    println!(
        "                 [ {}  ]",
        "Difficult (1-100)   Press: 100".green().bold()
    );
    println!(
        "                 [ {} ]",
        "Super (1-1000)      Press: 1000".red().bold()
    );
    let mut range = String::new();
    io::stdin().read_line(&mut range).expect("error");
    let range: usize = match range.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("\n               {}", "Error: Invalid Range !".red().bold());
            return;
        }
    };
    match range {
        50 => {
            get_guess(50);
        }
        100 => {
            get_guess(100);
        }
        1000 => {
            get_guess(1000);
        }
        _ => {
            println!(
                "\n                  {}",
                "Error: Invalid Range !".red().bold()
            );
        }
    }
}
fn get_guess(range: i32) {
    println!(
        "{}",
        format!(
            "                 [ Enter The Number From 1 To {}   ]",
            range
        )
        .red()
        .bold()
    );
    let machine_no = rand::rng().random_range(1..=range);
    loop {
        println!(
            "\n                 [ {}   ]",
            format!("Enter Guessing Number 1 To {}", range)
                .green()
                .bold()
        );
        let mut user_no = String::new();
        io::stdin().read_line(&mut user_no).unwrap();
        let user_no: i32 = match user_no.trim().parse() {
            Ok(num) => {
                if num >= 1 && num <= range {
                    num
                } else {
                    println!("\n{}",format!("                  Error: Invalid Number, Enter Only A Number From 1 to {}",range).red().bold());
                    continue;
                }
            }
            Err(_) => {
                println!("\nInvalid data");
                continue;
            }
        };
        match user_no.cmp(&machine_no) {
            Ordering::Less => {
                println!("\n                  {}", "Ohh This Is Small!".red().bold());
            }
            Ordering::Greater => {
                println!("\n                  {}", "Ahhh This Is Big!".red().bold());
            }
            Ordering::Equal => {
                println!("\n                  {}","Perfect! You Got It This Is Right Ones!!!\n                  Game Is Closing Visit Again....\n\n\n\n".green().bold());
                break;
            }
        }
    }
}
