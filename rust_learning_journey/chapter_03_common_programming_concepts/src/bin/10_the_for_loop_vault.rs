//10_the_for_loop_vault
fn main() {
    let ports: [i32; 5] = [80, 443, 3306, 8080, 53];
    for elements in ports {
        println!("➡️ :{elements}");
    }
    for numbers in (1..=5).rev() {
        println!("🔢 ={numbers}");
    }
}
