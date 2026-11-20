enum Ticket{
    Age(i32),
}
fn main(){
    let a=Ticket::Age(33);
    let Ticket::Age(age)=a;
    println!("{age}");
    match age{
        kid_age @ 1..=16=>{
            println!("Kid age ={kid_age}");
        }
        _=>{}
    }
}
