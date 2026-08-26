//04_function_move_trap
fn main() {
    let s: String = String::from("Hello world");
    let i: i32 = 65;
    take_string(s);
    copy_int(i);
    println!("Integer after giving its ownership to function={i}");
    //println!("String after giving its ownership to fuction={s}");//error !
}
fn take_string(s: String) {
    println!("String inside fuction={s}");
}
fn copy_int(i: i32) {
    println!("Integer inside function={i}");
}
