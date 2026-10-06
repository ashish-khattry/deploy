use core_lib::capital_string;
fn main() {
    let s = String::from("Java become old now");
    let c_s = capital_string(&s);
    println!("Small string={s}\nCapital string ={c_s}");
}
