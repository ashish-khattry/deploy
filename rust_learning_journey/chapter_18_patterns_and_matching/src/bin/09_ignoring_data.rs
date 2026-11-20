struct Player{
    id:i32,
    score:String,
    health:String,
    mana:bool,
}
fn main(){
    let player1=Player{
        id:8884,
        score:String::from("HIGH"),
        health:String::from("FINE"),
        mana:true,
    };
    let Player{id,..}=player1;
    println!("{id}");
}
