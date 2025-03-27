//! [kube::core]
mod custom_resource_ext;
mod dynamic_object;
mod group_version_kind;
mod object;
mod object_list;
mod resource;
mod type_meta;
mod version;
pub use self::custom_resource_ext::*;
pub use self::dynamic_object::*;
pub use self::group_version_kind::*;
pub use self::object::*;
pub use self::object_list::*;
pub use self::resource::*;
pub use self::type_meta::*;
pub use self::version::*;
pub use kube::core::NotUsed;
pub use kube::core::Status;
