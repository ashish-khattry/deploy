enum Ticket{
    Age(i32),
}
fn main(){
    let my_ticket=Ticket::Age(18);
    match my_ticket{
        Ticket::Age(kid_age @ 1..=16)=>{
            println!("Kid age :{kid_age}");
        },
        Ticket::Age(senior) if senior>60=>{
            println!("Senior citizen age :{senior}");
        },
        Ticket::Age(age)=>{
            println!("Other age: {age}");
        },
    }
}
