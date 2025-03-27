/// [syn::spanned::Spanned]
pub trait Spanned {
    /// [syn::spanned::Spanned::span]
    fn span(&self) -> proc_macro2::Span;
}
