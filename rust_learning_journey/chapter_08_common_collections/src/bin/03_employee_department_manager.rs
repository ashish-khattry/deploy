//03_employee_department_manager
use std::collections::HashMap;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter command like \"Add Ashish to Engineering\"");
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Enter command ");
        let mut command = String::new();
        std::io::stdin().read_line(&mut command)?;
        let command = command.trim();
        if command.to_lowercase() == "exit" {
            println!("Terminal closed");
            break;
        }
        let parts: Vec<&str> = command.split_whitespace().into_iter().collect();
        if parts.len() != 4 || parts[0].to_lowercase() != "add" || parts[2].to_lowercase() != "to" {
            println!("Invalid command");
            continue;
        }
        let name = parts[1].to_string();
        let dep = parts[3].to_string();
        let employee_list = company.entry(dep.clone()).or_insert(Vec::new());
        if employee_list.contains(&name) {
            println!(
                "Duplicate alert {} is already exit in {} department",
                name, dep
            );
        } else {
            employee_list.push(name.clone());
            println!("{} has been added to {} department", name, dep);
        }
    }
    Ok(())
}
