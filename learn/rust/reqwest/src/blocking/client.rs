use crate::blocking::request;
use reqwest::{self, header};
use std::net;
/// [reqwest::blocking::ClientBuilder]
pub struct ClientBuilder {}
impl ClientBuilder {
    /// [reqwest::blocking::ClientBuilder::new]
    pub fn new() -> Self {
        todo!()
    }
    /// [reqwest::blocking::ClientBuilder::proxy]
    pub fn proxy(self, proxy: reqwest::Proxy) -> ClientBuilder {
        todo!()
    }
    /// [reqwest::blocking::ClientBuilder::resolve]
    pub fn resolve(self, domain: &str, addr: net::SocketAddr) -> ClientBuilder {
        todo!()
    }
    /// [reqwest::blocking::ClientBuilder::default_headers]
    pub fn default_headers(self, headers: header::HeaderMap) -> ClientBuilder {
        todo!()
    }
    /// [reqwest::blocking::ClientBuilder::build]
    pub fn build(self) -> reqwest::Result<Client> {
        todo!()
    }
}
/// [reqwest::blocking::Client]
pub struct Client {}
impl Client {
    /// [reqwest::blocking::Client::request]
    pub fn request<U: reqwest::IntoUrl>(
        &self,
        method: reqwest::Method,
        url: U,
    ) -> request::RequestBuilder {
        todo!()
    }
}
