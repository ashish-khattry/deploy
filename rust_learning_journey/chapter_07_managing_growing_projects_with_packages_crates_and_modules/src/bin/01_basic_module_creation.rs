//01_basic_module_creation
#[allow(dead_code)]
mod authentication {
    pub fn login() {
        println!("Login successfull");
    }
    fn verify_password() {
        println!("Password is hidden");
    }
}
fn main() {
    authentication::login();
}
