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
