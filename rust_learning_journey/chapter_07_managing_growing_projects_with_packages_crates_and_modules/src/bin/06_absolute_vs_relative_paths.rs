//06_absolute_vs_relative_paths
mod cloud {
    pub mod storage {
        pub fn upload_file() {
            println!("File is uploaded");
        }
    }
    pub mod analytics {
        pub fn run() {
            crate::cloud::storage::upload_file();
            super::storage::upload_file();
        }
    }
}
fn main() {
    cloud::analytics::run();
}
