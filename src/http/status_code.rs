use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Default)]
pub enum StatusCode {
    #[default]
    Ok,
    Created,
    Accepted,
    NoContent,
    BadRequest,
    NotFound,
}

impl From<u16> for StatusCode {
    fn from(value: u16) -> Self {
        match value {
            200 => StatusCode::Ok,
            201 => StatusCode::Created,
            202 => StatusCode::Accepted,
            204 => StatusCode::NoContent,
            400 => StatusCode::BadRequest,
            404 => StatusCode::NotFound,
            _ => unimplemented!(),
        }
    }
}

impl From<&StatusCode> for u16 {
    fn from(value: &StatusCode) -> Self {
        match value {
            StatusCode::Ok => 200,
            StatusCode::Created => 201,
            StatusCode::Accepted => 202,
            StatusCode::NoContent => 204,
            StatusCode::BadRequest => 400,
            StatusCode::NotFound => 404,
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let code = self.as_u16();
        let text = self.text();

        write!(f, "{} {}", code, text)
    }
}

impl StatusCode {
    pub fn new(code: u16) -> Self {
        StatusCode::from(code)
    }

    pub fn text(&self) -> String {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::Created => "CREATED",
            StatusCode::Accepted => "ACCEPTED",
            StatusCode::NoContent => "NO CONTENT",
            StatusCode::BadRequest => "BAD REQUEST",
            StatusCode::NotFound => "NOT FOUND",
        }
        .to_string()
    }

    pub fn as_u16(&self) -> u16 {
        self.into()
    }
}
