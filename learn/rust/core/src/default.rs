use core::default;
pub trait Default: Sized
where
    Self: default::Default,
{
    fn default() -> Self {
        default::Default::default()
    }
}
