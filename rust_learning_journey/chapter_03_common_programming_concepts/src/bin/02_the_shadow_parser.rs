//02_the_shadow_parser
fn main() {
    let data = "    404    ";
    println!("={data}");
    let data = data.trim();
    println!("={data}");
    let data: u32 = data.trim().parse::<u32>().expect("Converting error!");
    println!("={data}");
}
