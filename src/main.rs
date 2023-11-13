mod http_message;
mod parser;
mod status_code;

use std::io::{Read, Write};
use std::net::TcpListener;

use anyhow::Result;

use crate::http_message::{ContentType, HttpMessage};
use crate::parser::RawHttpRequest;
use crate::status_code::StatusCode;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Failed to bind TcpListener");
    println!("Server stated - listening on :4221");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection");

                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();

                let req = RawHttpRequest::parse(&buffer)?;

                println!("{:#?}", req);

                if req.path == "/" {
                    let res = HttpMessage::new().status_code(StatusCode::Ok).build();
                    stream.write(&res)?;
                    return Ok(());
                }

                if req.path.contains("/echo/") {
                    let message = req.path.split("/echo/").collect::<Vec<&str>>().join("");
                    let res = HttpMessage::new()
                        .status_code(StatusCode::Ok)
                        .content_type(ContentType::Text)
                        .body(message.to_string())
                        .build();

                    stream.write(&res)?;

                    return Ok(());
                }

                let res = HttpMessage::new()
                    .status_code(StatusCode::NotFound)
                    .content_type(ContentType::Text)
                    .build();

                stream.write(&res)?;

                return Ok(());
            }
            Err(e) => {
                println!("error: {}", e)
            }
        }
    }

    Ok(())
}
