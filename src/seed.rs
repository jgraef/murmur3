pub trait Seedable<T> {
    fn from_seed(seed: T) -> Self;
}
