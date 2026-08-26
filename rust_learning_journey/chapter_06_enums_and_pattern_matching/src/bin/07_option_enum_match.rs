//07_option_enum_match
#[allow(dead_code)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x //this code has erro fix it later
    {
        Some(n)=>Some(n+1),
        None=>
        {
            println!("This is none");
            None
        }
    }
}
fn main() {
    let a: Option<i32> = Some(32);
    let i = plus_one(a);
    println!("I={:?}", i);
}
