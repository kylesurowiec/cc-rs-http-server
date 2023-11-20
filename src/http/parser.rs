use std::io::BufReader;
use std::net::TcpStream;

use anyhow::Result;

use crate::http::{Header, StatusLine};

#[derive(Debug)]
pub struct RawHttpRequest {
    pub status_line: StatusLine,
    pub headers: Vec<Header>,
}

impl RawHttpRequest {
    pub fn parse(mut buffer: BufReader<&mut TcpStream>) -> Result<RawHttpRequest> {
        // First line of request is always the status line
        let status_line = StatusLine::from_buffer(&mut buffer)?;
        // Every line under the status line is considered
        // a header until an empty line (\r\n) is reached
        let headers = Header::parse_all_from_buffer(&mut buffer)?;

        Ok(RawHttpRequest {
            status_line,
            headers,
        })
    }
}
