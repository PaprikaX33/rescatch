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

impl FromStr for HttpRequestHeader {
    type Err = HttpRequestError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Define a regular expression to parse HTTP requests
        let re = Regex::new(r"^(?P<method>\w+) (?P<path>[/\w]+) (?P<version>HTTP/\d+\.\d+)$")
            .expect("Error in regex creation");
        // Use the regex to capture parts of the input
        if let Some(captures) = re.captures(s) {
            let method = captures["method"].to_string();
            let uri = captures["path"].to_string();
            let version = captures["version"].to_string();

            Ok(HttpRequestHeader {
                method,
                uri,
                version,
                header: std::vec::Vec::new(),
            })
        } else {
            Err(HttpRequestError(s.to_string()))
        }
    }
}

/*fn main() {
    // Example usage
    let raw_request = "GET /index.html HTTP/1.1";
    match HttpRequest::from_str(raw_request) {
        Ok(request) => println!("{:?}", request),
        Err(()) => println!("Failed to parse HTTP request."),
    }
}
*/

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
