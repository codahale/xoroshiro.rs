# xoroshiro
[![Build Status](https://travis-ci.org/codahale/xoroshiro.rs.svg)](https://travis-ci.org/codahale/xoroshiro.rs)
[![Apache V2 License](http://img.shields.io/badge/license-Apache%20V2-blue.svg)](https://github.com/codahale/xoroshiro.rs/blob/master/LICENSE)

A Rust implementation of the [Xoroshiro128+](http://xoroshiro.di.unimi.it) PRNG.

The Xoroshiro128+ algorithm is not suitable for cryptographic purposes but
provides an excellent combination of speed and unpredictability.

It's faster than `rand::XorShiftRng` and produces higher-quality output:

```
running 4 tests
test xoroshiro128_fill_bytes ... bench:     955,430 ns/iter (+/- 164,936) = 1097 MB/s
test xoroshiro128_next_u32   ... bench:           1 ns/iter (+/- 0)
test xorshift_fill_bytes     ... bench:   1,222,814 ns/iter (+/- 140,058) = 857 MB/s
test xorshift_next_u32       ... bench:           1 ns/iter (+/- 0)
```
