//10_mega_collection_database
use std::collections::HashMap;

fn add_score(db: &mut HashMap<String, Vec<i32>>, player_name: &str, score: i32) {
    let scores_list = db.entry(player_name.to_string()).or_insert(Vec::new());
    scores_list.push(score);
    println!("{} score added for {}", score, player_name);
}

fn show_scores(db: &HashMap<String, Vec<i32>>, player_name: &str) {
    match db.get(player_name) {
        Some(scores) => println!(" {}'s Scores: {:?}", player_name, scores),
        None => println!(" Player '{}' not found in database!", player_name),
    }
}

fn main() {
    let mut game_db: HashMap<String, Vec<i32>> = HashMap::new();

    add_score(&mut game_db, "Ashish", 100);
    add_score(&mut game_db, "Ashish", 250);
    add_score(&mut game_db, "Vikas", 50);

    println!("-------------------------");

    show_scores(&game_db, "Ashish");
    show_scores(&game_db, "Punit");
}
