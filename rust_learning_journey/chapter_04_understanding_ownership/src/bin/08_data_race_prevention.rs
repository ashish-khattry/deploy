//08_data_race_prevention
fn main() {
    let mut s1: String = String::from("Narendra Modi!");
    {
        let r1 = &mut s1;
        r1.push_str("Chorr hai !");
        println!("{s1}");
    }
    let r2 = &mut s1;
    r2.push_str("Saath saath nikamma bhi hai !");
    println!("{s1}");
}
