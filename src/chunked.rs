use std::slice::ArrayChunks;

use arrayvec::ArrayVec;

/// Takes in arbitrarily-sized slices and returns iterators over fixed size
/// chunks of the input.
///
/// Note: This buffers incomplete chunks. Thus `T: Clone` is necessary to copy
/// from the input slice to the internal buffer.
#[derive(Debug, Default)]
pub struct Chunked<T, const N: usize> {
    buf: ArrayVec<T, N>,
    first: Option<[T; N]>,
}

impl<T: Clone, const N: usize> Chunked<T, N> {
    pub fn next<'a>(&'a mut self, mut data: &'a [T]) -> ChunkIter<'a, T, N> {
        self.first = None;

        debug_assert!(!self.buf.is_full());

        if !self.buf.is_empty() {
            // Some data was buffered. Fill the buffer and pass this on as first item to
            // emit from the iterator.

            let n = self.buf.remaining_capacity().min(data.len());
            self.buf.extend(data[..n].iter().cloned());
            data = &data[n..];
            self.first = Some(
                self.buf
                    .take()
                    .into_inner()
                    .unwrap_or_else(|_| unreachable!()),
            );
        }

        // Iterator for full chunks without any buffer involved.
        let chunks = data.array_chunks::<N>();

        // Buffer remainder
        self.buf.extend(chunks.remainder().iter().cloned());

        ChunkIter {
            return_first: true,
            first: self.first.as_ref(),
            chunks,
        }
    }

    /// Returns the current remainder. This is useful to call this after you
    /// finished pushing data and want to process any data that is still
    /// buffered.
    pub fn remainder(&self) -> &[T] {
        self.buf.as_slice()
    }
}

pub struct ChunkIter<'a, T, const N: usize> {
    return_first: bool,
    first: Option<&'a [T; N]>,
    chunks: ArrayChunks<'a, T, N>,
}

impl<'a, T, const N: usize> Iterator for ChunkIter<'a, T, N> {
    type Item = &'a [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        match (self.first, self.return_first) {
            (Some(first), true) => {
                self.return_first = false;
                Some(first)
            }
            _ => self.chunks.next(),
        }
    }
}
