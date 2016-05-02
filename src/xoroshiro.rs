use rand::{Rng, SeedableRng};

/// An implementation of the [Xoroshiro128+](http://xoroshiro.di.unimi.it) random number generator:
///
/// > Instead of perpetuating Marsaglia's tradition of xorshift as a basic operation, xoroshiro128+
/// > uses a carefully handcrafted shift/rotate-based linear transformation designed in
/// > collaboration with David Blackman. The result is a significant improvement in speed (well
/// > below a nanosecond per integer) and a significant improvement in statistical quality, as
/// > detected by the long-range tests of PractRand. xoroshiro128+ is our current suggestion for
/// > replacing low-quality generators commonly found in programming languages.
///
/// It produces better results than XorShift and is faster.
#[derive(Clone, Debug)]
pub struct Xoroshiro128Rng {
    state: [u64; 2],
}

impl Xoroshiro128Rng {
    /// Returns a new `Xoroshiro128Rng` instance which is not seeded.
    ///
    /// The initial values of this RNG are constants, so all generators created by this function
    /// will yield the same stream of random numbers. It is highly recommended that this is created
    /// through `SeedableRng` instead of this function.
    pub fn new_unseeded() -> Xoroshiro128Rng {
        // Hand-crafted, artisanally-produced, locally-curated random numbers.
        Xoroshiro128Rng::from_seed([0xaeecf86f7878dd75, 0x1cd153642e72622])
    }
}

impl Rng for Xoroshiro128Rng {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        let s0 = self.state[0];
        let mut s1 = self.state[1];
        let result = s0.wrapping_mul(s1);

        s1 ^= s0;
        self.state[0] = s0.rotate_left(55) ^ s1 ^ (s1 << 14);
        self.state[1] = s1.rotate_left(36);

        result
    }
}

impl SeedableRng<[u64; 2]> for Xoroshiro128Rng {
    fn reseed(&mut self, seed: [u64; 2]) {
        self.state = seed;
    }

    fn from_seed(seed: [u64; 2]) -> Xoroshiro128Rng {
        Xoroshiro128Rng { state: seed }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rand::{Rng, SeedableRng};

    #[test]
    fn output() {
        let mut rng = Xoroshiro128Rng::new_unseeded();

        let v: Vec<u32> = rng.gen_iter().take(6).collect();

        assert_eq!(v,
                   vec![4018128778, 2230751931, 3794516864, 1640624936, 1749030944, 1428004884]);
    }

    #[test]
    fn overflow() {
        let mut rng = Xoroshiro128Rng::from_seed([!0, 54]);
        rng.next_u32();
    }
}
