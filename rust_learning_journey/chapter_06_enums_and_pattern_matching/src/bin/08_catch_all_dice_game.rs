//08_catch_all_dice_game
fn main() {
    let dice: u8 = 3;
    match dice {
        3 => {
            println!("You win a hat");
        }
        7 => {
            println!("You lose a hat");
        }
        _ => {
            println!("Roll again");
        }
    }
}
