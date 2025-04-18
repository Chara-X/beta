#![deny(missing_docs)]
#![allow(unused_variables)]
#![allow(clippy::new_without_default)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
#![allow(private_bounds)]
#![forbid(unsafe_code)]
//! [kube]
pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod discovery;
