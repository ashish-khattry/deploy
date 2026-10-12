use std::cell::RefCell;
trait Messenger {
    fn send(&self, msg: String);
}
#[derive(Debug)]
struct MockMessenger {
    my_messege: RefCell<Vec<String>>,
}
impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            my_messege: RefCell::new(vec![]),
        }
    }
}
impl Messenger for MockMessenger {
    fn send(&self, msg: String) {
        self.my_messege.borrow_mut().push(msg);
    }
}
struct LimitTracker<'a, T> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage = self.value as f64 / self.max as f64;
        if percentage >= 1.0 {
            self.messenger
                .send("You have used more than 100% of your quata!".to_string());
        } else if percentage >= 0.9 {
            self.messenger
                .send("You have used more than 90% of your quata!".to_string());
        } else if percentage >= 0.75 {
            self.messenger
                .send("You have used more than 75% of your quata!".to_string());
        }
    }
}

fn main() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    limit_tracker.set_value(80);
    println!("Mock messenger is {:?}", mock_messenger.my_messege.borrow());
}
