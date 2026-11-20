enum Action{
    Move{x:i32, y:i32,},
    Speak(String),
}
fn main(){
    let m=Action::Move{x:10,y:20};
    let _=Action::Speak(String::from("hello world"));
    match m{
        Action::Move{x,y}=>{
            println!("{x} {y}");
        },
        Action::Speak(data)=>{
            println!("{data}");
        },
        
    }
}
