use anyhow::Result;

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    #[allow(dead_code)]
    Patch,
    #[allow(dead_code)]
    Post,
    #[allow(dead_code)]
    Put,
    #[allow(dead_code)]
    Delete,
}

#[derive(Debug)]
pub struct Header {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct RawHttpRequest {
    pub status_line: String,
    pub path: String,
    pub method: HttpMethod,
    pub version: String,
    pub headers: Vec<Header>,
}

impl RawHttpRequest {
    pub fn parse(buffer: &[u8]) -> Result<Self> {
        let buf_as_str = String::from_utf8_lossy(buffer);

        let status_line = Self::parse_status_line(&buf_as_str)?;
        let path = Self::parse_path(&status_line)?;
        let version = Self::parse_version(&status_line)?;

        Ok(Self {
            status_line: status_line.to_string(),
            path: path.to_string(),
            method: HttpMethod::Get,
            version: version.to_string(),
            headers: vec![Header {
                key: "test".to_string(),
                value: "test".to_string(),
            }],
        })
    }

    fn parse_status_line(raw_req: &str) -> Result<&str> {
        raw_req
            .split("\r\n")
            .nth(0)
            .ok_or(anyhow::Error::msg("Failed to parse status line"))
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
