/// [quote::ToTokens]
pub trait ToTokens {
    /// [quote::ToTokens::to_tokens]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream);
}
