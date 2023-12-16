pub struct HttpRequestError {
    msg: String,
}
impl std::error::Error for HttpRequestError {}

impl std::fmt::Display for HttpRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parsing HTTP error. Malformed http header: {}", self.msg)
    }
}
impl From<String> for HttpRequestError {
    fn from(err: String) -> Self {
        HttpRequestError { msg: err }
    }
}
impl std::fmt::Debug for HttpRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Http Request Error:: ")
            .field("Malformed header", &self.msg)
            .finish()
    }
}

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
            /*ServerHandlerError::IoErr(err) => std::io::Error::fmt(err, f),
            ServerHandlerError::HttpRequestError(err) => HttpRequestError::fmt(err, f),*/
        }
    }
}

impl std::fmt::Debug for ServerHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerHandlerError::IoErr(err) => err.fmt(f),
            ServerHandlerError::HttpRequestError(err) => err.fmt(f),
            /*ServerHandlerError::IoErr(err) => std::io::Error::fmt(err, f),
            ServerHandlerError::HttpRequestError(err) => HttpRequestError::fmt(err, f),*/
        }
    }
}
