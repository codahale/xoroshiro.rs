#![feature(convert,test)]

extern crate xoroshiro;
extern crate rand;
extern crate test;

use xoroshiro::Xoroshiro128Rng;
use test::Bencher;
use rand::{Rng, XorShiftRng};

#[bench]
fn xoroshiro128_next_u32(b: &mut Bencher) {
    let mut rng = Xoroshiro128Rng::new_unseeded();

    b.iter(|| rng.next_u32())
}

#[bench]
fn xoroshiro128_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024 * 1024;
    let mut rng = Xoroshiro128Rng::new_unseeded();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| rng.fill_bytes(x.as_mut_slice()))
}

#[bench]
fn xorshift_next_u32(b: &mut Bencher) {
    let mut rng = XorShiftRng::new_unseeded();

    b.iter(|| rng.next_u32())
}

#[bench]
fn xorshift_fill_bytes(b: &mut Bencher) {
    b.bytes = 1024 * 1024;
    let mut rng = XorShiftRng::new_unseeded();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| rng.fill_bytes(x.as_mut_slice()))
}
