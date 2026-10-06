fn main() {
    let mut sum: u64 = 0;
    let mut count: u64 = 0;
    let start_time = std::time::Instant::now();
    for i in 1..=1_00_000 {
        sum = sum+1;
        count = i;
    }
    let duration = start_time.elapsed();
    println!("Count={count} and Sum={sum}");
    println!("Time taken={:?}", duration);
}
