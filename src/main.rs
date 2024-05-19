pub mod request;
use http_server_starter_rust::ThreadPoll;
use request::Request;
use std::{
    env, fs,
    io::Write,
    net::{TcpListener, TcpStream},
    path::Path,
};

fn handle_connection(mut stream: TcpStream, directory: String) {
    let request = Request::new(&mut stream).unwrap();
    let response = match request.status_line.http_method.as_str() {
        "GET" => {
            let path = request.status_line.path.as_str();
            let response = if path.starts_with('/') && path.len() == 1 {
                String::from("HTTP/1.1 200 OK\r\n\r\n")
            } else if path.starts_with("/echo") {
                let query: Vec<&str> = path.split('/').collect();
                let contents = query.last().unwrap();
                let length = contents.len();
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {length}\r\n\r\n{contents}"
                )
            } else if path.starts_with("/files") {
                let params: Vec<&str> = path.split("/").collect();
                let query = params.last().unwrap();
                let file_path = format!("{directory}/{query}");
                let file = Path::new(file_path.as_str());
                let contents;
                let response = if file.exists() {
                    contents = fs::read_to_string(file_path).unwrap();
                    let length = contents.len();
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {length}\r\n\r\n{contents}"
                    )
                } else {
                    String::from("HTTP/1.1 404 Not Found\r\n\r\n")
                };
                response
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
    stream.write_all(response.as_bytes()).unwrap();
}
fn main() {
    let mut directory = None;

    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        if args[i] == "--directory" && i + 1 < args.len() {
            directory = Some(args[i + 1].clone());
            break;
        }
    }
    let dir = match directory {
        Some(dir) => dir,
        None => String::from(""),
    };
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPoll::new(4);
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                let dir_clone = dir.clone();
                pool.execute(|| {
                    handle_connection(_stream, dir_clone);
                })
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
