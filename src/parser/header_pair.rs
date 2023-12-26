use std::fmt::{Debug, Display};
//#[derive(Debug)]
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
