//10_mega_bank_architecture
struct BankAccount {
    name: String,
    amount: u32,
}
impl BankAccount {
    fn open_account(name: String) -> Self {
        Self { name, amount: 0 }
    }
    fn deposit(&mut self, new_balance: u32) {
        self.amount = new_balance
    }
    fn check_balance(&self) {
        println!("Balance in {} account  is ={}", self.name, self.amount)
    }
}
fn main() {
    let mut my_account = BankAccount::open_account("Ashish".to_string());
    my_account.deposit(15_00_000);
    my_account.check_balance();
    let wife_account = BankAccount {
        name: String::from("Seeta"),
        ..my_account
    };
    wife_account.check_balance();
}
