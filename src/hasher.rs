use crate::{
    chunked::Chunked,
    seed::Seedable,
    state::{
        State,
        State64x128,
    },
};

pub trait Hasher {
    type Output;

    /// Pushes the `data` into the hasher, updating its state.
    fn update(&mut self, data: &[u8]);

    /// Finalizes the hash and returns it.
    fn finish(self) -> Self::Output;
}

/// Short-hand for constructing the hasher with `seed`, calling [`Self::update`]
/// with the data and then [`Self::finish`].
pub fn hash<H: Hasher + Seedable<T>, T>(seed: T, data: &[u8]) -> H::Output {
    let mut hasher = H::from_seed(seed);
    hasher.update(data);
    hasher.finish()
}

/// Higher-level implementation of a hasher. Generic over the hashing state.
///
/// # TODO
///
/// - See the TODO section at [`crate::state::State`], for information about why
///   this struct needs `N` as parameter.
pub struct Murmur3Hasher<S, const N: usize> {
    state: S,
    chunked: Chunked<u8, N>,
}

impl<S: State<N>, const N: usize> Hasher for Murmur3Hasher<S, N> {
    type Output = S::Output;

    fn update(&mut self, data: &[u8]) {
        for block in self.chunked.next(data) {
            self.state.push_block(block);
        }
    }

    fn finish(mut self) -> S::Output {
        self.state.push_tail(self.chunked.remainder());
        self.state.finish()
    }
}

impl<S: Seedable<T>, T, const N: usize> Seedable<T> for Murmur3Hasher<S, N> {
    fn from_seed(seed: T) -> Self {
        Self {
            state: S::from_seed(seed),
            chunked: Chunked::default(),
        }
    }
}

pub type Murmur3x64x128 = Murmur3Hasher<State64x128, 16>;

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::{
        hash,
        Hasher,
        Murmur3x64x128,
    };
    use crate::seed::Seedable;

    fn test_with_data(data: &[u8]) {
        let expected = murmur3::murmur3_x64_128(&mut Cursor::new(data), 0).unwrap();
        let got = hash::<Murmur3x64x128, _>(0, data);
        assert_eq!(got, expected);
    }

    #[test]
    fn it_hashes_short_data() {
        test_with_data(b"Hello World");
    }

    #[test]
    fn it_hashes_long_data() {
        let mut data = vec![];
        while data.len() < 1024 {
            data.extend(b"Hello World!");
        }

        test_with_data(&data)
    }

    #[test]
    fn it_hashes_in_multiple_steps() {
        let mut n = 0;
        let mut hasher = Murmur3x64x128::from_seed(0);
        let data = b"Got anymore test strings?";
        let mut expected_data = vec![];

        while n < 1024 {
            hasher.update(data);
            n += data.len();
            expected_data.extend(data);
        }

        let expected = murmur3::murmur3_x64_128(&mut Cursor::new(expected_data), 0).unwrap();
        assert_eq!(hasher.finish(), expected);
    }
}
