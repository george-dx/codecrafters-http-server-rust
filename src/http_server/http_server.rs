use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use itertools::Itertools;

const OK_RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_RESPONSE: &[u8] = b"HTTP/1.1 404 Not Found\r\n\r\n";

pub struct HttpServer {
    address: String,
    port: u16,
}

impl HttpServer {
    pub fn new(address: String, port: u16) -> Self {
        Self { address, port }
    }

    pub fn listen(&self) {
        let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("accepted new connection");
                    self.handle_connection(stream)
                }
                Err(e) => {
                    println!("Error on tcp stream: {}", e);
                }
            }
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 4096];
        let _request_bytes = stream.read(&mut buffer).unwrap();
        let request_str = String::from_utf8_lossy(&buffer);
        let request_lines: Vec<&str> = request_str.split("\r\n").collect_vec();
        println!("Requested lines: {:?}", request_lines);
        let start_line = request_lines.get(0).unwrap_or(&"Missing starting line");
        let start_line_parts: Vec<&str> = start_line.split(' ').collect();
        let path = start_line_parts
            .get(1)
            .unwrap_or(&"Missing start part get path");

        let parts: Vec<&str> = path.split("/echo/").collect();
        println!("{:?}", parts);
        match path {
            &"/" => match stream.write(OK_RESPONSE) {
                Ok(_) => (),
                Err(e) => println!("Error on ok response: {}", e),
            },
            path if path.starts_with("/echo/") => {
                let echo_message_parts: Vec<&str> = path.split("/echo/").collect();
                let echo_message = echo_message_parts.get(1).unwrap();
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    echo_message.len(),
                    echo_message
                );
                let _ = stream.write(response.as_bytes()).unwrap();
            }
            _ => match stream.write(NOT_FOUND_RESPONSE) {
                Ok(_) => (),
                Err(e) => println!("Error on not found response: {}", e),
            },
        }
    }
}
