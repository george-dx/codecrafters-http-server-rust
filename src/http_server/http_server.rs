use itertools::Itertools;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

const OK_RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_RESPONSE: &[u8] = b"HTTP/1.1 404 Not Found\r\n\r\n";

pub struct HttpServer {
    ip_addr: String,
    port: u16,
}

impl HttpServer {
    pub fn new(ip_addr: String, port: u16) -> Self {
        Self { ip_addr, port }
    }

    pub fn listen(&self) {
        let address = format!("{}:{}", self.ip_addr, self.port);
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("accepted new connection");
                    thread::spawn(move || handle_request(stream));
                }
                Err(e) => {
                    println!("Error on tcp stream: {}", e);
                }
            }
        }
    }

    fn _handle_request_deprecated(&self, mut stream: TcpStream) {
        let mut buffer = [0; 4096];
        let request_bytes = stream.read(&mut buffer).unwrap();
        let request_str = String::from_utf8_lossy(&buffer);
        let _request_lines: Vec<&str> = request_str.split("\r\n").collect_vec();
        println!("Requested lines: {:?}", request_bytes);
    }
}

fn handle_request(mut stream: TcpStream) {
    let buffer: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_bytes: Vec<String> = buffer
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect_vec();
    // println!("Requested lines: {:?}", request_bytes);
    handle_response(request_bytes, stream);
}

fn handle_response(request_lines: Vec<String>, mut stream: TcpStream) {
    let start_line: &String = request_lines.get(0).expect("Missing first line");
    let start_line_parts: Vec<&str> = start_line.split(' ').collect();
    let path = start_line_parts
        .get(1)
        .unwrap_or(&"Missing start part get path");

    let parts: Vec<&str> = path.split("/echo/").collect();
    println!("{:?}", parts);
    match path {
        &"/" => match stream.write_all(OK_RESPONSE) {
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
            let _ = stream.write_all(response.as_bytes()).unwrap();
        }
        path if path.starts_with("/user-agent") => {
            let mut user_agent_line = String::new();
            for element in request_lines {
                if element.contains(&"User-Agent") {
                    user_agent_line = element.to_string();
                }
            }
            let user_agent: Vec<&str> = user_agent_line.split(": ").collect();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n",
                user_agent.get(1).expect("Missing user agent").len(),
                user_agent.get(1).expect("Missing user agent")
            );
            let _ = stream.write_all(response.as_bytes()).unwrap();
        }
        _ => match stream.write_all(NOT_FOUND_RESPONSE) {
            Ok(_) => (),
            Err(e) => println!("Error on not found response: {}", e),
        },
    }
}
