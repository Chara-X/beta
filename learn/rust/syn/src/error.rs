use std::fmt;
/// [syn::Error]
pub struct Error {}
impl Error {
    /// [syn::Error::new]
    pub fn new<T: fmt::Display>(span: proc_macro2::Span, message: T) -> Self {
        todo!()
    }
    /// [syn::Error::span]
    pub fn span(&self) -> proc_macro2::Span {
        todo!()
    }
    /// [syn::Error::to_compile_error]
    pub fn to_compile_error(&self) -> proc_macro2::TokenStream {
        todo!()
    }
}
