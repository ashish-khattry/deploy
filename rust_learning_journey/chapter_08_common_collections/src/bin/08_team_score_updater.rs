//08_team_score_updater
use std::collections::HashMap;
fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("ashish"), 65);
    scores.insert(String::from("ashish"), 99);
    scores.entry(String::from("ashish")).or_insert(100);
    scores.entry(String::from("vijay")).or_insert(55);
    scores.entry(String::from("vinay")).or_insert(74);
    for (key, value) in scores {
        println!("{} {}", key, value);
    }
}
