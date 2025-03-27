use crate::discovery;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiextension;
/// [kube::core::CustomResourceExt]
pub trait CustomResourceExt {
    /// [kube::core::CustomResourceExt::api_resource]
    fn api_resource() -> discovery::ApiResource;
    /// [kube::core::CustomResourceExt::crd_name]
    fn crd_name() -> &'static str;
    /// [kube::core::CustomResourceExt::crd]
    fn crd() -> apiextension::CustomResourceDefinition;
}
