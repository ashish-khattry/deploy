//03_super_keyword
fn alert_admin() {
    println!("Admin has been alerted");
}
pub mod system {
    pub mod cpu {
        pub fn check_temprature() {
            super::super::alert_admin();
        }
    }
}
fn main() {
    system::cpu::check_temprature();
}
