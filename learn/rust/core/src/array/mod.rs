pub struct Array<T, const N: usize>([T; N]);
impl<T, const N: usize> Array<T, N> {
    pub fn each_ref(&self) -> [&T; N] {
        self.0.each_ref()
    }
    pub fn each_mut(&mut self) -> [&mut T; N] {
        self.0.each_mut()
    }
    pub fn map<U, F>(self, f: F) -> Array<U, N>
    where
        F: FnMut(T) -> U,
    {
        Array(self.0.map(f))
    }
}
