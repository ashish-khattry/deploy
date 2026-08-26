//04_safe_vector_retrieval
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Enter the index number ");
    let mut index = String::new();
    std::io::stdin().read_line(&mut index).expect("Input error");
    let index: usize = match index.trim().parse::<usize>() {
        Ok(index) => index,
        Err(_) => {
            println!("Invalid index number");
            return;
        }
    };
    let number = match numbers.get(index) {
        Some(number) => number,
        None => {
            println!("Invalid index number");
            return;
        }
    };
    println!("Number={}", number);
}
