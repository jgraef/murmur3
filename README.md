[![MIT license](https://img.shields.io/badge/license-MIT-brightgreen)](https://opensource.org/licenses/MIT)
![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)

# `murmur3`

## Overview

This crate provides an implementation of the [Murmur3] non-cryptographic
hash function.

Currently only the `x64_128` variant of Murmur3 is implemented. There is a
more mature version of murmur3 available [here](https://github.com/stusmall/murmur3).

## Why a new crate?

The before mentioned crate that already exists for Murmur3, only implements
it as a single function. Therefore it's impossible to create a hasher state
and update it in a streaming fashion. Furthermore this crate also provides
wrappers for [`std::io::Read`] and [`std::io::Write`] that compute a hash
function while data is being read or written.

[Murmur3]: https://en.wikipedia.org/wiki/MurmurHash

## License

Licensed under MIT license ([LICENSE](LICENSE) or https://opensource.org/licenses/MIT)
