pub struct PiggyBank {
    balance: u32,
}
impl PiggyBank {
    pub fn new() -> Self {
        Self { balance: 0 }
    }
    pub fn add_money(&mut self, add_data: u32) {
        self.balance = add_data.saturating_add(self.balance);
    }
    pub fn check_balance(&self) {
        println!("\nBalance {}", self.balance);
    }
}
fn main() {
    let mut p = PiggyBank::new();
    p.add_money(5_000_0000);
    p.check_balance();
}
