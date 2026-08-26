use std::io::prelude::*;
use std::net::TcpListener;
fn main() {
    let server = TcpListener::bind("127.0.0.1:8080").expect("Port 8080 is busy!");
    println!("🚀 Dynamic Table Server is LIVE on http://127.0.0.1:8080");
    for connection in server.incoming() {
        let mut connect = connection.expect("Failed to make connection!");
        let mut buffer = [0; 1024];
        connect.read(&mut buffer).expect("Failed to read request!");
        let text = String::from_utf8_lossy(&buffer[..]).to_string();
        println!("\nBrowser Requested: {}", text);
        if text.contains("GET / HTTP/1.1") {
            let form_html="HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n<h2>Dynamic Math Engine 🚀</h2><form action='/' method='GET'><input type='text' name='num' placeholder='Enter a number...'><input type='submit' value='Generate Table'></form>";
            connect
                .write_all(form_html.as_bytes())
                .expect("Error sending form");
        } else if text.contains("GET /?num=") {
            let part1 = text.split("=").nth(1).unwrap_or("0");
            let raw_number_string = part1.split(" ").next().unwrap_or("0");
            let final_number = raw_number_string.parse::<i32>().unwrap_or(0);
            if final_number == 0 {
                let error_page = "HTTP/1.1 400 BAD REQUEST\r\nContent-Type: text/html; charset=utf-8\r\n\r\n<h2>❌ Invalid Input! Enter numbers only.</h2><a href='/'>Go Back</a>";
                connect
                    .write_all(error_page.as_bytes())
                    .expect("Error sending error page");
            } else {
                let mut table_data = String::new();
                for i in 1..=10 {
                    let row = format!("{}<br>", final_number, i, final_number * i);
                    table_data.push_str(&row);
                }
                let dynamic_response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n<h2>Table of {} 🚀</h2>{}<br><a href='/'>Go Back</a>", final_number, table_data);

                connect
                    .write_all(dynamic_response.as_bytes())
                    .expect("Error sending table");
            }
        } else {
            let error_404 = "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html; charset=utf-8\r\n\r\n<h2>404 ERROR: Page Not Found! 🚧</h2>";
            connect
                .write_all(error_404.as_bytes())
                .expect("Error sending 404");
        }
        connect.flush().expect("Failed to flush!");
    }
}
