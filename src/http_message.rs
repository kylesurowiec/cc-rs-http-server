use std::fmt::{Display, Formatter, Result};

use bytes::BufMut;

use crate::status_code::StatusCode;

pub const HTTP_VERSION_1_1: &str = "HTTP/1.1";
pub const CONTENT_TYPE_HTML: &str = "Content-Type: text/html";
pub const CONTENT_TYPE_JSON: &str = "Content-Type: text/json";
pub const CONTENT_TYPE_TEXT: &str = "Content-Type: text/plain";

#[derive(Debug, Default)]
pub enum ContentType {
    Html,
    Json,
    #[default]
    Text,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let header_text = match self {
            ContentType::Html => CONTENT_TYPE_HTML,
            ContentType::Json => CONTENT_TYPE_JSON,
            ContentType::Text => CONTENT_TYPE_TEXT,
        };
        write!(f, "{}", header_text)
    }
}

#[derive(Debug, Default)]
pub struct HttpMessage {
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
        // Headers
        buffer.put(format!("{}\r\n\r\n", self.content_type).as_bytes());
        // Body
        buffer.put(format!("{}", self.body).as_bytes());

        buffer
    }
}
