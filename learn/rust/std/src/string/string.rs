use std::{ops, string};
/// [string::String]
pub struct String {}
impl String {
    /// [string::String::new]
    pub const fn new() -> String {
        todo!()
    }
    /// [string::String::from_utf8]
    pub fn from_utf8(vec: Vec<u8>) -> Result<String, string::FromUtf8Error> {
        todo!()
    }
    /// [string::String::len]
    pub fn len(&self) -> usize {
        todo!()
    }
    /// [string::String::capacity]
    pub fn capacity(&self) -> usize {
        todo!()
    }
    /// [string::String::reserve]
    pub fn reserve(&mut self, additional: usize) {
        todo!()
    }
    /// [string::String::reserve_exact]
    pub fn reserve_exact(&mut self, additional: usize) {
        todo!()
    }
    /// [string::String::shrink_to]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        todo!()
    }
    /// [string::String::shrink_to_fit]
    pub fn shrink_to_fit(&mut self) {
        todo!()
    }
    /// [string::String::push]
    pub fn push(&mut self, ch: char) {
        todo!()
    }
    /// [string::String::push_str]
    pub fn push_str(&mut self, string: &str) {
        todo!()
    }
    /// [string::String::pop]
    pub fn pop(&mut self) {
        todo!()
    }
    /// [string::String::insert]
    pub fn insert(&mut self, idx: usize, ch: char) {
        todo!()
    }
    /// [string::String::insert_str]
    pub fn insert_str(&mut self, idx: usize, string: &str) {
        todo!()
    }
    /// [string::String::remove]
    pub fn remove(&mut self, idx: usize) -> char {
        todo!()
    }
    /// [string::String::clear]
    pub fn clear(&mut self) {
        todo!()
    }
    /// [string::String::retain]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(char) -> bool,
    {
        todo!()
    }
    /// [string::String::replace_range]
    pub fn replace_range<R>(&mut self, range: R, replace_with: &str)
    where
        R: ops::RangeBounds<usize>,
    {
        todo!()
    }
    /// [string::String::as_bytes]
    pub fn as_bytes(&self) -> &[u8] {
        todo!()
    }
    /// [string::String::as_str]
    pub fn as_str(&self) -> &str {
        todo!()
    }
    /// [string::String::as_mut_str]
    pub fn as_mut_str(&mut self) -> &mut str {
        todo!()
    }
    /// [string::String::into_bytes]
    pub fn into_bytes(self) -> Vec<u8> {
        todo!()
    }
    /// [string::String::into_boxed_str]
    pub fn into_boxed_str(self) -> Box<str> {
        todo!()
    }
}
