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
pub struct HttpResponse {
    code: u8,
    errMessage: Option<String>,
    version: HttpVersion,
    body: MessageBody,
}

fn builder(code: u8, err: Option<String>, version: HttpVersion, body: MessageBody) -> HttpResponse {
    return HttpResponse {
        code,
        version,
        body,
        errMessage: err,
    };
}
