mod parser;

use std::io::{Read, Write};
use std::net::TcpListener;

use anyhow::Result;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Failed to bind TcpListener");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection");

                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();

                let raw_http = parser::RawHttpRequest::new(&buffer)?;

                if raw_http.path == "/" {
                    let _ = stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
                } else {
                    let _ = stream.write(b"HTTP/1.1 404 NOT FOUND\r\n\r\n");
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
