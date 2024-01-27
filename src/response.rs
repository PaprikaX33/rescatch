mod builder;
use builder::HttpResponseBuilder;

enum MessageBody {
    Str(String),
    Vec(std::vec::Vec<u8>),
}
enum HttpVersion {
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
                match self.code {
                    400 => "Bad Request",
                    _ => "Unknown Code",
                },
            )
            .as_bytes(),
        );
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
    ) -> HttpResponse {
        return HttpResponse {
            code,
            version,
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
