//02_nested_modules
pub mod server {
    pub mod database {
        pub fn connect_db() {
            println!("Database connected");
        }
    }
}
fn main() {
    server::database::connect_db();
}
