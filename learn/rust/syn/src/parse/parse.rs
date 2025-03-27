use super::*;
/// [syn::parse::Parser::parse]
pub fn parse<T: Parse>(tokens: proc_macro2::TokenStream) -> syn::Result<T> {
    todo!();
}
/// [syn::parse::Parse]
pub trait Parse: Sized {
    /// [syn::parse::Parse::parse]
    fn parse(input: &ParseBuffer<'_>) -> syn::Result<Self>;
}
