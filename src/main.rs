mod parser;

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Failed to bind TcpListener");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection");

                let mut buf = [0; 1024];

                stream.read(&mut buf).expect("Failed to read stream");

                println!("{:#?}", buf.make_ascii_lowercase());

                let _raw = parser::RawHttpRequest::new(buf.to_ascii_lowercase());

                println!("made it here");

                let _ = stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
