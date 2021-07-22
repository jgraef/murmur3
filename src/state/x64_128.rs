use std::convert::{
    TryFrom,
    TryInto,
};

use crate::{
    seed::Seedable,
    state::State,
};

fn fmix64(mut k: u64) -> u64 {
    k ^= k >> 33;
    k = k.wrapping_mul(0xff51afd7ed558ccd);
    k ^= k >> 33;
    k = k.wrapping_mul(0xc4ceb9fe1a85ec53);
    k ^= k >> 33;
    k
}

/// Implementation of [MurmurHash3_x64_128](https://github.com/aappleby/smhasher/blob/master/src/MurmurHash3.cpp#L316-L331)
pub struct State64x128 {
    h1: u64,
    h2: u64,
    len: usize,
}

impl State64x128 {
    const C1: u64 = 0x87c37b91114253d5;
    const C2: u64 = 0x4cf5ad432745937f;

    fn do_k1(&mut self, mut k1: u64) {
        // k1 *= c1; k1  = ROTL64(k1,31); k1 *= c2; h1 ^= k1;
        k1 = k1.wrapping_mul(Self::C1);
        k1 = k1.rotate_left(31);
        k1 = k1.wrapping_mul(Self::C2);
        self.h1 ^= k1;
    }

    fn do_k2(&mut self, mut k2: u64) {
        // k2 *= c2; k2  = ROTL64(k2,33); k2 *= c1; h2 ^= k2;
        k2 = k2.wrapping_mul(Self::C2);
        k2 = k2.rotate_left(33);
        k2 = k2.wrapping_mul(Self::C1);
        self.h2 ^= k2;
    }
}

impl Seedable<u32> for State64x128 {
    fn from_seed(seed: u32) -> Self {
        Self {
            h1: seed.into(),
            h2: seed.into(),
            len: 0,
        }
    }
}

impl State<16> for State64x128 {
    type Output = u128;

    fn push_block(&mut self, data: &[u8; 16]) {
        // body
        // https://github.com/aappleby/smhasher/blob/61a0530f28277f2e850bfc39600ce61d02b518de/src/MurmurHash3.cpp#L267-L284

        // Unfortunately there is no way right now to split `data` into 2 `&[u8; 8]`
        // right now.
        let k1 = u64::from_le_bytes((&data[..8]).try_into().unwrap());
        let k2 = u64::from_le_bytes((&data[8..]).try_into().unwrap());

        self.do_k1(k1);
        self.h1 = self.h1.rotate_left(27);
        self.h1 = self.h1.wrapping_add(self.h2);
        self.h1 = self.h1.wrapping_mul(5).wrapping_add(0x52dce729);

        self.do_k2(k2);
        self.h2 = self.h2.rotate_left(31);
        self.h2 = self.h2.wrapping_add(self.h1);
        self.h2 = self.h2.wrapping_mul(5).wrapping_add(0x38495ab5);

        self.len += 16;
    }

    fn push_tail(&mut self, data: &[u8]) {
        // FIXME: Test `it_hashes_a_tail` fails.

        // tail
        // https://github.com/aappleby/smhasher/blob/61a0530f28277f2e850bfc39600ce61d02b518de/src/MurmurHash3.cpp#L286-L314

        let n = data.len();

        if n >= 16 {
            // Note: It's okay if we accept length = 0, because it'll be a NOP.
            panic!("Tail length must be less than 16");
        }
        else if n > 0 {
            let mut k1 = 0u64;
            let mut k2 = 0u64;

            for i in (9..=n).rev() {
                k2 ^= u64::from(data[i - 1]) << ((i - 9) * 8);
            }
            self.do_k2(k2);

            for i in (1..=n.min(8)).rev() {
                k1 ^= u64::from(data[i - 1]) << ((i - 1) * 8);
            }
            self.do_k1(k1);
        }

        self.len += n;
    }

    fn finish(mut self) -> u128 {
        // finalization
        // https://github.com/aappleby/smhasher/blob/61a0530f28277f2e850bfc39600ce61d02b518de/src/MurmurHash3.cpp#L316-L331

        self.h1 ^= u64::try_from(self.len).expect("Length overflowed");
        self.h2 ^= u64::try_from(self.len).expect("Length overflowed");

        self.h1 = self.h1.wrapping_add(self.h2);
        self.h2 = self.h2.wrapping_add(self.h1);

        self.h1 = fmix64(self.h1);
        self.h2 = fmix64(self.h2);

        self.h1 = self.h1.wrapping_add(self.h2);
        self.h2 = self.h2.wrapping_add(self.h1);

        u128::from(self.h2) << 64 | u128::from(self.h1)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        convert::TryInto,
        io::Cursor,
    };

    use super::State64x128;
    use crate::{
        seed::Seedable,
        state::State,
    };

    #[test]
    fn it_hashes_a_single_block() {
        let data = b"meeeeeeooooowwww";

        let expected_hash = murmur3::murmur3_x64_128(&mut Cursor::new(&data), 69).unwrap();

        let mut h = State64x128::from_seed(69);
        h.push_block(data);
        assert_eq!(h.finish(), expected_hash);
    }

    #[test]
    fn it_hashes_two_blocks() {
        let data = b"nyyyyyyyyyyyyyyyyyyyyyyyyyyyyyan";

        let expected_hash = murmur3::murmur3_x64_128(&mut Cursor::new(&data), 69).unwrap();

        let mut h = State64x128::from_seed(69);
        h.push_block((&data[0..16]).try_into().unwrap());
        h.push_block((&data[16..32]).try_into().unwrap());
        assert_eq!(h.finish(), expected_hash);
    }

    #[test]
    fn it_hashes_empty() {
        let data = b"";

        let expected_hash = murmur3::murmur3_x64_128(&mut Cursor::new(&data), 69).unwrap();

        let h = State64x128::from_seed(69);
        assert_eq!(h.finish(), expected_hash);
    }

    #[test]
    fn it_hashes_a_tail() {
        let data = b"foobar";

        let expected_hash = murmur3::murmur3_x64_128(&mut Cursor::new(&data), 69).unwrap();

        let mut h = State64x128::from_seed(69);
        h.push_tail(data);
        assert_eq!(h.finish(), expected_hash);
    }

    #[test]
    fn it_hashes_body_and_tail() {
        let data = b"This text is a bit longer and not aligned to 16 bytes.";

        let expected_hash = murmur3::murmur3_x64_128(&mut Cursor::new(&data), 69).unwrap();

        let mut h = State64x128::from_seed(69);
        h.push_block((&data[0..16]).try_into().unwrap());
        h.push_block((&data[16..32]).try_into().unwrap());
        h.push_block((&data[32..48]).try_into().unwrap());
        h.push_tail(&data[48..54]);
        assert_eq!(h.finish(), expected_hash);
    }
}
