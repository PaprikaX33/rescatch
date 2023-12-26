use regex::Regex;
pub mod header_line;
pub mod header_pair;
pub mod traits;

use header_line::HttpHeaderLine;
use header_pair::HttpHeaderPair;
pub struct HttpRequestMessage {
    requestLine: HttpHeaderLine,
    headers: std::vec::Vec<HttpHeaderPair>,
    body: String,
}

impl std::fmt::Debug for HttpRequestMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpRequestMessage")
            .field("Request", &self.requestLine)
            .field("Headers", &self.headers)
            .field("Body", &self.body)
            .finish()
    }
}

impl std::fmt::Display for HttpRequestMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n", self.requestLine)?;
        for pair in self.headers {
            write!(f, "{}\n", pair)?;
        }
        write!(f, "\n{}", self.body)
    }
}
