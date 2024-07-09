pub struct OwnsBuffer<const N: usize, T> {
    buffer: [T; N],
}

impl<const N: usize, T: Default> Default for OwnsBuffer<N, T> {
    fn default() -> Self {
        Self { buffer: core::array::from_fn(|_| T::default()) }
    }
}

impl<const N: usize, T> OwnsBuffer<N, T> {
    pub fn new(buffer: [T; N]) -> Self {
        Self { buffer }
    }

    pub fn get(&self) -> &[T; N] {
        &self.buffer
    }
    pub fn get_mut(&mut self) -> &mut [T; N] {
        &mut self.buffer
    }
}
