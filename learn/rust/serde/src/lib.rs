#![deny(missing_docs)]
#![allow(unused_variables)]
//! [serde]
mod deserialize;
mod deserializer;
mod serialize;
mod serializer;
pub use self::deserialize::*;
pub use self::deserializer::*;
pub use self::serialize::*;
pub use self::serializer::*;
