#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    Post,
    Get,
    Patch,
    Delete,
}

pub enum HttpStatus {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    MethodNotFound = 405,
    InternalServerError = 500,
}

impl HttpStatus {
    pub(crate) fn code(self) -> u16 {
        self as u16
    }
}
