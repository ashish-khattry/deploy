use std::env;
struct Config{
    query:String,
    file_path:String,////incomplete and wrong code 
    ignore_case:bool,
}
impl Config{
    fn build(args:impl Iterator<Item=String>)->Result<Config,&'static str>{
        let args=args.next();
        let query=match args.next(){
            Some(q)=>q,
            None=>return Err("Qeury error found.");
        };
        let file_path=match args.next(){
            Some(f)=>f,
            None=>return Err("File path error found.");
        };
        let ignore_case=env::var("IGNORE_CASE").is_ok();
        Ok(
            Config{
                query,
                file_path,
                ignore_case,
            }
        )
    }
}
fn create_file(file_path:&str){
    let file=std::fs::File(file_path).expect("File creating error.");
    std::io::Write::write_all(&mut file,"Twinkle twinkle little star\nHow i wonder what you are\nSome people wonder what see you\nDamage all finger that you feel".as_bytes()).expect("Error while wrting file.");
    drop(file);
}
fn search(query:String,content:String)->Vec<String>{
    content.lines().filter(|x|x.contains(&query)).collect()
}
fn main(){
    let args=env::args().collect();
    let config=Config::build(args).unwrap_or_else(|e|{println!("{e}");});
    let search_result:Vec<_>=search(config.query,config.file_path);
    println!("{:?}",search_result);
}