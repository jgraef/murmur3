//! Low-level implementation of the Murmur3 hashing state.

mod x64_128;

pub use x64_128::State64x128;

/// Trait for Murmur3 hash state. It only accepts blocks of data with fixed size
/// `N` via [`State::push_block`]. Only the last block may be of length `0 < n <
/// N`.
///
/// # TODO:
///
/// - This currently takes `N` as const generic parameter, but we want it to be
///   a associated constant. This is only supported using the unstable feature
///   `const_evaluatable_checked`.
pub trait State<const N: usize> {
    /// The type of hash this outputs.
    type Output;

    /// Processes a block of data into the hash state.
    fn push_block(&mut self, data: &[u8; N]);

    /// Processes the last block of data into the hash state. This may panic if
    /// the block is not of length `0 < n < N`.
    fn push_tail(&mut self, data: &[u8]);

    /// Finalizes the hash and returns it.
    fn finish(self) -> Self::Output;
}
