fn main(){
    let pin=1234;
    match pin{
        1234=>{
            println!("Accesee granted");
        }
        _=>{
            println!("Access denied!");
        }
    }
}
