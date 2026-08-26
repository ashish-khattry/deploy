//06_string_slicing_safeguard
fn main() {
    let string = "नमस्ते🚀";
    let result = string.chars().take(2).collect::<String>();
    println!("Result={}", result);
}
