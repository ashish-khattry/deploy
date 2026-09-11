trait Summary {
    fn summarize(&self);
}
struct NewsArticle {
    content: String,
    location: String,
}
struct Tweet {
    topic: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) {
        println!("Content ={} location={}", self.content, self.location);
    }
}
impl Summary for Tweet {
    fn summarize(&self) {
        println!("Topic={}", self.topic);
    }
}
fn main() {
    let na = NewsArticle {
        content: String::from("A massive flood just appear on nepal today!"),
        location: String::from("Nepal"),
    };
    let tw = Tweet {
        topic: String::from("Rust new version has released!"),
    };
    na.summarize();
    tw.summarize();
}
