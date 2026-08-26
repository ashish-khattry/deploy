use rusqlite::{params, Connection, Result};
#[allow(unused_imports)]
use serde::Serialize;
use serde_json;
use std::io::{Read, Write};
use std::net::TcpListener;

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
    role: String,
}
fn main() {
    let db = match Connection::open("users.db") {
        Ok(connection) => connection,
        Err(e) => {
            println!("Server error :{}", e);
            std::process::exit(1);
        }
    };

    match db.execute("create table if not exists users (id integer primary key,name text not null, role text not null)",[])
    {
        Ok(_)=>(),
        Err(e)=>
        {
            println!("Error accurred:{}",e);
            std::process::exit(1);
        }
    }

    match db.execute(
        "insert or ignore into users (id,name,role) values (?1,?2,?3)",
        params![1, "Ashish Khattry", "RUST developer"],
    ) {
        Ok(_) => (),
        Err(e) => {
            println!("Error accured:{}", e);
            std::process::exit(1);
        }
    };
    let server = match TcpListener::bind("127.0.0.1:8080") {
        Ok(port) => port,
        Err(e) => {
            println!("Error accurred:{}", e);
            std::process::exit(1);
        }
    };

    for connection in server.incoming() {
        let mut connect = match connection {
            Ok(connect) => connect,
            Err(e) => {
                println!("Error accurred:{}", e);
                continue;
            }
        };

        let mut buffer = [0; 1024];

        match connect.read(&mut buffer) {
            Ok(_) => (),
            Err(e) => {
                println!("Error accurred:{}", e);
                continue;
            }
        };

        let user_info = String::from_utf8_lossy(&buffer);

        if user_info.contains("GET /users HTTP/1.1") {
            let mut statement = match db.prepare("select id, name, role from users") {
                Ok(database) => database,
                Err(e) => {
                    println!("Error accurred:{}", e);
                    continue;
                }
            };
            let user_iterator = statement
                .query_map([], |row| {
                    Ok(User {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        role: row.get(2)?,
                    })
                })
                .expect("error accurred");

            let mut user_list: Vec<User> = Vec::new();

            for user in user_iterator {
                user_list.push(user.expect("error accurred"));
            }

            let json_data = match serde_json::to_string(&user_list) {
                Ok(data) => data,
                Err(e) => {
                    println!("Error accured:{}", e);
                    continue;
                }
            };

            let response = format!(
                "HTTP/1.1 \r\ncontent-type:application/json; charset=utf-8\r\n\r\n{}",
                json_data
            );

            match connect.write_all(response.as_bytes()) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error accurred:{}", e);
                    continue;
                }
            };
        } else {
            let error_res =
                "HTTP/1.1 \r\ncontent-type:application/json\r\n\r\n{\"error\":\"route not found\"}";
            match connect.write_all(error_res.as_bytes()) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error accured:{}", e);
                    continue;
                }
            };
        }
    }
}
