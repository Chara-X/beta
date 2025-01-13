use reqwest::{self, blocking, header};
use serde;
/// [blocking::RequestBuilder]
pub struct RequestBuilder {}
impl RequestBuilder {
    /// [blocking::RequestBuilder::query]
    pub fn query<T: serde::Serialize + ?Sized>(self, query: &T) -> RequestBuilder {
        todo!()
    }
    /// [blocking::RequestBuilder::version]
    pub fn version(self, version: reqwest::Version) -> RequestBuilder {
        todo!()
    }
    /// [blocking::RequestBuilder::header]
    pub fn header<K, V>(self, key: K, value: V) -> RequestBuilder
    where
        header::HeaderName: TryFrom<K>,
        header::HeaderValue: TryFrom<V>,
        <header::HeaderName as TryFrom<K>>::Error: Into<reqwest::Error>,
        <header::HeaderValue as TryFrom<V>>::Error: Into<reqwest::Error>,
    {
        todo!()
    }
    /// [blocking::RequestBuilder::body]
    pub fn body<T: Into<blocking::Body>>(self, body: T) -> RequestBuilder {
        todo!()
    }
    /// [blocking::RequestBuilder::send]
    pub fn send(self) -> reqwest::Result<blocking::Response> {
        todo!()
    }
}
