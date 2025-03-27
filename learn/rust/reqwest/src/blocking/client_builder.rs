use super::*;
use reqwest::{self, dns, header, tls};
use reqwest::{cookie, redirect};
use std::sync;
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
    /// [reqwest::blocking::ClientBuilder::cookie_provider]
    pub fn cookie_provider<C: cookie::CookieStore + 'static>(
        self,
        cookie_store: sync::Arc<C>,
    ) -> ClientBuilder {
        todo!()
    }
    /// [reqwest::blocking::ClientBuilder::dns_resolver]
    pub fn dns_resolver<R: dns::Resolve + 'static>(self, resolver: sync::Arc<R>) -> ClientBuilder {
        todo!()
    }
    /// [reqwest::blocking::ClientBuilder::add_root_certificate]
    pub fn add_root_certificate(self, cert: tls::Certificate) -> ClientBuilder {
        todo!()
    }
    /// [reqwest::blocking::ClientBuilder::identity]
    pub fn identity(self, identity: tls::Identity) -> ClientBuilder {
        todo!()
    }
    /// [reqwest::blocking::ClientBuilder::danger_accept_invalid_hostnames]
    pub fn danger_accept_invalid_hostnames(self, accept_invalid_hostname: bool) -> ClientBuilder {
        todo!()
    }
    /// [reqwest::blocking::ClientBuilder::danger_accept_invalid_certs]
    pub fn danger_accept_invalid_certs(self, accept_invalid_certs: bool) -> ClientBuilder {
        todo!()
    }
    /// [reqwest::blocking::ClientBuilder::redirect]
    pub fn redirect(self, policy: redirect::Policy) -> ClientBuilder {
        todo!()
    }
    /// [reqwest::blocking::ClientBuilder::gzip]
    pub fn gzip(self, enable: bool) -> ClientBuilder {
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
