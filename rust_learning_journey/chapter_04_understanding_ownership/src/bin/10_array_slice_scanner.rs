//10_array_slice_scanner
fn main() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let b = &a[1..4];
    print_slice(b);
}
fn print_slice(s: &[i32]) {
    println!("First element is ={}", s[0]);
}
