use proc_macro2::extra;
use std::marker;
/// [syn::buffer::Cursor]
pub struct Cursor<'a> {
    _data: marker::PhantomData<&'a ()>,
}
impl<'a> Cursor<'a> {
    /// [syn::buffer::Cursor::token_tree]
    pub fn token_tree(self) -> Option<(proc_macro2::TokenTree, Cursor<'a>)> {
        todo!()
    }
    /// [syn::buffer::Cursor::group]
    pub fn group(
        self,
        delim: proc_macro2::Delimiter,
    ) -> Option<(Cursor<'a>, extra::DelimSpan, Cursor<'a>)> {
        todo!()
    }
    /// [syn::buffer::Cursor::ident]
    pub fn ident(self) -> Option<(proc_macro2::Ident, Cursor<'a>)> {
        todo!()
    }
    /// [syn::buffer::Cursor::punct]
    pub fn punct(self) -> Option<(proc_macro2::Punct, Cursor<'a>)> {
        todo!()
    }
    /// [syn::buffer::Cursor::literal]
    pub fn literal(self) -> Option<(proc_macro2::Literal, Cursor<'a>)> {
        todo!()
    }
    /// [syn::buffer::Cursor::eof]
    pub fn eof(self) -> bool {
        todo!()
    }
    /// [syn::buffer::Cursor::token_stream]
    pub fn token_stream(self) -> proc_macro2::TokenStream {
        todo!()
    }
}
