use std::io;

fn main() {
    let mut password_list: Vec<String> = Vec::new();
    let mut id_list: Vec<String> = Vec::new();

    loop {
        println!("\n1. Add Password");
        println!("2. Show Passwords");
        println!("3. Delete Password");
        println!("4. Update Password");
        println!("5. Search Password");
        println!("6. Total Passwords");
        println!("7. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Choice!");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter ID:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                id_list.push(id.trim().to_string());

                println!("Enter Password:");
                let mut password = String::new();
                io::stdin().read_line(&mut password).unwrap();
                password_list.push(password.trim().to_string());

                println!("Password Saved Successfully!");
            }

            2 => {
                if id_list.is_empty() {
                    println!("No Password Saved!");
                    continue;
                }

                println!("\nAll Passwords:\n");

                for (i, id) in id_list.iter().enumerate() {
                    println!("{}. {} -> {}", i + 1, id, password_list[i]);
                }
            }

            3 => {
                println!("Enter ID:");

                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();

                let id = id.trim();

                match id_list.iter().position(|x| x == id) {
                    Some(index) => {
                        id_list.remove(index);
                        password_list.remove(index);
                        println!("Password Deleted!");
                    }
                    None => {
                        println!("ID Not Found!");
                    }
                }
            }

            4 => {
                println!("Enter ID:");

                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();

                let id = id.trim();

                match id_list.iter().position(|x| x == id) {
                    Some(index) => {
                        println!("Old Password = {}", password_list[index]);

                        println!("Enter New Password:");

                        let mut new_pass = String::new();
                        io::stdin().read_line(&mut new_pass).unwrap();

                        password_list[index] = new_pass.trim().to_string();

                        println!("Password Updated!");
                    }

                    None => {
                        println!("ID Not Found!");
                    }
                }
            }

            5 => {
                println!("Enter ID:");

                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();

                let id = id.trim();

                match id_list.iter().position(|x| x == id) {
                    Some(index) => {
                        println!("Password = {}", password_list[index]);
                    }

                    None => {
                        println!("ID Not Found!");
                    }
                }
            }

            6 => {
                println!("Total Passwords = {}", password_list.len());
            }

            7 => {
                println!("Thank You For Using This App!");
                break;
            }

            _ => {
                println!("Invalid Choice!");
            }
        }
    }
}
