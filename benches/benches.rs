extern crate criterion;
extern crate rand;
extern crate xoroshiro;

use criterion::{Bencher, Fun, Criterion};
use rand::{Rng, SeedableRng, XorShiftRng};
use xoroshiro::Xoroshiro128Rng;

fn bench_xorshift(b: &mut Bencher, _: &u32) {
    let mut rng = XorShiftRng::from_seed([100, 200, 300, 400]);
    b.iter(|| rng.next_u32());
}

fn bench_xoroshiro(b: &mut Bencher, _: &u32) {
    let mut rng = Xoroshiro128Rng::from_seed([100, 200]);
    b.iter(|| rng.next_u32());
}

#[test]
fn comparison() {
    let xorshift = Fun::new("XorShiftRng", bench_xorshift);
    let xoroshiro = Fun::new("Xoroshiro128Rng", bench_xoroshiro);
    let rngs = vec![xorshift, xoroshiro];

    Criterion::default()
        .bench_functions("Rng", rngs, &1);
}
