//! This file can be deleted after transfering the parser out
use regex::Regex;
use std::fmt::{Debug, Display};
pub struct HttpHeaderPair {
    key: String,
    val: String,
}

impl Debug for HttpHeaderPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpHeaderPair")
            .field("key", &self.key)
            .field("val", &self.val)
            .finish()
    }
}

impl Display for HttpHeaderPair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.key, self.val)
    }
}
impl Into<(String, String)> for HttpHeaderPair {
    // Required method
    fn into(self) -> (String, String) {
        return (self.key, self.val);
    }
}
impl HttpHeaderPair {
    pub fn parse_header_line(s: &str) -> Result<Self, ()> {
        let rex =
            Regex::new(r"(?ui)^(?P<key>[\w\-!#$%&'*+.^_`|~]+)\s*:\s*(?P<val>[^\r\n]+)\r?\n?$")
                .expect(&format!("Unable to compile the regex in {}", file!()));
        if let Some(captures) = rex.captures(s) {
            let extractor = |name| captures.name(name).ok_or(());
            let key = extractor("key")?.as_str().to_string();
            let val = extractor("val")?.as_str().to_string();
            Ok(Self { key, val })
        } else {
            Err(())
        }
    }
}
