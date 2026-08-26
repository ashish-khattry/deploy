//02_smart_ip_scanner
#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr {
    V4(u8),
    V6(String),
}
fn main() {
    let v4 = IpAddr::V4(127);
    let v6 = IpAddr::V6(String::from("127.0.0.1"));
    println!("V4={:?} and v6={:?}", v4, v6);
}
