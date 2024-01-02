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
