fn main(){
    let pin=1234;
    match pin{
        1234=>{
            println!("Access granted");
        }
        _=>{
            println!("Access denied!");
        }
    }
}
