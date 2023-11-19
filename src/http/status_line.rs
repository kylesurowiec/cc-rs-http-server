use std::io::{BufRead, BufReader};
use std::net::TcpStream;

use anyhow::Result;

use crate::http::Method;

#[derive(Debug)]
pub struct StatusLine {
    pub method: Method,
    pub version: String,
    pub path: String,
}

impl StatusLine {
    pub fn from_buffer(buffer: &mut BufReader<&mut TcpStream>) -> Result<StatusLine> {
        let mut status_line = String::new();
        buffer.read_line(&mut status_line)?;

        Ok(StatusLine {
            method: Self::parse_method(&status_line)?,
            version: Self::parse_version(&status_line)?.to_string(),
            path: Self::parse_path(&status_line)?.to_string(),
        })
    }

    fn parse_method(start_line: &str) -> Result<Method> {
        Ok(Method::from(
            start_line
                .split_whitespace()
                .nth(0)
                .ok_or(anyhow::Error::msg("Failed to parse method"))?,
        ))
    }

    fn parse_path(start_line: &str) -> Result<&str> {
        start_line
            .split_whitespace()
            .nth(1)
            .ok_or(anyhow::Error::msg("Failed to parse path"))
    }

    fn parse_version(start_line: &str) -> Result<&str> {
        start_line
            .split_whitespace()
            .nth(2)
            .ok_or(anyhow::Error::msg("Failed to parse version"))
    }
}
