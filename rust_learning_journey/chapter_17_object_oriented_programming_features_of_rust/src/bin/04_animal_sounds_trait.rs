trait MakeSound {
    fn make_sound(&self);
}
struct Dog {}
struct Cat {}
impl MakeSound for Dog {
    fn make_sound(&self) {
        println!("Dog is barking");
    }
}
impl MakeSound for Cat {
    fn make_sound(&self) {
        println!("Cat is making sound");
    }
}
fn main() {
    let mut animal: Box<dyn MakeSound> = Box::new(Dog {});
    animal.make_sound();
    animal = Box::new(Cat {});
    animal.make_sound();
}
