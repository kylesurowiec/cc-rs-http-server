mod http;
mod json;

use std::io::{BufReader, Write};
use std::net::TcpListener;

use anyhow::Result;

use crate::http::{ContentType, Message, RawHttpRequest, StatusCode};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Failed to bind TcpListener");
    println!("Server stated - listening on :4221");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection");

                let buffer = BufReader::new(&mut stream);
                let req = RawHttpRequest::parse(buffer)?;

                println!("{:#?}", req);

                if req.status_line.path == "/" {
                    let res = Message::new().status_code(StatusCode::Ok).build();
                    stream.write(&res)?;
                    return Ok(());
                }

                if req.status_line.path.contains("/echo/") {
                    let message = req
                        .status_line
                        .path
                        .split("/echo/")
                        .collect::<Vec<_>>()
                        .join("");

                    let res = Message::new()
                        .status_code(StatusCode::Ok)
                        .content_type(ContentType::Text)
                        .body(message.to_string())
                        .build();

                    stream.write(&res)?;

                    return Ok(());
                }

                if req.status_line.path.contains("/user-agent") {
                    let message = req
                        .status_line
                        .path
                        .split("/user-agent")
                        .collect::<Vec<_>>()
                        .join("");

                    let res = Message::new()
                        .status_code(StatusCode::Ok)
                        .content_type(ContentType::Text)
                        .body(message.to_string())
                        .build();

                    stream.write(&res)?;

                    return Ok(());
                }

                let res = Message::new()
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
