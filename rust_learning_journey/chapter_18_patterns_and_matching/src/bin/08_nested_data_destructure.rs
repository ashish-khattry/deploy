enum Color{
    Rgb(i32,i32,i32),
}
enum Message{
    Change(Color),
}
fn main(){
    let c=Message::Change(Color::Rgb(10,20,30));
    match c{
        Message::Change(Color::Rgb(r,g,b))=>{
            println!("Colors are :\nR:{r}\nG:{g}\nB:{b}");
        },
       
    }
}


