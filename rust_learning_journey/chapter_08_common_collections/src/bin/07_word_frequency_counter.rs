//07_word_frequency_counter
use std::collections::HashMap;
fn main() {
    let para = "hellow word my name is remesos and king of egypton";
    let mut word_counter: HashMap<String, i32> = HashMap::new();
    for word in para.split_whitespace() {
        let lower_word = word.to_lowercase();
        let count = word_counter.entry(lower_word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_counter);
}
