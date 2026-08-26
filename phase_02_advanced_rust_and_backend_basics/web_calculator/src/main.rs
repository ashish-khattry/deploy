use std::io::prelude::*;
use std::net::TcpListener;
fn main() {
    let server = TcpListener::bind("127.0.0.1:8080").expect("Port 8080 is Busy!");
    println!("Boom server is conneted and LIVE on https://127.0.0.1:8080");
    for connection in server.incoming() {
        let mut connect = connection.expect("Connection error!");
        let mut buffer = [0; 1024];
        connect
            .read(&mut buffer)
            .expect("Error while taking request!");
        let user_info = String::from_utf8_lossy(&buffer[..]);
        if user_info.contains("GET / HTTP/1.1") {
            let html_form = "HTTP/1.1 200 OK\r\ncontent-type:text/html;charset=utf-8\r\n\r\n
             <h3>Ⓞⓝⓛⓘⓝⓔ Ⓒⓐⓛⓒⓤⓛⓐⓣⓞⓡ 🔢➕➖✖️➗= </h2><form action='/' method='GET'>
             <input type='text' name='n1' placeholder='【1】'>
             <input type='text' name='n2' placeholder='【2】'>
             <br><br>
             <button type='submit' name='op' value='add'> ➕ </button>
             <button type='submit' name='op' value='sub'> ➖ </button>
             <button type='submit' name='op' value='mul'> ✖️ </button>
             <button type='submit' name='op' value='div'> ➗ </button>
             <button type='submit' name='op' value='rem'> ％ </button>
             <button type='submit' name='op' value='pow'> xʸ </button>
             </form>";
            connect
                .write_all(html_form.as_bytes())
                .expect("Error while responding!");
        } else if user_info.contains("GET /?n1=") {
            let p11 = user_info.split("=").nth(1).unwrap_or("0.0");
            let p22 = p11.split("&").next().unwrap_or("0.0");
            let n1: f64 = p22.trim().parse::<f64>().unwrap_or(0.0);
            let p1 = user_info.split("=").nth(2).unwrap_or("0.0");
            let p2 = p1.split("&").next().unwrap_or("0.0");
            let n2: f64 = p2.trim().parse::<f64>().unwrap_or(0.0);
            let op1 = user_info.split("=").nth(3).unwrap_or("add");
            let op2 = op1.split(" ").next().unwrap_or("add");
            let op = op2;
            match op {
                "add" => {
                    let result = format!("{}", n1 + n2);
                    let r=format!("HTTP/1.1 200 OK\r\ncontent-type:text/html; charset=utf-8\r\n\r\n<h3>➡️={}</h3><a href='/'>go back</a>",result);
                    connect
                        .write_all(r.as_bytes())
                        .expect("Error while responding!");
                }
                "sub" => {
                    let result = format!("{}", n1 - n2);
                    let r=format!("HTTP/1.1 200 OK\r\ncontent-type:text/html; charset=utf-8\r\n\r\n<h3>➡️={}</h3><a href='/'>go back</a>",result);
                    connect
                        .write_all(r.as_bytes())
                        .expect("Error while responding!");
                }
                "mul" => {
                    let result = format!("{}", n1 * n2);
                    let r=format!("HTTP/1.1 200 OK\r\ncontent-type:text/html; charset=utf-8\r\n\r\n<h3>➡️={}</h3><a href='/'>go back</a>",result);
                    connect
                        .write_all(r.as_bytes())
                        .expect("Error while responding!");
                }
                "div" => {
                    if n2 == 0.0 {
                        connect.write_all("HTTP/1.1 400 BAD REQUEST\r\ncontent-type:text/html;charset=utf-8\r\n\r\n<h2>second number if zero devision not possible</h2><a href='/'>go back</a>".as_bytes()).expect("failure to response");
                    } else {
                        let result = format!("{}", n1 / n2);
                        let r=format!("HTTP/1.1 200 OK\r\ncontent-type:text/html; charset=utf-8\r\n\r\n<h3>➡️={}</h3><a href='/'>go back</a>",result);
                        connect
                            .write_all(r.as_bytes())
                            .expect("Error while responding!");
                    }
                }
                "pow" => {
                    let r = format!("{}", n1.powf(n2));
                    let res=format!("HTTP/1.1\r\ncontent-type:text/html;charset=utf-8\r\n\r\n<h3>➡️={}</h3><a href='/'>go back</a>",r);
                    connect
                        .write_all(res.as_bytes())
                        .expect("Error while responding!");
                }
                "rem" => {
                    if n2 == 0.0 {
                        connect.write_all("HTTP/1.1 400 BAD REQUEST\r\ncontent-type:text/html;charset=utf-8\r\n\r\n<h2>second number if zero devision not possible</h2><a href='/'>go back</a>".as_bytes()).expect("failure to response");
                    } else {
                        let r = format!("{}", n1 % n2);
                        let res=format!("HTTP/1.1\r\ncontent-type:text/html;charset=utf-8\r\n\r\n<h3>➡️={}</h3><a href='/'>go back</a>",r);
                        connect
                            .write_all(res.as_bytes())
                            .expect("Error while responding!");
                    }
                }
                _ => {
                    let r = "HTTP/1.1 400 BAD REQUEST\r\nContent-Type: text/html\r\n\r\n<h2>Invalid Operator!</h2><a href='/'>Go Back</a>";
                    connect
                        .write_all(r.as_bytes())
                        .expect("Error while responding!");
                }
            }
        } else {
            connect.write_all("HTTP/1.1 404 NOT FOUND\r\ncontent-type:text/html; charset=utf-8\r\n\r\n<h2>page not found</h2><a href='/'>go back</a>".as_bytes()).expect("Error while responding!");
        }
    }
}
