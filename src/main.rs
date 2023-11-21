use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};

use itertools::Itertools;

const OK_RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_RESPONSE: &[u8] = b"HTTP/1.1 404 Not Found\r\n\r\n";

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 4096];
    let _request_bytes = stream.read(&mut buffer).unwrap();
    let request_str = String::from_utf8_lossy(&buffer);
    let request_lines: Vec<&str> = request_str.split("\r\n").collect_vec();
    println!("Requested lines: {:?}", request_lines);
    let start_line = request_lines.get(0).unwrap_or(&"Missing starting line");
    let start_line_parts: Vec<&str> = start_line.split(' ').collect();
    let path = start_line_parts.get(1).unwrap_or(&"Missing start part get path");

    match path {
        &"/" => match stream.write(OK_RESPONSE) {
            Ok(_) => (),
            Err(e) => println!("Error on ok response: {}", e),
        },
        _ => match stream.write(NOT_FOUND_RESPONSE) {
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
