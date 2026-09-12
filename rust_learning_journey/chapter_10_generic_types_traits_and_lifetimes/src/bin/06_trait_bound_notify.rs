trait Summary {
    fn summarize(&self) {
        println!("Summary is ");
    }
}
struct Video {}
struct Tweet {
    topic: String,
}
impl Summary for Video {}
impl Summary for Tweet {
    fn summarize(&self) {
        println!("Summary is {}", self.topic);
    }
}
fn notify(item: &impl Summary) {
    item.summarize();
}
fn main() {
    let v = Video {};
    let t = Tweet {
        topic: String::from("Narendra modi goback"),
    };
    notify(&t);
    notify(&v);
}
