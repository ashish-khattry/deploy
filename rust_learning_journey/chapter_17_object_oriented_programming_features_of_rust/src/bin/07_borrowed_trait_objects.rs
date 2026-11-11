trait Worker {
    fn work(&self);
}
struct Plumber {}
struct Electrician {}
impl Worker for Plumber {
    fn work(&self) {
        println!("Plumber is working");
    }
}
impl Worker for Electrician {
    fn work(&self) {
        println!("Electrician is working");
    }
}
fn assign_task(worker: &dyn Worker) {
    worker.work();
}
fn main() {
    let plumber = Plumber {};
    let electrician = Electrician {};
    assign_task(&plumber);
    assign_task(&electrician);
}
