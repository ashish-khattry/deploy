//04_banking_enum_method
#[derive(Debug)]
#[allow(dead_code)]
enum Transaction {
    Deposit(u32),
    Withdraw(u32),
}
impl Transaction {
    fn process(&self) {
        println!("Process trasaction");
    }
}
fn main() {
    let d = Transaction::Deposit(5_00_000);
    let w = Transaction::Withdraw(25_000);
    d.process();
    w.process();
    println!("Deposit ={:?} withdraw={:?}", d, w);
}
