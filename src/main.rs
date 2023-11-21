use std::borrow::Cow;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

const OK_RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_RESPONSE: &[u8] = b"HTTP/1.1 404 Not Found\r\n\r\n";

fn handle_connection(mut stream: TcpStream) {
    let buf: Vec<u8> = Vec::new();
    let request_str = String::from_utf8_lossy(&buf);
    let request_lines: Vec<&str> = request_str.split("\r\n").collect();
    let start_line = request_lines.get(0).unwrap_or(&"Missing starting line");
    let start_line_parts: Vec<&str> = start_line.split(" ").collect();
    let path = start_line_parts.get(1).unwrap_or(&"Missing start part get path");

    match path {
        &"/" => match stream.write_all(OK_RESPONSE) {
            Ok(_) => (),
            Err(e) => println!("Error on ok response: {}", e),
        },
        _ => match stream.write_all(NOT_FOUND_RESPONSE) {
            Ok(_) => (),
            Err(e) => println!("Error on not found response: {}", e),
        },
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream)
            }
            Err(e) => {
                println!("Error on tcp stream: {}", e);
            }
        }
    }
}
