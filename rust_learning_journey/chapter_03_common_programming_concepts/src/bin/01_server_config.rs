//01_server_config
fn main() {
    const FIX_CONNECTION: u32 = 10;
    println!("Fix connection={FIX_CONNECTION}");
    let mut connection = 4;
    println!("Now connection={connection}");
    connection = 5;
    println!("Now connection={connection}");
    connection = 6;
    println!("Now connection={connection}");
}
