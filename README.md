# xoroshiro
[![Build Status](https://travis-ci.org/codahale/xoroshiro.rs.svg)](https://travis-ci.org/codahale/xoroshiro.rs)
[![Apache V2 License](http://img.shields.io/badge/license-Apache%20V2-blue.svg)](https://github.com/codahale/xoroshiro.rs/blob/master/LICENSE)

A Rust implementation of the [Xoroshiro128+](http://xoroshiro.di.unimi.it) PRNG.

The Xoroshiro128+ algorithm is not suitable for cryptographic purposes but
provides an excellent combination of speed and unpredictability.

It produces high-quality output than Rust's default `XorShiftRng`, and is faster:

![results](https://github.com/codahale/xoroshiro.rs/blob/master/README.md)
