use kube::discovery;
/// [discovery::ApiCapabilities]
pub struct ApiCapabilities {
    /// [discovery::ApiCapabilities::scope]
    pub scope: discovery::Scope,
    /// [discovery::ApiCapabilities::operations]
    pub operations: Vec<String>,
}
