use crate::parse;
use std::fmt;
/// [syn::meta::ParseNestedMeta]
pub struct ParseNestedMeta<'a> {
    /// [syn::meta::ParseNestedMeta::path]
    pub path: syn::Path,
    /// [syn::meta::ParseNestedMeta::input]
    pub input: &'a parse::ParseBuffer<'a>,
}
impl<'a> ParseNestedMeta<'a> {
    /// [syn::meta::ParseNestedMeta::value]
    pub fn value(&self) -> syn::Result<&parse::ParseBuffer<'a>> {
        todo!()
    }
    /// [syn::meta::ParseNestedMeta::parse_nested_meta]
    pub fn parse_nested_meta(
        &self,
        logic: impl FnMut(ParseNestedMeta<'_>) -> syn::Result<()>,
    ) -> syn::Result<()> {
        todo!()
    }
    /// [syn::meta::ParseNestedMeta::error]
    pub fn error(&self, msg: impl fmt::Display) -> crate::Error {
        todo!()
    }
}
