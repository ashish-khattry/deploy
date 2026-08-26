pub fn calculate_table(n: i32) {
    println!("Calculated table of {}", n);
    for i in 1..=10 {
        println!("{} * {} = {}", n, i, i * n);
    }
}
