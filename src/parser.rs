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
    pub fn new(req: Vec<u8>) -> Self {}
}
