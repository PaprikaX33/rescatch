mod builder;
use super::HeaderArgument;
pub use builder::HttpResponseBuilder;

pub enum MessageBody {
    Str(String),
    Vec(std::vec::Vec<u8>),
}
pub enum HttpVersion {
    /// Basic
    /// as the basic of HTTP/1.0
    Basic,
    /// Extended
    /// as the extended of HTTP/1.1
    Extended,
}
/// The structure of the response
///
/// TODO : There is still no header in here
pub struct HttpResponse {
    code: u16,
    err_message: Option<String>,
    version: HttpVersion,
    body: Option<MessageBody>,
    /// Note : args will ignore Content-Length
    args: HeaderArgument,
}

impl HttpResponse {
    pub fn as_bytes(&self) -> std::vec::Vec<u8> {
        let mut header = Vec::<u8>::new();
        header.extend_from_slice(
            format!(
                "HTTP/{} {} {}\r\n",
                match self.version {
                    HttpVersion::Basic => "1.0",
                    HttpVersion::Extended => "1.1",
                },
                self.code,
                match &self.err_message {
                    Some(txt) => txt.as_str(),
                    None => match self.code {
                        400 => "Bad Request",
                        _ => "Unknown Code",
                    },
                },
            )
            .as_bytes(),
        );
        for (key, val) in &self.args {
            if key != "Content-Length" {
                header.extend_from_slice(format!("{key}: {val}\r\n").as_bytes());
            }
        }
        match &self.body {
            None => header.extend_from_slice("\r\n".as_bytes()),
            Some(body) => {
                let length = body.len();
                header.extend_from_slice(format!("Content-Length: {length}\r\n\r\n").as_bytes());
                header.extend_from_slice(body.as_bytes());
            }
        };
        return header;
    }
    fn builder(
        code: u16,
        err_message: Option<String>,
        version: HttpVersion,
        body: Option<MessageBody>,
        args: HeaderArgument,
    ) -> HttpResponse {
        return HttpResponse {
            code,
            version,
            args,
            body,
            err_message,
        };
    }
}

impl MessageBody {
    fn as_bytes(&self) -> &[u8] {
        match self {
            MessageBody::Str(txt) => txt.as_bytes(),
            MessageBody::Vec(vec) => vec.as_slice(),
        }
    }
    fn len(&self) -> usize {
        match self {
            MessageBody::Str(txt) => txt.len(),
            MessageBody::Vec(vec) => vec.len(),
        }
    }
}
