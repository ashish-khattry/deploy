//07_the_access_gateway
fn main() {
    println!("Enter server ping number");
    let mut ping = String::new();
    std::io::stdin().read_line(&mut ping).expect("Input error!");
    let ping: u32 = match ping.trim().parse::<u32>() {
        Ok(value) => value,

        Err(_) => {
            println!("Invalid input 👎!");
            return;
        }
    };
    if ping < 50 {
        println!("Excellent 👍!");
    } else if ping >= 50 && ping <= 100 {
        println!("Good 🫡!");
    } else {
        println!("Danger 💀!")
    }
}
