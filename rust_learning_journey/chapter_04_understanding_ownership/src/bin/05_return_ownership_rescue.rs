//05_return_ownership_rescue
fn main() {
    let s1 = give_string();
    let s2 = take_string_and_give_string(s1);
    println!("String is ={s2}");
}
fn give_string() -> String {
    String::from("Seeta ke pati ka naam le lekar\nNeeta ke pati ka kaam kiya ja raha hai 🤣 !")
}
fn take_string_and_give_string(s: String) -> String {
    s
}
