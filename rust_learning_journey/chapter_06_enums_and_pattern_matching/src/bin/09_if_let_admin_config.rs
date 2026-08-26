//09_if_let_admit_config
fn main() {
    let config = Some("Admin");
    if let Some(role) = config {
        println!("Access granted {:?}", role)
    }
}
