use std::{env, ffi};
/// [env::var]
pub fn var<K: AsRef<ffi::OsStr>>(key: K) -> Result<String, env::VarError> {
    todo!()
}
