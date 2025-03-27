use super::*;
use reqwest;
/// [reqwest::blocking::Client]
pub struct Client {}
impl Client {
    /// [reqwest::blocking::Client::execute]
    pub fn execute(&self, request: Request) -> reqwest::Result<Response> {
        todo!()
    }
}
