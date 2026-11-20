enum Color{
    Rbg(i32,i32,i32),
}
enum Message{
    Change(Color),
}
fn main(){
    let c=Message::Change(Color::Rbg(10,20,30));
    match c{
        Message::Change(Color::Rbg(x,y,z))=>{
            println!("{x} {y} {z}");
        },
       
    }
}


