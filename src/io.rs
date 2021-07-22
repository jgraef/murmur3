use std::io::{
    Error,
    Read,
    Write,
};

use crate::hasher::Hasher;

#[derive(Debug)]
pub struct ReadHasher<R, H> {
    reader: R,
    hasher: H,
}

impl<R, H> ReadHasher<R, H> {
    pub fn new(reader: R, hasher: H) -> Self {
        Self { reader, hasher }
    }

    pub fn into_parts(self) -> (R, H) {
        (self.reader, self.hasher)
    }
}

impl<R, H: Hasher> ReadHasher<R, H> {
    pub fn finish(self) -> (R, <H as Hasher>::Output) {
        let hash = self.hasher.finish();
        (self.reader, hash)
    }
}

impl<R: Read, H: Hasher> Read for ReadHasher<R, H> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let n = self.reader.read(buf)?;

        self.hasher.update(&buf[..n]);

        Ok(n)
    }
}

#[derive(Debug)]
pub struct WriteHasher<W, H> {
    writer: W,
    hasher: H,
}

impl<W, H> WriteHasher<W, H> {
    pub fn new(writer: W, hasher: H) -> Self {
        Self { writer, hasher }
    }

    pub fn into_parts(self) -> (W, H) {
        (self.writer, self.hasher)
    }
}

impl<W, H: Hasher> WriteHasher<W, H> {
    pub fn finish(self) -> (W, <H as Hasher>::Output) {
        let hash = self.hasher.finish();
        (self.writer, hash)
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
