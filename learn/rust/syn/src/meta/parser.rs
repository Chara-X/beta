use super::*;
use syn::parse;
/// [syn::meta::parser]
pub fn parser(
    logic: impl FnMut(ParseNestedMeta<'_>) -> syn::Result<()>,
) -> impl parse::Parser<Output = ()> {
    struct Todo;
    impl parse::Parser for Todo {
        type Output = ();
        fn parse2(self, tokens: proc_macro2::TokenStream) -> syn::Result<Self::Output> {
            todo!()
        }
    }
    Todo
}
