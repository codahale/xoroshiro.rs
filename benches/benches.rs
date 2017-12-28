extern crate criterion;
extern crate rand;
extern crate xoroshiro;

use criterion::{Bencher, Criterion, Fun};
use rand::{Rng, SeedableRng, XorShiftRng};
use xoroshiro::Xoroshiro128Rng;

fn bench_xorshift_u32(b: &mut Bencher, _: &u32) {
    let mut rng = XorShiftRng::from_seed([100, 200, 300, 400]);
    b.iter(|| rng.next_u32());
}

fn bench_xorshift_u64(b: &mut Bencher, _: &u32) {
    let mut rng = XorShiftRng::from_seed([100, 200, 300, 400]);
    b.iter(|| rng.next_u64());
}

fn bench_xoroshiro(b: &mut Bencher, _: &u32) {
    let mut rng = Xoroshiro128Rng::from_seed([100, 200]);
    b.iter(|| rng.next_u32());
}

#[test]
fn comparison() {
    let xorshift_u32 = Fun::new("XorShiftRng-u32", bench_xorshift_u32);
    let xorshift_u64 = Fun::new("XorShiftRng-u64", bench_xorshift_u64);
    let xoroshiro = Fun::new("Xoroshiro128Rng-u64", bench_xoroshiro);
    let rngs = vec![xorshift_u32, xorshift_u64, xoroshiro];

    Criterion::default().bench_functions("Rng", rngs, &1);
}
