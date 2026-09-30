fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Please enter 2 CL arguments");
        std::process::exit(1);
    }
    println!("First arguments={}\nSecond arguments={}", args[1], args[2]);
}
