//04_struct_privacy
mod ecommerce {
    pub struct UserAccount {
        pub username: String,
        credit_card: String,
    }
    impl UserAccount {
        pub fn new() -> Self {
            Self {
                username: String::new(),
                credit_card: String::new(),
            }
        }
    }
}
fn main() {
    let mut u = ecommerce::UserAccount::new();
    u.username = String::from("Ashish");
    println!("={:?}", u.username);
}
