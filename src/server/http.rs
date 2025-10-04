#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Delete => "DELETE",
            HttpMethod::Get => "GET",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
        }
    }
}

pub enum HttpStatus {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    UNAUTHORIZED = 401,
    InternalServerError = 500,
}

impl HttpStatus {
    pub(crate) fn code(self) -> u16 {
        self as u16
    }
}
