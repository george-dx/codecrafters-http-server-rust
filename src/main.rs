use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buf: Vec<u8> = Vec::new();
    let _request_bytes = stream.read(&mut buf);
    
    match stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n") {
        Ok(_) => (),
        Err(e) => println!("error: {}", e),
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
                println!("error: {}", e);
            }
        }
    }
}
