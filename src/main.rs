pub mod request;
use http_server_starter_rust::ThreadPoll;
use request::Request;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let request = Request::new(&mut stream).unwrap();
    let response = match request.status_line.http_method.as_str() {
        "GET" => {
            let path = request.status_line.path.as_str();
            let response = if path.starts_with("/") && path.len() == 1 {
                String::from("HTTP/1.1 200 OK\r\n\r\n")
            } else if path.starts_with("/echo") {
                let query: Vec<&str> = path.split("/").collect();
                let contents = query.last().unwrap();
                let length = contents.len();
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{contents}"
                )
            } else if path.starts_with("/user-agent") {
                let contents = request.headers.get("User-Agent").unwrap();
                let length = contents.len();
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{contents}"
                )
            } else {
                String::from("HTTP/1.1 404 Not Found\r\n\r\n")
            };
            response
        }
        &_ => todo!(),
    };
    // dbg!(response);
    stream.write_all(response.as_bytes()).unwrap();
    dbg!(request);
}
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPoll::new(4);
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => pool.execute(|| {
                handle_connection(_stream);
            }),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
