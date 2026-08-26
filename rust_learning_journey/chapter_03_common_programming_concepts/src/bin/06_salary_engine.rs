//06_salary_engine
fn main() {
    company_message();
    let final_salary = get_salary(25_00_000, 5_00_000);
    println!("CTO salary is 💰={final_salary}");
}
fn company_message() {
    println!("This is Google MNC USA 🎯");
}
fn get_salary(base_pay: u32, bonus: u32) -> u32 {
    base_pay + bonus
}
