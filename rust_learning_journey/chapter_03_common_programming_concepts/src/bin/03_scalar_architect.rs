//03_scalar_architect
fn main() {
    let age: u8 = 26;
    let salary: f64 = 25_00_000.78_000;
    let is_active: bool = true;
    let grade: char = 'A';
    println!("CTO detail age={age} salary={salary} is active={is_active} and grade={grade}");
    let a: f64 = 10.0;
    let b: f64 = 20.0;
    let c: f64 = a + b;
    println!("Addition={c}");
    let c: f64 = a - b;
    println!("Substraction={c}");
    let c: f64 = a * b;
    println!("Multiplication={c}");
    let c: f64 = a / b;
    println!("Division={c}");
    let c: f64 = a % b;
    println!("Remainder={c}");
}
