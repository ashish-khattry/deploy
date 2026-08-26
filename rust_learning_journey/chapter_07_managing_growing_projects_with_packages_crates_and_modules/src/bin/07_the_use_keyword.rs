//07_the_use_keyword
pub mod os {
    pub mod file_system {
        pub mod permissions {
            pub fn grant_access() {
                println!("Welcome to my computer");
            }
        }
    }
}
use os::file_system::permissions;
fn main() {
    permissions::grant_access();
}
