struct DatabaseLock{
    room_name:String,
}
impl Drop for DatabaseLock{
    fn drop(&mut self){
        println!("Cleaning up and unlocking {}",self.room_name);
    }
}
fn main(){
#[allow(dead_code)]
    let d1=DatabaseLock{
        room_name:String::from("VIP room number 100")
    };
    println!("Room locked!");
    std::mem::drop(d1);
    println!("Main fuction is runnig right now but lock is var is cleared");
}
