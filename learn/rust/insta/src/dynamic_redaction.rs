use insta::internals;
/// [insta::dynamic_redaction]
pub fn dynamic_redaction<I, F>(func: F) -> internals::Redaction
where
    I: Into<internals::Content>,
    F: Fn(internals::Content, internals::ContentPath<'_>) -> I + Send + Sync + 'static,
{
    todo!()
}
