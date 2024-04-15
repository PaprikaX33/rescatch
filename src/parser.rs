//! Parsing module for the HTTP protocol message

use regex::Regex;
mod error;
pub mod header_line;
//pub mod header_pair;
pub mod traits;
use error::HttpRequestMessageErr;
use header_line::HttpHeaderLine;
pub use header_line::HttpMethod;
use std::collections::HashMap;
use std::io::BufRead;
pub use traits::FromBuf;

/// Default type for the header argument, considering that in http, the header argument
/// is in the structure of key:value
type HeaderArgument = HashMap<String, String>;

pub struct HttpRequestMessage {
    request_line: HttpHeaderLine,
    headers: HeaderArgument,
    body: std::vec::Vec<u8>,
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
        for (key, val) in &self.headers {
            write!(f, "{}: {}\n", key, val)?;
        }
        match std::str::from_utf8(self.body.as_slice()) {
            Ok(textural) => write!(f, "\n{}", textural),
            Err(_) => write!(
                f,
                "\n{}",
                self.body
                    .iter()
                    .map(|val| format!("{:02X}", val))
                    .collect::<String>()
            ),
        }
    }
}

impl HttpRequestMessage {
    fn new() -> Self {
        HttpRequestMessage {
            request_line: HttpHeaderLine::new(),
            headers: HeaderArgument::new(),
            body: Vec::new(),
        }
    }
}

impl std::default::Default for HttpRequestMessage {
    fn default() -> Self {
        return Self::new();
    }
}

fn parse_header_argument_by_line(s: &str) -> Result<(&str, &str), ()> {
    let rex = Regex::new(r"(?ui)^(?P<key>[\w\-!#$%&'*+.^_`|~]+)\s*:\s*(?P<val>[^\r\n]+)\r?\n?$")
        .expect(&format!("Unable to compile the regex in {}", file!()));
    if let Some(captures) = rex.captures(s) {
        let extractor = |name| captures.name(name).ok_or(());
        return Ok((extractor("key")?.as_str(), extractor("val")?.as_str()));
    } else {
        Err(())
    }
}

impl FromBuf for HttpRequestMessage {
    type Err = HttpRequestMessageErr;
    fn from_buf<T>(s: &mut T) -> Result<Self, Self::Err>
    where
        T: BufRead,
    {
        let end_rex = Regex::new(r"(?ui)^\r?\n?$")
            .expect(&format!("Unable to compile the regex in {}", file!()));
        let line_tok = b'\n';
        let mut act_buf = Vec::<u8>::new();
        let head_read = s.read_until(line_tok, &mut act_buf)?;
        if head_read == 0 {
            // End of input
            return Ok(HttpRequestMessage::new());
        }
        let head_line = std::str::from_utf8(&act_buf)?.parse::<HttpHeaderLine>()?;
        let mut head_arg: HeaderArgument = HeaderArgument::new();
        loop {
            act_buf.clear();
            let arg_read = s.read_until(line_tok, &mut act_buf)?;
            if arg_read == 0 {
                break;
            }
            let arg_line = std::str::from_utf8(&act_buf)?;
            if end_rex.is_match(arg_line) {
                // No more header
                break;
            }
            if let Ok(result) = parse_header_argument_by_line(arg_line) {
                let (key, val) = result;
                head_arg.insert(key.to_string(), val.to_string());
            } else {
                break;
            }
        }
        //Body or bodyless
        if !head_line.method().has_body() {
            return Ok(Self {
                request_line: head_line,
                headers: head_arg,
                body: Vec::new(),
            });
        } else {
            loop {
                let chunk = s.fill_buf()?;
                let len = chunk.len();

                if len == 0 {
                    // Buffer is empty, break out of the loop
                    break;
                }
                // Extend the data vector with the read data
                act_buf.extend_from_slice(chunk);
                // Consume the data read from the buffer
                s.consume(len);
            }
        }
        println!("{:?}", head_arg);
        return Ok(Self {
            request_line: head_line,
            headers: head_arg,
            body: act_buf,
        });
    }
}
