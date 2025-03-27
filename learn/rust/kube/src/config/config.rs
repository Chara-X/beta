use kube::config;
/// [config::Config]
pub struct Config {
    /// [config::Config::cluster_url]
    pub cluster_url: http::Uri,
    /// [config::Config::default_namespace]
    pub default_namespace: String,
    /// [config::Config::root_cert]
    pub root_cert: Option<Vec<Vec<u8>>>,
    /// [config::Config::accept_invalid_certs]
    pub accept_invalid_certs: bool,
    /// [config::Config::auth_info]
    pub auth_info: config::AuthInfo,
    /// [config::Config::proxy_url]
    pub proxy_url: Option<http::Uri>,
    /// [config::Config::headers]
    pub headers: Vec<(http::HeaderName, http::HeaderValue)>,
}
impl Config {
    /// [config::Config::infer]
    pub async fn infer() -> Result<Config, config::InferConfigError> {
        todo!()
    }
}
