use kube::{api, core::request};
/// [request::Request]
pub struct Request {
    /// [request::Request::url_path]
    pub url_path: String,
}
impl Request {
    /// [request::Request::create]
    pub fn create(
        &self,
        pp: &api::PostParams,
        data: Vec<u8>,
    ) -> Result<http::Request<Vec<u8>>, request::Error> {
        todo!()
    }
    /// [request::Request::replace]
    pub fn replace(
        &self,
        name: &str,
        pp: &api::PostParams,
        data: Vec<u8>,
    ) -> Result<http::Request<Vec<u8>>, request::Error> {
        todo!()
    }
    /// [request::Request::patch]
    pub fn patch<P>(
        &self,
        name: &str,
        pp: &api::PatchParams,
        patch: &api::Patch<P>,
    ) -> Result<http::Request<Vec<u8>>, request::Error>
    where
        P: serde::Serialize,
    {
        todo!()
    }
    /// [request::Request::delete]
    pub fn delete(
        &self,
        name: &str,
        dp: &api::DeleteParams,
    ) -> Result<http::Request<Vec<u8>>, request::Error> {
        todo!()
    }
    /// [request::Request::list]
    pub fn list(&self, lp: &api::ListParams) -> Result<http::Request<Vec<u8>>, request::Error> {
        todo!()
    }
    /// [request::Request::get]
    pub fn get(
        &self,
        name: &str,
        gp: &api::GetParams,
    ) -> Result<http::Request<Vec<u8>>, request::Error> {
        todo!()
    }
    /// [request::Request::watch]
    pub fn watch(
        &self,
        wp: &api::WatchParams,
        ver: &str,
    ) -> Result<http::Request<Vec<u8>>, request::Error> {
        todo!()
    }
    /// [request::Request::replace_subresource]
    pub fn replace_subresource(
        &self,
        subresource_name: &str,
        name: &str,
        pp: &api::PostParams,
        data: Vec<u8>,
    ) -> Result<http::Request<Vec<u8>>, request::Error> {
        todo!()
    }
    /// [request::Request::patch_subresource]
    pub fn patch_subresource<P>(
        &self,
        subresource_name: &str,
        name: &str,
        pp: &api::PatchParams,
        patch: &api::Patch<P>,
    ) -> Result<http::Request<Vec<u8>>, request::Error>
    where
        P: serde::Serialize,
    {
        todo!()
    }
    /// [request::Request::logs]
    pub fn logs(
        &self,
        name: &str,
        lp: &api::LogParams,
    ) -> Result<http::Request<Vec<u8>>, request::Error> {
        todo!()
    }
    /// [request::Request::attach]
    pub fn attach(
        &self,
        name: &str,
        ap: &api::AttachParams,
    ) -> Result<http::Request<Vec<u8>>, request::Error> {
        todo!()
    }
    /// [request::Request::exec]
    pub fn exec<I, T>(
        &self,
        name: &str,
        command: I,
        ap: &api::AttachParams,
    ) -> Result<http::Request<Vec<u8>>, request::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        todo!()
    }
    /// [request::Request::portforward]
    pub fn portforward(
        &self,
        name: &str,
        ports: &[u16],
    ) -> Result<http::Request<Vec<u8>>, request::Error> {
        todo!()
    }
}
