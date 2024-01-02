pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct HttpHeaderLine {
    method: HttpMethod,
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
impl HttpHeaderLine {
    pub fn new() -> Self {
        HttpHeaderLine {
            uri: String::new(),
            method: HttpMethod::new(),
            version: String::new(),
        }
    }
}

impl HttpMethod {
    fn new() -> Self {
        return HttpMethod::GET;
    }
}
impl std::fmt::Debug for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}
impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                HttpMethod::GET => "GET",
                HttpMethod::POST => "POST",
                HttpMethod::PUT => "PUT",
                HttpMethod::DELETE => "DELETE",
            }
        )
    }
}
