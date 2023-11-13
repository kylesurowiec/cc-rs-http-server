use std::{
    fmt::{Display, Formatter, Result},
    mem::size_of,
};

use bytes::BufMut;

use crate::status_code::StatusCode;

pub const HTTP_VERSION_1_1: &str = "HTTP/1.1";
pub const H_CONTENT_TYPE_HTML: &str = "Content-Type: text/html";
pub const H_CONTENT_TYPE_JSON: &str = "Content-Type: text/json";
pub const H_CONTENT_TYPE_TEXT: &str = "Content-Type: text/plain";
pub const H_CONTENT_LENGTH: &str = "Content-Length:";

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
        let header_text = match self {
            ContentType::Html => H_CONTENT_TYPE_HTML,
            ContentType::Json => H_CONTENT_TYPE_JSON,
            ContentType::Text => H_CONTENT_TYPE_TEXT,
        };
        write!(f, "{}", header_text)
    }
}

#[derive(Debug, Default)]
pub struct HttpMessage {
    #[allow(dead_code)]
    status_line: String,
    pub body: String,
    pub content_type: ContentType,
    pub status_code: StatusCode,
    pub status_text: String,
    pub version: String,
}

impl HttpMessage {
    pub fn new() -> Self {
        HttpMessage::default()
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
        self.body = body;
        self
    }

    pub fn build(&self) -> Vec<u8> {
        let mut buffer = vec![];

        // TODO: Support multiple headers
        // Status line
        buffer.put(format!("{} {}\r\n", HTTP_VERSION_1_1, self.status_code).as_bytes());
        // Content type header
        buffer.put(format!("{}\r\n", self.content_type).as_bytes());
        // Content length header (if applicable)
        let content_length = self.body.bytes().count();
        if content_length > 0 {
            buffer.put(format!("{} {}\r\n", H_CONTENT_LENGTH, self.body.len()).as_bytes());
        }
        // Body
        buffer.put(format!("\r\n{}", self.body).as_bytes());

        buffer
    }
}
