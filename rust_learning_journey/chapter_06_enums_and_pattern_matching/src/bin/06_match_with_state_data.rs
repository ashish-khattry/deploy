//06_match_with_state_data
#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alaska,
    Alabama,
}
#[allow(dead_code)]
enum Coin {
    Penny,
    Quarter(UsState),
}
fn main() {
    let c = Coin::Quarter(UsState::Alaska);
    match c {
        Coin::Penny => {
            println!("This is a penny");
        }
        Coin::Quarter(state) => {
            println!("This quarter belongs to {:?} state", state);
        }
    }
}
