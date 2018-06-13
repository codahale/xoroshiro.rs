#[macro_use]
extern crate criterion;
extern crate rand;
extern crate xoroshiro;

use criterion::{Criterion, Fun};
use rand::{Rng, SeedableRng, XorShiftRng};
use xoroshiro::Xoroshiro128Rng;

fn bench_prng(c: &mut Criterion) {
    let xorshift_u32 = Fun::new("XorShift", |b, _| {
        let mut rng = XorShiftRng::from_seed([100, 200, 300, 400]);
        return b.iter(|| rng.next_u32());
    });
    let xoroshiro128_u32 = Fun::new("Xoroshiro128", |b, _| {
        let mut rng = Xoroshiro128Rng::from_seed([100, 200]);
        return b.iter(|| rng.next_u32());
    });
    c.bench_functions("next_u32", vec![xorshift_u32, xoroshiro128_u32], 0);

    let xorshift_u64 = Fun::new("XorShift", |b, _| {
        let mut rng = XorShiftRng::from_seed([100, 200, 300, 400]);
        return b.iter(|| rng.next_u64());
    });
    let xoroshiro128_u64 = Fun::new("Xoroshiro128", |b, _| {
        let mut rng = Xoroshiro128Rng::from_seed([100, 200]);
        return b.iter(|| rng.next_u64());
    });
    c.bench_functions("next_u64", vec![xorshift_u64, xoroshiro128_u64], 0);
}

criterion_group!(benches, bench_prng);
criterion_main!(benches);
