use super::HttpResponse;
use super::HttpVersion;
use super::MessageBody;
pub struct HttpResponseBuilder {
    code: Option<u16>,
    err_message: Option<String>,
    version: Option<HttpVersion>,
    body: Option<MessageBody>,
}

impl HttpResponseBuilder {
    pub fn new() -> HttpResponseBuilder {
        HttpResponseBuilder {
            code: None,
            err_message: None,
            version: None,
            body: None,
        }
    }
    pub fn set_code(&mut self, code: u16) -> &mut Self {
        self.code = Some(code);
        self
    }
    pub fn set_version(&mut self, ver: HttpVersion) -> &mut Self {
        self.version = Some(ver);
        self
    }
    pub fn set_err_message(&mut self, err: String) -> &mut Self {
        self.err_message = Some(err);
        self
    }
    pub fn set_body(&mut self, body: MessageBody) -> &mut Self {
        self.body = Some(body);
        self
    }
    /// Finalized the construction
    /// TODO: Proper error
    pub fn finalize(self) -> Result<HttpResponse, String> {
        let Some(code) = self.code else {
            return Err("Missing response code".to_string());
        };
        let Some(ver) = self.version else {
            return Err("Missing response HTTP version".to_string());
        };
        let err = self.err_message;
        let body = self.body;
        Ok(HttpResponse::builder(code, err, ver, body))
    }
}
