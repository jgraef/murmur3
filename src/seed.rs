//! Trait for hashers that are created wit ha seed.

// Implementors of this create can be created using a seed of type `T`.
pub trait Seedable<T> {
    fn from_seed(seed: T) -> Self;
}
