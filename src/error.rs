pub struct HttpRequestError(String);
impl std::error::Error for HttpRequestError {}

impl std::fmt::Display for HttpRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parsing HTTP error. Malformed http header: {}", self)
    }
}

impl std::fmt::Debug for HttpRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Http Request Error:: ")
            .field("Malformed header", self)
            .finish()
    }
}
