pub fn create_file(file_path: &str) {
    let mut file = std::fs::File::create(file_path).expect("Error while creating the file!");
    std::io::Write::write_all(&mut file,"Rust is a very powerfull programming language.\nIts is all the power and speed of c++ and advance feature of java and python\nRust provide very high speed while software engineering.".as_bytes()).expect("Error while writing the file!");
    drop(file);
}
pub struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {
    pub fn  build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Arguments are incomplete!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nSearching of {}", config.query);
    println!("In file {}", config.file_path);
    let file_path = config.file_path.clone();
    let content = std::fs::read_to_string(file_path)?;
    let ignore_case = std::env::var("IGNORE_CASE").is_ok();
    if ignore_case {
        for line in search_insensitive(&config.query, &content) {
            println!("{line}");
        }
    } else {
        for line in search_sensitive(&config.query, &content) {
            println!("{line}");
        }
    }

    Ok(())
}

pub fn search_sensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}
pub fn search_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<String> {
    let mut result = Vec::new();
    let query_lower = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query_lower) {
            result.push(line.to_string())
        }
    }
    result
}
#[cfg(test)]
mod testing_search {
    use super::{search_insensitive, search_sensitive};
    #[test]
    fn test_search_sensitive() {
        let content = "Rust is fast.
It is secure.
It is safe
It is demanding language.";
        let query = "Rust";
        assert_eq!(search_sensitive(&query, &content), vec!["Rust is fast."]);
    }
    #[test]
    fn test_search_insensitive() {
        let content = "Rust is fast.
It is secure.
It is safe
It is demanding language.";
        let query = "RUSt";
        assert_eq!(
            search_insensitive(&query, &content),
            vec!["Rust is fast.".to_string()]
        );
    }
}
