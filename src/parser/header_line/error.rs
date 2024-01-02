pub type Text = String;
pub type Element = String;

pub enum HttpHeaderLineErr {
    RegexErr(regex::Error),
    InvalidHeader(Text),
    MissingInfo(Element, Text),
}

impl From<regex::Error> for HttpHeaderLineErr {
    fn from(err: regex::Error) -> Self {
        Self::RegexErr(err)
    }
}

impl std::error::Error for HttpHeaderLineErr {}
impl std::fmt::Debug for HttpHeaderLineErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RegexErr(rex_err) => write!(f, "Regex error as {:?}", rex_err),
            Self::InvalidHeader(text) => write!(f, "Invalid HTTP Header {}", text),
            Self::MissingInfo(elem, text) => write!(
                f,
                "Missing element of {} from HTTP header of {}",
                elem, text
            ),
        }
    }
}
impl std::fmt::Display for HttpHeaderLineErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //write!(f, "Parsing HTTP error. Malformed http header: {}", self.msg)
        match self {
            Self::RegexErr(rex_err) => write!(f, "Regex error as {}", rex_err),
            Self::InvalidHeader(text) => write!(f, "Invalid HTTP Header {}", text),
            Self::MissingInfo(elem, text) => write!(
                f,
                "Missing element of {} from HTTP header of {}",
                elem, text
            ),
        }
    }
}
