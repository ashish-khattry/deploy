///Devide two numbers safely, and return final result
///
/// #Panics
///
/// This function will panic will panic instantly
/// if the second number is zero 0.
/// Becuse deviding a number by zero is not possible. And it will crash the system.
///
pub fn devide(a: usize, b: usize) -> usize {
    if b == 0 {
        panic!("It is not possible to devide a number by zero!");
    }
    a / b
}
fn main() {}
