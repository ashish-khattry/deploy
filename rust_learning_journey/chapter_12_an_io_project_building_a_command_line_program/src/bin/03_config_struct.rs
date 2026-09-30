struct Config{
    user:String,
    password:String,
}
impl Config{
    fn build(args:&Vec<String>)->Result<Config,&'static str>{
        if args.len()<3{
            return Err("Wrong arguments");
        }
        let user=args[1].clone();
        let password=args[2].clone();
        return Ok(Config{user,password});
    }
}
fn main(){
    let args=std::env::args().collect();
    let config=Config::build(&args).unwrap_or_else(|e|{
        println!("Error accur {e}");
        std::process::exit(1);
    });
    println!("User {} and Password {}",config.user,config.password);
}
