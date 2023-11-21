use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buf: Vec<u8> = Vec::new();
                let _request_bytes = stream.read_to_end(&mut buf);

                let response: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";
                stream.write(response);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
