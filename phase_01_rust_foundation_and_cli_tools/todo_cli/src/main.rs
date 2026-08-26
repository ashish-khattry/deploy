use colored::Colorize;
use std::io;
fn main() {
    let mut to_do: Vec<String> = Vec::new();
    loop {
        println!("\n[  {}                    ]\n[  {}                           ]\n[  {}               ]\n[  {}      ]\n[  {}                  ]\n","Enter Down Given Task Numbers For ".green().bold(),"Press:-  1 For Add Any Task".green().bold(),"Press:-  2 For See Your Tasks In A List".green().bold(),"Press:-  3 For Delete Any Task That Dont Require".red().bold(),"Press:-  4 For Quick The APPLICATION".red().bold());
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        match choice {
            "1" => {
                println!("\n{}", "Type Complete All Task Summary ".green().bold());
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                to_do.push(task.trim().to_string());
                println!(
                    "\n{}",
                    "This Task Has Been Added!\nPress 2 For See.."
                        .yellow()
                        .bold()
                );
            }
            "2" => {
                println!(
                    "\n\n{}",
                    "All Listed Tasks Are As Below :---".green().bold()
                );
                for (no, task1) in to_do.iter().enumerate() {
                    println!("\n{}.{}", no + 1, task1.green());
                }
            }
            "3" => {
                println!(
                    "\n{}",
                    "Enter the Task Number For Deletion? . \n\n".cyan().bold()
                );
                let mut delete = String::new();
                io::stdin().read_line(&mut delete).unwrap();
                let delete: usize = match delete.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("{}", "\nError:404( Wrong Input ) ".red().bold());
                        continue;
                    }
                };
                if delete > 0 && delete <= to_do.len() {
                    let data1 = to_do.remove(delete - 1);
                    println!(
                        "\nThis Task -> [ {} ] \nTask Have Been Deleted! \n\n\n",
                        data1.blue().bold()
                    );
                } else {
                    println!("\n{}", "Error:404 ( Wrong Input ) ".red().bold());
                }
            }
            "4" => {
                println!(
                    "\n{}",
                    "App is closing  Thank You ..Visit Agian........\n"
                        .green()
                        .blink()
                );
                break;
            }
            _ => {
                println!("\n{}", "Error:404 ( Wrong Input ) \n\n".red().bold());
            }
        }
    }
}
