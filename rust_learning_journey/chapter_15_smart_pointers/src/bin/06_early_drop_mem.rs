fn main() {
    let var = String::from("VIP room 100");
    print!("Room locked. ");
    std::mem::drop(var);
    //print!("{var}"); //error[E0382]: borrow of moved value: `var`
    print!("Room forcefully dropped! Main function continues...")
}
