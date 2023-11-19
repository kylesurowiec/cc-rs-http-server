#[derive(Debug)]
pub enum Method {
    Get,
    Patch,
    Post,
    Put,
    Delete,
    Unknown,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "PATCH" => Method::Patch,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            _ => Method::Unknown,
        }
    }
}
