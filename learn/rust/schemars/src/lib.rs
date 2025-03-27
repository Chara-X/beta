#![deny(missing_docs)]
#![allow(unused_variables)]
//! [schemars]
mod json_schema;
mod schema_generator;
pub use self::json_schema::*;
pub use self::schema_generator::*;
