pub struct Header {
    pub key: String,
    pub value: String,
}

pub struct RawHttpRequest {
    pub start_line: String,
    pub path: String,
    pub http_method: String,
    pub http_version: String,
    pub headers: Vec<Header>,
}

impl RawHttpRequest {
    pub fn new(_req: Vec<u8>) -> Self {
        let header = Header {
            key: "test".to_string(),
            value: "test".to_string(),
        };
        Self {
            start_line: "".to_string(),
            path: "".to_string(),
            http_method: "".to_string(),
            http_version: "".to_string(),
            headers: vec![header],
        }
    }
}
