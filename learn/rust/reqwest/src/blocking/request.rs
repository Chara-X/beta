use reqwest::{blocking, header};
/// [blocking::Request]
pub struct Request {}
impl Request {
    /// [blocking::Request::new]
    pub fn new(method: reqwest::Method, url: reqwest::Url) -> Self {
        todo!()
    }
    /// [blocking::Request::method]
    pub fn method(&self) -> &reqwest::Method {
        todo!()
    }
    /// [blocking::Request::method_mut]
    pub fn method_mut(&mut self) -> &mut reqwest::Method {
        todo!()
    }
    /// [blocking::Request::url]
    pub fn url(&self) -> &reqwest::Url {
        todo!()
    }
    /// [blocking::Request::url_mut]
    pub fn url_mut(&mut self) -> &mut reqwest::Url {
        todo!()
    }
    /// [blocking::Request::version]
    pub fn version(&self) -> reqwest::Version {
        todo!()
    }
    /// [blocking::Request::version_mut]
    pub fn version_mut(&mut self) -> &mut reqwest::Version {
        todo!()
    }
    /// [blocking::Request::headers]
    pub fn headers(&self) -> &header::HeaderMap {
        todo!()
    }
    /// [blocking::Request::headers_mut]
    pub fn headers_mut(&mut self) -> &mut header::HeaderMap {
        todo!()
    }
    /// [blocking::Request::body]
    pub fn body(&self) -> Option<&blocking::Body> {
        todo!()
    }
    /// [blocking::Request::body_mut]
    pub fn body_mut(&mut self) -> Option<&mut blocking::Body> {
        todo!()
    }
}
