//09_infinite_counter
fn main() {
    let mut counter = 0;
    let counter_value = loop {
        counter += 1;
        if counter == 15 {
            break counter;
        }
    };
    println!("Counter value 🔄️={counter_value}");
}
