use std::any;
/// [std::panic::panic_any]
pub fn panic_any<M: 'static + any::Any + Send>(msg: M) -> ! {
    todo!()
}
