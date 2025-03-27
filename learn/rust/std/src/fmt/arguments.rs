use std::marker;
/// [std::fmt::Arguments]
pub struct Arguments<'a> {
    _data: marker::PhantomData<&'a ()>,
}
