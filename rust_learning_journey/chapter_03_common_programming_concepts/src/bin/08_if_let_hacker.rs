//08_if_let_hacker
fn main() {
    let is_cto_ready = true;
    let target_salary = if is_cto_ready { 3_00_000 } else { 50_000 };
    println!("Final CTO salary 💵={target_salary}");
}
