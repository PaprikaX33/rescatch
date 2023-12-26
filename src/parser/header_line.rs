pub struct HttpHeaderLine {
    method: String,
    uri: String,
    version: String,
}
impl std::fmt::Debug for HttpHeaderLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpHeaderLine")
            .field("Method", &self.method)
            .field("URI", &self.uri)
            .field("version", &self.version)
            .finish()
    }
}

impl std::fmt::Display for HttpHeaderLine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} HTTP/{}", self.method, self.uri, self.version)
    }
}
