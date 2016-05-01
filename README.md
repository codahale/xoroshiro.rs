# xoroshiro
[![Build Status](https://travis-ci.org/codahale/xoroshiro.svg)](https://travis-ci.org/codahale/xoroshiro)
[![Apache V2 License](http://img.shields.io/badge/license-Apache%20V2-blue.svg)](https://github.com/codahale/xoroshiro/blob/master/LICENSE)

A Rust implementation of the [Xoroshiro128+](http://xoroshiro.di.unimi.it) PRNG.

The Xoroshiro128+ algorithm is not suitable for cryptographic purposes but
provides an excellent combination of speed and unpredictability. It is only
slightly slower than `rand::XorShiftRng` but provides much higher-quality
output.
