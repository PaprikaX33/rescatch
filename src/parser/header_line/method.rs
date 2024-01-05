use super::error::HttpHeaderLineErr;

#[derive(Clone)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
}

impl HttpMethod {
    pub fn new() -> Self {
        return HttpMethod::GET;
    }
    pub fn has_body(&self) -> bool {
        match self {
            Self::GET => false,
            Self::HEAD => false,
            Self::POST => true,
            Self::PUT => true,
            Self::DELETE => false,
        }
    }
}
impl std::fmt::Debug for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}
impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::GET => "GET",
                Self::HEAD => "HEAD",
                Self::POST => "POST",
                Self::PUT => "PUT",
                Self::DELETE => "DELETE",
            }
        )
    }
}

impl std::default::Default for HttpMethod {
    fn default() -> Self {
        return Self::new();
    }
}

impl std::str::FromStr for HttpMethod {
    type Err = HttpHeaderLineErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let st = s.to_uppercase();
        Ok(match st.as_str() {
            "GET" => Self::GET,
            "HEAD" => Self::HEAD,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            _ => return Err(HttpHeaderLineErr::InvalidMethod(s.to_string())),
        })
    }
}
