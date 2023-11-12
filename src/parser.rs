use anyhow::Result;

pub enum HttpMethod {
    Get,
    Patch,
    Post,
    Put,
    Delete,
}

pub struct Header {
    pub key: String,
    pub value: String,
}

pub struct RawHttpRequest {
    pub start_line: String,
    pub path: String,
    pub method: HttpMethod,
    pub version: String,
    pub headers: Vec<Header>,
}

impl RawHttpRequest {
    pub fn new(buffer: &[u8]) -> Result<Self> {
        let buf_as_string = String::from_utf8_lossy(buffer).to_string();

        let start_line = RawHttpRequest::parse_start_line(buf_as_string)?;
        let path = RawHttpRequest::parse_path(start_line.clone())?;
        let version = RawHttpRequest::parse_version(start_line.clone())?;

        println!("{:#?}", version);

        let header = Header {
            key: "test".to_string(),
            value: "test".to_string(),
        };

        Ok(Self {
            start_line,
            path,
            method: HttpMethod::Get,
            version,
            headers: vec![header],
        })
    }

    fn parse_start_line(raw_req: String) -> Result<String> {
        let start_line = raw_req.split("\r\n").nth(0);

        match start_line {
            Some(start_line) => Ok(start_line.to_string()),
            None => Err(anyhow::Error::msg("Failed to parse start line")),
        }
    }

    fn parse_path(start_line: String) -> Result<String> {
        let path = start_line.split_whitespace().nth(1);

        match path {
            Some(path) => Ok(path.to_string()),
            None => Err(anyhow::Error::msg("Failed to parse path")),
        }
    }

    fn parse_version(start_line: String) -> Result<String> {
        let version = start_line.split_whitespace().nth(2);

        match version {
            Some(version) => Ok(version.to_string()),
            None => Err(anyhow::Error::msg("Failed to parse path")),
        }
    }
}
