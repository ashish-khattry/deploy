// libaray file
pub use private_core::boot_system;
mod private_core {
    pub fn boot_system() {
        println!("System is booting..");
    }
}
