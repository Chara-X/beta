//! [cmp]
use std::cmp;
/// [cmp::PartialEq]
pub trait PartialEq {
    /// [cmp::PartialEq::eq]
    fn eq(&self, other: &Self) -> bool;
}
/// [cmp::Ord]
pub trait Ord: cmp::Ord {
    /// [cmp::Ord::cmp]
    fn cmp(&self, other: &Self) -> cmp::Ordering;
}
