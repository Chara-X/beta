use super::*;
/// [proc_macro2::TokenTree]
pub enum TokenTree {
    /// [proc_macro2::TokenTree::Group]
    Group(Group),
    /// [proc_macro2::TokenTree::Ident]
    Ident(Ident),
    /// [proc_macro2::TokenTree::Punct]
    Punct(Punct),
    /// [proc_macro2::TokenTree::Literal]
    Literal(Literal),
}
impl TokenTree {
    /// [proc_macro2::TokenTree::span]
    pub fn span(&self) -> proc_macro2::Span {
        todo!()
    }
    /// [proc_macro2::TokenTree::set_span]
    pub fn set_span(&mut self, span: proc_macro2::Span) {
        todo!()
    }
}
