struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Wrong arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        return Ok(Config { query, file_path });
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
    let content = std::fs::read_to_string(&config.file_path)?;
    println!("File text:\n{content}");
    Ok(())
}
fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(&query) {
            result.push(line);
        }
    }
    result
}
#[cfg(test)]
mod testing {
    use super::*;
    #[test]
    fn test_search() {
        let content = "Rust:
Memory safe, fast, efficient.
Duct tape.
Keep coding ";
        let query = "Rust:";
        assert_eq!(search(&query,&content),vec!["Rust:"]);
    }
}

