//08_aliasing_with_as
pub mod local_db {
    pub fn connect() {
        println!("Database connected");
    }
}
pub mod cloud_db {
    pub fn connect() {
        println!("Cloud database conneted");
    }
}
use cloud_db::connect as CloudConnect;
use local_db::connect as LocalConnect;
fn main() {
    LocalConnect();
    CloudConnect();
}
