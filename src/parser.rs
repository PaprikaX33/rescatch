//use regex::Regex;
mod error;
pub mod header_line;
pub mod header_pair;
pub mod traits;
use error::HttpRequestMessageErr;
use header_line::HttpHeaderLine;
use header_pair::HttpHeaderPair;
use std::io::BufRead;
pub use traits::FromBuf;
pub struct HttpRequestMessage {
    request_line: HttpHeaderLine,
    headers: std::vec::Vec<HttpHeaderPair>,
    body: String,
}

impl std::fmt::Debug for HttpRequestMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpRequestMessage")
            .field("Request", &self.request_line)
            .field("Headers", &self.headers)
            .field("Body", &self.body)
            .finish()
    }
}

impl std::fmt::Display for HttpRequestMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n", self.request_line)?;
        for pair in &self.headers {
            write!(f, "{}\n", pair)?;
        }
        write!(f, "\n{}", self.body)
    }
}

impl HttpRequestMessage {
    fn new() -> Self {
        HttpRequestMessage {
            request_line: HttpHeaderLine::new(),
            headers: Vec::new(),
            body: String::new(),
        }
    }
}

impl std::default::Default for HttpRequestMessage {
    fn default() -> Self {
        return Self::new();
    }
}

impl FromBuf for HttpRequestMessage {
    type Err = HttpRequestMessageErr;
    fn from_buf<T>(s: &mut T) -> Result<Self, Self::Err>
    where
        T: BufRead,
    {
        let line_tok = b'\n';
        let mut act_buf = Vec::<u8>::new();
        let head_read = s.read_until(line_tok, &mut act_buf)?;
        if head_read == 0 {
            // End of input
            return Ok(HttpRequestMessage::new());
        }
        let head_line = std::str::from_utf8(&act_buf)?.parse::<HttpHeaderLine>()?;
        println!("{}", head_line);
        todo!()
    }
}
