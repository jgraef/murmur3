//! Wrappers for [`std::io::Read`] and [`std::io::Write`] that also compute a
//! hash.

use std::io::{
    Error,
    Read,
    Write,
};

use crate::hasher::Hasher;

/// A [`std::io::Read`] wrapper, that also hashes any data that is read.
#[derive(Debug)]
pub struct ReadHasher<R, H> {
    reader: R,
    hasher: H,
}

impl<R, H> ReadHasher<R, H> {
    pub fn new(reader: R, hasher: H) -> Self {
        Self { reader, hasher }
    }

    pub fn into_hasher(self) -> H {
        self.hasher
    }
}

impl<R, H: Hasher> ReadHasher<R, H> {
    pub fn hash(self) -> <H as Hasher>::Output {
        self.hasher.finish()
    }
}

impl<R: Read, H: Hasher> Read for ReadHasher<R, H> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let n = self.reader.read(buf)?;

        self.hasher.update(&buf[..n]);

        Ok(n)
    }
}

/// A [`std::io::Write`] wrapper, that also hashes any data that is written.
#[derive(Debug)]
pub struct WriteHasher<W, H> {
    writer: W,
    hasher: H,
}

impl<W, H> WriteHasher<W, H> {
    pub fn new(writer: W, hasher: H) -> Self {
        Self { writer, hasher }
    }

    pub fn into_hasher(self) -> H {
        self.hasher
    }
}

impl<W, H: Hasher> WriteHasher<W, H> {
    pub fn hash(self) -> <H as Hasher>::Output {
        self.hasher.finish()
    }
}

impl<W: Write, H: Hasher> Write for WriteHasher<W, H> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let n = self.writer.write(buf)?;

        self.hasher.update(&buf[..n]);

        Ok(n)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.writer.flush()
    }
}
