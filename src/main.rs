mod parser;

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Failed to bind TcpListener");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection");

                let mut buffer = vec![];
                stream.read(&mut buffer).unwrap();

                let buffer_as_string = String::from_utf8_lossy(&buffer);
                println!("{:#?}", buffer_as_string);

                //let _raw = parser::RawHttpRequest::new(buffer.to_ascii_lowercase());

                println!("made it here");

                let _ = stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
