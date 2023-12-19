use crate::error;
use regex::Regex;
use std::str::FromStr;

pub struct HeaderPair {
    key: String,
    val: String,
}
pub struct HttpRequestHeader {
    method: String,
    uri: String,
    version: String,
    header: std::vec::Vec<HeaderPair>,
}
impl std::fmt::Debug for HttpRequestHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpRequestHeader")
            .field("Method", &self.method)
            .field("URI", &self.uri)
            .field("version", &self.version)
            .finish()
    }
}

impl std::fmt::Display for HttpRequestHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} HTTP/{}", self.method, self.uri, self.version)
    }
}

impl FromStr for HttpRequestHeader {
    type Err = error::HttpRequestError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Define a regular expression to parse HTTP requests
        let re = Regex::new(r"^(?P<method>\w+) (?P<path>[/\w]+) (?P<version>HTTP/\d+\.\d+)$")
            .expect("Error in regex creation");
        // Use the regex to capture parts of the input
        if let Some(captures) = re.captures(s) {
            let extractor = |name| captures.name(name).ok_or::<Self::Err>(s.to_string().into());
            let method = extractor("method")?.as_str().to_string();
            let uri = extractor("path")?.as_str().to_string();
            let version = extractor("version")?.as_str().to_string();
            let rest = extractor("rest")?.as_str().to_string();

            Ok(HttpRequestHeader {
                method,
                uri,
                version,
                header: std::vec::Vec::new(),
            })
        } else {
            Err(s.to_string().into())
        }
    }
}
