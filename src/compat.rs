use std::io::{Read, Result};

    use crate::{
        hasher::{Murmur3x64x128, Hasher},
        seed::Seedable,
    };

    pub fn murmur3_x64_128<T: Read>(source: &mut T, seed: u32) -> Result<u128> {
        let mut buf = [0; 1024];
        
        let mut hasher = Murmur3x64x128::from_seed(seed);

        loop {
            let n = source.read(&mut buf)?;
            if n == 0 {
                break Ok(hasher.finish());
            }
            hasher.update(&buf[0 .. n]);
        }
    }


#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn it_returns_same_hash_as_original() {
        let data = b"meeeeeeooooowwww";

        let expected_hash = murmur3::murmur3_x64_128(&mut Cursor::new(data), 69).unwrap();
        let got_hash = super::murmur3_x64_128(&mut Cursor::new(data), 69).unwrap();

        assert_eq!(expected_hash, got_hash);
    }
}
