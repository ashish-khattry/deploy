//05_infinit_echo
fn main() {
    loop {
        println!("This is infinit input taking loop");
        println!("-->> Enter your input!");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Input error!");
        println!("Input is ={}", input);
        println!("Enter \"exit\" for break the loop or type anything else to continue!");
        let mut ch = String::new();
        std::io::stdin().read_line(&mut ch).expect("Input error!");
        let ch = ch.trim();
        if ch == "exit" {
            println!("Infinit loop has been breaked!");
            break;
        }
    }
}
