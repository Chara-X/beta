use crate::core;
use futures::{io, stream};
use kube::{api, client};
/// [kube::Client]
pub struct Client {}
impl Client {
    /// [kube::Client::send]
    pub async fn send(
        &self,
        request: http::Request<client::Body>,
    ) -> Result<http::Response<client::Body>, kube::Error> {
        todo!()
    }
    /// [kube::Client::request_text]
    pub async fn request_text(
        &self,
        request: http::Request<Vec<u8>>,
    ) -> Result<String, kube::Error> {
        todo!()
    }
    /// [kube::Client::request_status]
    pub async fn request_status<T>(
        &self,
        request: http::Request<Vec<u8>>,
    ) -> Result<either::Either<T, core::Status>, kube::Error>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        todo!()
    }
    /// [kube::Client::request_stream]
    pub async fn request_stream(
        &self,
        request: http::Request<Vec<u8>>,
    ) -> Result<impl futures::AsyncBufRead, kube::Error> {
        Ok(io::empty())
    }
    /// [kube::Client::request_events]
    pub async fn request_events<T>(
        &self,
        request: http::Request<Vec<u8>>,
    ) -> Result<impl futures::TryStream<Ok = api::WatchEvent<T>, Error = kube::Error>, kube::Error>
    where
        T: Clone + for<'de> serde::Deserialize<'de>,
    {
        Ok(stream::empty::<Result<api::WatchEvent<T>, kube::Error>>())
    }
}
