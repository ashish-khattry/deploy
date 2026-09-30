struct Config {
    user: String,
    password: String,
}
impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Wrong arguments");
        }
        let user = args[1].clone();
        let password = args[2].clone();
        return Ok(Config { user, password });
    }
}
fn main() {
    let args = std::env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e| {
        eprintln!("Error accur {e}");
        std::process::exit(1);
    });
    if let Err(e) = run(&config) {
        println!("{e}");
    }
}
fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(&config.password)?;
    println!("File text:\n{content}");
    Ok(())
}
