use super::header_line::HttpHeaderLineErr;

pub enum HttpRequestMessageErr {
    HeaderLine(HttpHeaderLineErr),
    UTFErr(std::str::Utf8Error),
    IOErr(std::io::Error),
}

impl From<std::str::Utf8Error> for HttpRequestMessageErr {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::UTFErr(err)
    }
}

impl From<HttpHeaderLineErr> for HttpRequestMessageErr {
    fn from(err: HttpHeaderLineErr) -> Self {
        Self::HeaderLine(err)
    }
}
impl From<std::io::Error> for HttpRequestMessageErr {
    fn from(err: std::io::Error) -> Self {
        Self::IOErr(err)
    }
}
impl std::error::Error for HttpRequestMessageErr {}
impl std::fmt::Debug for HttpRequestMessageErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HeaderLine(header) => <HttpHeaderLineErr as std::fmt::Debug>::fmt(header, f),
            Self::UTFErr(utf_err) => write!(f, "Invalid UTF-8 sequence detected as {:?}", utf_err),
            Self::IOErr(io_err) => write!(f, "IO Error: {:?}", io_err),
        }
    }
}
impl std::fmt::Display for HttpRequestMessageErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //write!(f, "Parsing HTTP error. Malformed http header: {}", self.msg)
        match self {
            Self::HeaderLine(header) => <HttpHeaderLineErr as std::fmt::Display>::fmt(header, f),
            Self::UTFErr(utf_err) => write!(f, "Invalid UTF-8 sequence detected as {}", utf_err),
            Self::IOErr(io_err) => write!(f, "IO Error: {}", io_err),
        }
    }
}
