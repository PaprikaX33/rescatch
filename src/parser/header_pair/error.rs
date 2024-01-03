pub type Text = String;
pub type Token = String;
pub enum HttpHeaderPairErr {
    InvalidToken(Token, Text),
    InvalidLine(Text),
}

impl std::error::Error for HttpHeaderPairErr {}
impl std::fmt::Debug for HttpHeaderPairErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken(tok, text) => {
                write!(f, "Unknown Header line of {} from {}", tok, text)
            }
            Self::InvalidLine(text) => write!(f, "Invalid Header argument of: {}", text),
        }
    }
}
impl std::fmt::Display for HttpHeaderPairErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(&self, f)
    }
}
