use reqwest::header;
use std::net;
/// [reqwest::blocking::Response]
pub struct Response {}
impl Response {
    /// [reqwest::blocking::Response::version]
    pub fn version(&self) -> reqwest::Version {
        todo!()
    }
    /// [reqwest::blocking::Response::status]
    pub fn status(&self) -> reqwest::StatusCode {
        todo!()
    }
    /// [reqwest::blocking::Response::headers]
    pub fn headers(&self) -> &header::HeaderMap {
        todo!()
    }
    /// [reqwest::blocking::Response::headers_mut]
    pub fn headers_mut(&mut self) -> &mut header::HeaderMap {
        todo!()
    }
    /// [reqwest::blocking::Response::content_length]
    pub fn content_length(&self) -> Option<u64> {
        todo!()
    }
    /// [reqwest::blocking::Response::bytes]
    pub fn bytes(self) -> reqwest::Result<bytes::Bytes> {
        todo!()
    }
    /// [reqwest::blocking::Response::text]
    pub fn text(self) -> reqwest::Result<String> {
        todo!()
    }
    /// [reqwest::blocking::Response::json]
    pub fn json<T: for<'de> serde::Deserialize<'de>>(self) -> reqwest::Result<T> {
        todo!()
    }
    /// [reqwest::blocking::Response::url]
    pub fn url(&self) -> &reqwest::Url {
        todo!()
    }
    /// [reqwest::blocking::Response::remote_addr]
    pub fn remote_addr(&self) -> Option<net::SocketAddr> {
        todo!()
    }
    /// [reqwest::blocking::Response::error_for_status]
    pub fn error_for_status(self) -> reqwest::Result<Self> {
        todo!()
    }
    /// [reqwest::blocking::Response::error_for_status_ref]
    pub fn error_for_status_ref(&self) -> reqwest::Result<&Self> {
        todo!()
    }
}
