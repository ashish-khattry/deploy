pub struct Vault {
    secret_password: String,
}
impl Vault {
    fn new() -> Self {
        Self {
            secret_password: String::from("Admin@1999"),
        }
    }
    fn verify(&self, input: &str) -> bool {
        if self.secret_password == input.to_string() {
            true
        } else {
            false
        }
    }
}
fn main() {
    let v = Vault::new();
    let is_matched = v.verify("admin@123");
    println!("Does password match : {}", is_matched);
}
