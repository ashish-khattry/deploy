use std::env;
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}
impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(q) => q,
            None => return Err("Qeury error found."),
        };
        let file_path = match args.next() {
            Some(f) => f,
            None => return Err("File path error found."),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
fn create_file(file_path: &str) {
    let mut file = std::fs::File::create(file_path).expect("File creating error.");
    std::io::Write::write_all(&mut file,"Twinkle twinkle little star\nHow i wonder what you are\nSome people wonder what see you\nDamage all finger that you feel".as_bytes()).expect("Error while wrting file.");
    drop(file);
}
fn search(query: String, content: String) -> Vec<String> {
    content
        .lines()
        .filter(|x| x.contains(&query))
        .map(|s| s.to_string())
        .collect()
}
fn main() {
    create_file("poem.txt");
    let args = env::args();
    let config = Config::build(args).unwrap_or_else(|e| {
        println!("{e}");
        std::process::exit(1);
    });

    let content = std::fs::read_to_string(&config.file_path).unwrap();
    let search_result = search(config.query, content);
    println!("{:?}", search_result);
}
