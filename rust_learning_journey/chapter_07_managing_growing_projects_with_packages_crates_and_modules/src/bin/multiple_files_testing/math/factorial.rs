pub fn calculate_factorial(n: i32) {
    let mut fact = 1;
    for i in 1..=n {
        fact = fact * i;
    }
    println!("Factorial of {} is ={}", n, fact);
}
