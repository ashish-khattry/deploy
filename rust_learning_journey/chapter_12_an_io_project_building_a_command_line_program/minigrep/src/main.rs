use minigrep::{Config,run,create_file};
fn main() {
    create_file("rust.txt");
    let args: Vec<String> = std::env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e| {
        eprintln!("Error while calling build method {e}");
        std::process::exit(1);
    });
    if let Err(e) = run(&config) {
        eprintln!("Error while calling run method {e}");
        std::process::exit(1);
    }
}