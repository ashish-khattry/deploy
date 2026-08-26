//02_buffer_overread_prevention
fn main() {
    let array = [1, 2, 3, 4, 5]; // array of 5 elements
    println!("100th element of array={}", array[99]); // rust will call panic! macro here becouse 100 elements is not exist in this array but if this is code were written in C or any other programming language this print statement will print any randon value of memory
}
