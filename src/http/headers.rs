use std::io::{BufRead, BufReader};
use std::net::TcpStream;

use anyhow::Result;

pub const CONTENT_TYPE_KEY: &str = "Content-Type";
pub const CONTENT_TYPE_VALUE_HTML: &str = "text/html";
pub const CONTENT_TYPE_VALUE_JSON: &str = "text/json";
pub const CONTENT_TYPE_VALUE_TEXT: &str = "text/plain";

pub const CONTENT_LENGTH_KEY: &str = "Content-Length";

#[derive(Debug)]
pub struct Header {
    pub key: String,
    pub value: String,
}

impl Header {
    pub fn parse_all_from_buffer(buffer: &mut BufReader<&mut TcpStream>) -> Result<Vec<Header>> {
        let mut headers: Vec<Header> = vec![];
        loop {
            let mut line = String::new();
            buffer.read_line(&mut line)?;
            // Empty line signals the end of headers
            // TODO: Need to account for malformed messages that
            // have no empty line
            if line == "\r\n" {
                break;
            }
            headers.push(Header::from_string(line)?);
        }

        Ok(headers)
    }

    pub fn from_string(buffer: String) -> Result<Header> {
        let (key, value) = buffer
            .split_once(':')
            .ok_or(anyhow::Error::msg("Failed to parse header"))?;

        Ok(Header {
            key: key.to_string(),
            value: value.to_string(),
        })
    }
}
