use std::fmt::{Display, Formatter, Result};

use bytes::BufMut;

use crate::http::*;

pub const HTTP_VERSION_1_1: &str = "HTTP/1.1";

#[derive(Debug, Default)]
pub enum ContentType {
    #[allow(dead_code)]
    Html,
    #[allow(dead_code)]
    Json,
    #[default]
    Text,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let content_type_value = match self {
            ContentType::Html => CONTENT_TYPE_VALUE_HTML,
            ContentType::Json => CONTENT_TYPE_VALUE_JSON,
            ContentType::Text => CONTENT_TYPE_VALUE_TEXT,
        };
        write!(f, "{}: {}\r\n", CONTENT_TYPE_KEY, content_type_value)
    }
}

#[derive(Debug, Default)]
pub struct Message {
    #[allow(dead_code)]
    status_line: String,
    pub body: Option<String>,
    pub headers: Vec<String>,
    pub content_type: ContentType,
    pub status_code: StatusCode,
    pub status_text: String,
    pub version: String,
}

impl Message {
    pub fn new() -> Self {
        Message::default()
    }

    pub fn status_code(&mut self, status_code: StatusCode) -> &mut Self {
        self.status_code = status_code;
        self
    }

    pub fn content_type(&mut self, content_type: ContentType) -> &mut Self {
        self.content_type = content_type;
        self
    }

    pub fn body(&mut self, body: String) -> &mut Self {
        self.body = Some(body);
        self
    }

    // TODO: Pass buffer to helper methods for content-length and body
    pub fn build(&self) -> Vec<u8> {
        let mut buffer = vec![];

        // Status line
        buffer.put(format!("{} {}\r\n", HTTP_VERSION_1_1, self.status_code).as_bytes());
        // Content-Type header
        buffer.put(self.content_type.to_string().as_bytes());
        // Content-Length header and body (if applicable)
        let (content_length_header, body) = self.get_body();
        buffer.put(content_length_header.as_bytes());
        buffer.put(body.as_bytes());

        buffer
    }

    fn get_body(&self) -> (String, String) {
        match self.body {
            Some(ref body) => (
                format!("{}: {}\r\n\r\n", CONTENT_LENGTH_KEY, body.len()),
                String::from(body),
            ),
            None => (
                format!("{}: {}\r\n\r\n", CONTENT_LENGTH_KEY, 0),
                String::from(""),
            ),
        }
    }
}
