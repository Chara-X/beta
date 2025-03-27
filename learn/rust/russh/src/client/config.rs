/// [russh::client::Config]
pub struct Config {
    /// [russh::client::Config::client_id]
    pub client_id: russh::SshId,
    /// [russh::client::Config::preferred]
    pub preferred: russh::Preferred,
    /// [russh::client::Config::anonymous]
    pub anonymous: bool,
}
