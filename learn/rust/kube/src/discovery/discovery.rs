use super::*;
use crate::client;
use std::iter;
/// [kube::discovery::Discovery]
pub struct Discovery {}
impl Discovery {
    /// [kube::discovery::Discovery::new]
    pub fn new(client: client::Client) -> Discovery {
        todo!()
    }
    /// [kube::discovery::Discovery::run]
    pub async fn run(self) -> Result<Discovery, kube::Error> {
        todo!()
    }
    /// [kube::discovery::Discovery::groups]
    pub fn groups(&self) -> impl Iterator<Item = &ApiGroup> {
        iter::empty()
    }
    /// [kube::discovery::Discovery::get]
    pub fn get(&self, group: &str) -> Option<&ApiGroup> {
        todo!()
    }
}
