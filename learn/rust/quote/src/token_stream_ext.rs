use super::*;
/// [quote::TokenStreamExt]
pub trait TokenStreamExt {
    /// [quote::TokenStreamExt::append]
    fn append<U>(&mut self, token: U)
    where
        U: Into<proc_macro2::TokenTree>;
    /// [quote::TokenStreamExt::append_all]
    fn append_all<I>(&mut self, iter: I)
    where
        I: IntoIterator,
        I::Item: ToTokens;
    /// [quote::TokenStreamExt::append_separated]
    fn append_separated<I, U>(&mut self, iter: I, op: U)
    where
        I: IntoIterator,
        I::Item: ToTokens,
        U: ToTokens;
    /// [quote::TokenStreamExt::append_terminated]
    fn append_terminated<I, U>(&mut self, iter: I, term: U)
    where
        I: IntoIterator,
        I::Item: ToTokens,
        U: ToTokens;
}
