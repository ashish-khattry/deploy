use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug)]
enum List{
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use List::{Cons,Nil};
fn main(){
    let data=Rc::new(RefCell::new(5));
    let r1=Rc::new(Cons(Rc::clone(&data), Rc::new(Nil)));
    let r2=Cons(Rc::new(RefCell::new(10)), Rc::clone(&r1));
    let r3=Cons(Rc::new(RefCell::new(20)),Rc::clone(&r1));
    *data.borrow_mut()+=10;
    println!("r1={:?}",r1);
    println!("r2={:?}",r2);
    println!("r3={:?}",r3);
}
