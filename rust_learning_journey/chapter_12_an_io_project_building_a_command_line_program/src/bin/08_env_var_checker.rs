fn main(){
    let debug_mode=std::env::var("DEBUG_MODE").is_ok();
    if debug_mode{
        println!("Debug mode is on");
    }
    else{
        println!("Debug mode is off");
    }
}
