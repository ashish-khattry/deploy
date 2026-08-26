use std::io::prelude::*;
use std::net::TcpListener;
fn main() {
    let server = TcpListener::bind("127.0.0.1:8080").expect("port 8080 is busy!");
    println!("Web server is activated and live on...\nhttps://127.0.0.1:8080\n");
    for connection in server.incoming() {
        let mut connect = connection.expect("Failed to make connection!");
        let mut buffer = [0; 1024];
        connect.read(&mut buffer).expect("Failed to take reqwest!");
        let text = String::from_utf8_lossy(&buffer[..]).to_string();
        println!("Browser reqwested={}", text);
        if text.contains("GET / HTTP/1.1") {
            connect.write_all("HTTP/1.1 200 OK\r\n\r\n\r\n<p>Web server is connected and running now on this browser<br>Connection status: Successfull<br>Server owner:Ashish(The Rust MasterMind)<br>Source language:Rust(The Giant)<br>Project no.:My first web PROJECT<br>(Meri Jingadi Ab Shuru Ho Gayi Hai)</p>".as_bytes()).expect("Failed to response!");
        } else {
            connect
                .write_all("HTTP/1.1 404 NOT FOUND\r\n\r\n<p>404 Page not found!!!</p>".as_bytes())
                .expect("Fail to response!");
        }
        connect.flush().expect("Failed to flush!");
    }
}
