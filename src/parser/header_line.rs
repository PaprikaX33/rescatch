use regex::Regex;
mod error;
pub use error::HttpHeaderLineErr;
mod method;
pub use method::HttpMethod;

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
impl std::default::Default for HttpHeaderLine {
    fn default() -> Self {
        return Self::new();
    }
}
impl std::str::FromStr for HttpHeaderLine {
    type Err = HttpHeaderLineErr;
    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rex = Regex::new(r"^(?P<method>\w+) (?P<path>[/\w]*) (?P<version>HTTP/\d+\.\d+)\r?$")?;
        if let Some(captures) = rex.captures(s) {
            let extractor = |name| {
                captures
                    .name(name)
                    .ok_or::<Self::Err>(HttpHeaderLineErr::MissingInfo(
                        name.to_string(),
                        s.to_string(),
                    ))
            };
            let method = extractor("method")?.as_str().to_string();
            let uri = extractor("path")?.as_str().to_string();
            let version = extractor("version")?.as_str().to_string();
            Ok(Self {
                method,
                uri,
                version,
            })
        } else {
            Err(HttpHeaderLineErr::InvalidHeader(s.to_string()))
        }
    }
}
