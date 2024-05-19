use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
#[derive(Debug)]
pub struct Request {
    pub status_line: StatusLine,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub struct StatusLine {
    pub http_method: String,
    pub path: String,
    pub http_protocol: String,
}
impl Request {
    pub fn new(stream: &mut TcpStream) -> Result<Request, &'static str> {
        let mut reader = BufReader::new(stream);
        let mut request_line = String::new();
        if reader.read_line(&mut request_line).is_err() {
            return Err("Error reading status line!");
        }
        if request_line.is_empty() {
            return Err("Request line is empty");
        }
        let status_line_parts: Vec<&str> = request_line.trim_end().split_whitespace().collect();
        if status_line_parts.len() != 3 {
            return Err("Invalid Request: Status line does not contain exactly three parts");
        }
        let status_line = StatusLine {
            http_method: status_line_parts[0].to_string(),
            path: status_line_parts[1].to_string(),
            http_protocol: status_line_parts[2].to_string(),
        };
        // Read headers
        let mut headers = HashMap::new();
        let mut header_line = String::new();
        loop {
            header_line.clear();
            if reader.read_line(&mut header_line).is_err() {
                return Err("Error reading header from stream");
            }
            let header = header_line.trim_end();
            if header.is_empty() {
                break; // End of headers, the rest is the body (which we're ignoring for now)
            }
            let parts: Vec<&str> = header.splitn(2, ": ").collect();
            if parts.len() != 2 {
                return Err("Invalid Request: Malformed header");
            }
            headers.insert(parts[0].to_string(), parts[1].to_string());
        }
        let mut body = Vec::new();
        if let Some(content_length) = headers.get("Content-Length") {
            let length: usize = content_length
                .parse()
                .map_err(|_| "Invalid Content-Length header")?;
            if reader.take(length as u64).read_to_end(&mut body).is_err() {
                return Err("Error reading body from stream");
            }
        }
        Ok(Request {
            status_line,
            headers,
            body,
        })
    }
}
