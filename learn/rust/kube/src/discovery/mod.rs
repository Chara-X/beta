//! [kube::discovery]
mod api_capabilities;
mod api_group;
mod api_resource;
mod discovery;
// pub mod verbs;
pub use self::api_capabilities::*;
pub use self::api_group::*;
pub use self::api_resource::*;
pub use self::discovery::*;
