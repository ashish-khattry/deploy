fn main() {
    let mut number = vec![1, 2, 3, 4];
    println!("Vector before closure call {:?}", number);
    let mut my_clo = || number.push(10);
    my_clo();
    println!("Vector after closure call {:?}", number);
}
