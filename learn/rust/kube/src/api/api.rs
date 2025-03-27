use super::*;
use crate::client;
use crate::core;
use futures::{io, stream};
use kube::api;
use std::{fmt, iter, marker};
/// [kube::Api]
pub struct Api<K> {
    _marker: marker::PhantomData<K>,
}
impl<K> Api<K>
where
    K: core::Resource,
    <K as core::Resource>::DynamicType: Default,
{
    /// [kube::Api::all]
    pub fn all(client: client::Client) -> Api<K> {
        todo!()
    }
    /// [kube::Api::namespaced]
    pub fn namespaced(client: client::Client, ns: &str) -> Api<K> {
        todo!()
    }
}
impl<K> Api<K>
where
    K: core::Resource,
{
    /// [kube::Api::all_with]
    pub fn all_with(
        client: client::Client,
        dyntype: &<K as core::Resource>::DynamicType,
    ) -> Api<K> {
        todo!()
    }
    /// [kube::Api::namespaced_with]
    pub fn namespaced_with(
        client: client::Client,
        ns: &str,
        dyntype: &<K as core::Resource>::DynamicType,
    ) -> Api<K>
    where
        K: core::Resource<Scope = core::DynamicResourceScope>,
    {
        todo!()
    }
    /// [kube::Api::resource_url]
    pub fn resource_url(&self) -> &str {
        todo!()
    }
    /// [kube::Api::into_client]
    pub fn into_client(self) -> client::Client {
        todo!()
    }
}
impl<K> Api<K>
where
    K: Clone + for<'de> serde::Deserialize<'de> + fmt::Debug,
{
    /// [kube::Api::create]
    pub async fn create(&self, pp: &api::PostParams, data: &K) -> Result<K, kube::Error>
    where
        K: serde::Serialize,
    {
        todo!()
    }
    /// [kube::Api::replace]
    pub async fn replace(
        &self,
        name: &str,
        pp: &api::PostParams,
        data: &K,
    ) -> Result<K, kube::Error>
    where
        K: serde::Serialize,
    {
        todo!()
    }
    /// [kube::Api::patch]
    pub async fn patch<P>(
        &self,
        name: &str,
        pp: &api::PatchParams,
        patch: &api::Patch<P>,
    ) -> Result<K, kube::Error>
    where
        P: serde::Serialize + fmt::Debug,
    {
        todo!()
    }
    /// [kube::Api::delete]
    pub async fn delete(
        &self,
        name: &str,
        dp: &api::DeleteParams,
    ) -> Result<either::Either<K, core::Status>, kube::Error> {
        todo!()
    }
    /// [kube::Api::list]
    pub async fn list(&self, lp: &api::ListParams) -> Result<core::ObjectList<K>, kube::Error> {
        todo!()
    }
    /// [kube::Api::get]
    pub async fn get(&self, name: &str) -> Result<K, kube::Error> {
        todo!()
    }
    /// [kube::Api::watch]
    pub async fn watch(
        &self,
        wp: &api::WatchParams,
        version: &str,
    ) -> Result<impl futures::Stream<Item = Result<api::WatchEvent<K>, kube::Error>>, kube::Error>
    {
        Ok(stream::empty())
    }
}
impl<K> Api<K>
where
    K: for<'de> serde::Deserialize<'de>,
{
    /// [kube::Api::replace_status]
    pub async fn replace_status(
        &self,
        name: &str,
        pp: &api::PostParams,
        data: Vec<u8>,
    ) -> Result<K, kube::Error> {
        todo!()
    }
    /// [kube::Api::patch_status]
    pub async fn patch_status<P>(
        &self,
        name: &str,
        pp: &api::PatchParams,
        patch: &api::Patch<P>,
    ) -> Result<K, kube::Error>
    where
        P: serde::Serialize + fmt::Debug,
    {
        todo!()
    }
}
impl<K> Api<K>
where
    K: for<'de> serde::Deserialize<'de> + api::Log,
{
    /// [kube::Api::logs]
    pub async fn logs(&self, name: &str, lp: &api::LogParams) -> Result<String, kube::Error> {
        todo!()
    }
    /// [kube::Api::log_stream]
    pub async fn log_stream(
        &self,
        name: &str,
        lp: &api::LogParams,
    ) -> Result<impl io::AsyncBufRead, kube::Error> {
        Ok(io::empty())
    }
}
impl<K> Api<K>
where
    K: Clone + for<'de> serde::Deserialize<'de> + api::Attach,
{
    /// [kube::Api::attach]
    pub async fn attach(
        &self,
        name: &str,
        ap: &api::AttachParams,
    ) -> Result<AttachedProcess, kube::Error> {
        todo!()
    }
}
impl<K> Api<K>
where
    K: Clone + for<'de> serde::Deserialize<'de> + api::Execute,
{
    /// [kube::Api::exec]
    pub async fn exec<I, T>(
        &self,
        name: &str,
        command: I,
        ap: &api::AttachParams,
    ) -> Result<AttachedProcess, kube::Error>
    where
        I: iter::IntoIterator<Item = T> + fmt::Debug,
        T: Into<String>,
    {
        todo!()
    }
}
impl<K> Api<K>
where
    K: Clone + for<'de> serde::Deserialize<'de> + api::Portforward,
{
    /// [kube::Api::portforward]
    pub async fn portforward(
        &self,
        name: &str,
        ports: &[u16],
    ) -> Result<Portforwarder, kube::Error> {
        todo!()
    }
}
