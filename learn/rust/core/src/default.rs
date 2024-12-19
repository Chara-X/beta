use core::default;
pub trait Default: default::Default {
    fn default() -> Self {
        default::Default::default()
    }
}
