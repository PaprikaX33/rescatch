use super::HttpRequestError;
pub enum ServerHandlerError {
    IoErr(std::io::Error),
    HttpRequestError(HttpRequestError),
}

impl From<std::io::Error> for ServerHandlerError {
    fn from(err: std::io::Error) -> Self {
        ServerHandlerError::IoErr(err)
    }
}
impl From<HttpRequestError> for ServerHandlerError {
    fn from(err: HttpRequestError) -> Self {
        ServerHandlerError::HttpRequestError(err)
    }
}
impl std::fmt::Display for ServerHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ServerHandlerError::IoErr(err) => err.fmt(f),
            ServerHandlerError::HttpRequestError(err) => err.fmt(f),
        }
    }
}

impl std::fmt::Debug for ServerHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerHandlerError::IoErr(err) => err.fmt(f),
            ServerHandlerError::HttpRequestError(err) => err.fmt(f),
        }
    }
}
