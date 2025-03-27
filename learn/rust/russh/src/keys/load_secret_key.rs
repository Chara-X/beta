use super::*;
use russh::keys;
use std::path;
/// [russh::keys::load_secret_key]
pub fn load_secret_key<P: AsRef<path::Path>>(
    secret_: P,
    password: Option<&str>,
) -> Result<PrivateKey, keys::Error> {
    todo!()
}
