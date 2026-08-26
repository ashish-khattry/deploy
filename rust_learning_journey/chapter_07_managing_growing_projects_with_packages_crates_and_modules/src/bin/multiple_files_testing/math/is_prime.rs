pub fn is_prime_number(n: i32) {
    let mut is_p = false;
    for i in 2..n {
        if n % i == 0 {
            is_p = true;
            break;
        }
    }
    if is_p == true {
        println!("{} is not a prime number", n);
    } else {
        println!("{}  is a prime number", n);
    }
}
