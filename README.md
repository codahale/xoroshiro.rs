# xoroshiro
[![Build Status](https://travis-ci.org/codahale/xoroshiro.rs.svg)](https://travis-ci.org/codahale/xoroshiro.rs)
[![Apache V2 License](http://img.shields.io/badge/license-Apache%20V2-blue.svg)](https://github.com/codahale/xoroshiro.rs/blob/master/LICENSE)

A Rust implementation of the [Xoroshiro128+](http://xoroshiro.di.unimi.it) PRNG.

The Xoroshiro128+ algorithm is not suitable for cryptographic purposes but
provides an excellent combination of speed and unpredictability.

It produces high-quality output than Rust's default `XorShiftRng`, and is faster:

```
next_u32/XorShift       time:   [1.4832 ns 1.4964 ns 1.5123 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
next_u32/Xoroshiro128   time:   [1.0594 ns 1.0796 ns 1.1051 ns]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

next_u64/XorShift       time:   [2.2982 ns 2.3233 ns 2.3526 ns]
Found 13 outliers among 100 measurements (13.00%)
  10 (10.00%) high mild
  3 (3.00%) high severe
next_u64/Xoroshiro128   time:   [1.0038 ns 1.0079 ns 1.0126 ns]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe
```
