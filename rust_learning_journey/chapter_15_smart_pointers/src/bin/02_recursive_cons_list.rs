#[derive(Debug)]
#[allow(unused)]
enum List {
    Cons(i32, Box<List>),
    Nill,
}
use crate::List::{Cons, Nill};
fn main() {
    let lists = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nill))))));
    println!("{:?}", lists);
}
