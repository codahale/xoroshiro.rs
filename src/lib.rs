//! An implementation of the [Xoroshiro128+](http://xoroshiro.di.unimi.it) random number generator.
//!
//! ```ignore
//! use rand::{Rng, SeedableRng};
//! use xoroshiro::Xoroshiro128Rng;
//!
//! let mut rng = xoroshiro::Xoroshiro128Rng::from_seed([100, 200]);
//!
//! let x = rng.gen_range(10, 100);
//!
//! assert!(x >= 10);
//! assert!(x <= 100);
//!
//! ```

#![feature(plugin)]
#![plugin(clippy)]

extern crate rand;

mod xoroshiro;

pub use xoroshiro::Xoroshiro128Rng;
