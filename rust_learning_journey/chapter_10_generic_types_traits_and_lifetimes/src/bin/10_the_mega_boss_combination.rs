fn longest_and_announcement<'a, T>(a: &'a str, b: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement is= {}", ann);
    if a > b {
        a
    } else {
        b
    }
}
fn main() {
    let a = "Rust";
    let b = "java";
    let ann = "Starting the mega project!";
    let long = longest_and_announcement(a, b, ann);
    println!("Longest is ={}", long);
}
