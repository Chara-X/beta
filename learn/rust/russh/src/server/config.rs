use crate::keys;
/// [russh::server::Config]
pub struct Config {
    /// [russh::server::Config::server_id]
    pub server_id: russh::SshId,
    /// [russh::server::Config::methods]
    pub methods: russh::MethodSet,
    /// [russh::server::Config::keys]
    pub keys: Vec<keys::PrivateKey>,
    /// [russh::server::Config::preferred]
    pub preferred: russh::Preferred,
    /// [russh::server::Config::nodelay]
    pub nodelay: bool,
}
