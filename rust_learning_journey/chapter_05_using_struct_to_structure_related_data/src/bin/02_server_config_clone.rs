//02_server_config_clone
#[derive(Debug)]
#[allow(dead_code)]
struct Server {
    ip_address: String,
    port: i32,
    is_online: bool,
    cpu_cores: i32,
}
fn main() {
    let server1 = Server {
        ip_address: "256.00.2651".to_string(),
        port: 8080,
        is_online: true,
        cpu_cores: 8,
    };
    println!("Server 1 detail\n{:#?}", server1);
    let server2 = Server {
        ip_address: "300.545.2551".to_string(),
        ..server1
    };
    println!("Server 2 detial\n{:#?}", server2);
}
