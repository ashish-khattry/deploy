fn create_file(file_path: &str) {
    let mut file = std::fs::File::create(file_path).expect("Error while creating the file!");
    std::io::Write::write_all(&mut file,"Rust is a very powerfull programming language.\nIts is all the power and speed of c++ and advance feature of java and python\nRust provide very high speed while software engineering.".as_bytes()).expect("Error while writing the file!");
    drop(file);
}
struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Arguments are incomplete!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nSearching of {}", config.query);
    println!("In file {}", config.file_path);
    let file_path = config.file_path.clone();
    let content = std::fs::read_to_string(file_path)?;
    println!("File text:\n{content}");
    Ok(())
}
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
fn search_sensitive<'a>(query:&'a str, content:&'a str)->Vec<&'a str>{
    let mut result=Vec::new();
    for line in content.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
}
fn search_insensitive<'a>(query:&'a str,content:&'a str)->Vec<String>{
    let mut result=Vec::new();
    let query_lower=query.to_lowercase();
    for line in content.lines(){
        if line.to_lowercase().contains(&query_lower){
            result.push(line.to_string())
        }
    }
    result
}
#[cfg(test)]
mod testing_search{
    use super::{search_sensitive,search_insensitive};
    #[test]
    fn test_search_sensitive(){
        let content="Rust is fast.
It is secure.
It is safe
It is demanding language.";
        let query="Rust";
        assert_eq!(search_sensitive(&query,&content),vec!["Rust is fast."]);
    }
    #[test]
    fn test_search_insensitive(){
        let content="Rust is fast.
It is secure.
It is safe
It is demanding language.";
        let query="RuSt";
        assert_eq!(search_insensitive(&query,&content),vec!["Rust is fast.".to_string()]);
    }
}