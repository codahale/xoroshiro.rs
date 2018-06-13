use rand::{OsRng, Rng, SeedableRng};
use std::io;

/// An implementation of the [Xoroshiro128+](http://xoroshiro.di.unimi.it) random number generator:
///
/// > Instead of perpetuating Marsaglia's tradition of xorshift as a basic operation, xoroshiro128+
/// > uses a carefully handcrafted shift/rotate-based linear transformation designed in
/// > collaboration with David Blackman. The result is a significant improvement in speed (well
/// > below a nanosecond per integer) and a significant improvement in statistical quality, as
/// > detected by the long-range tests of `PractRand`. xoroshiro128+ is our current suggestion for
/// > replacing low-quality generators commonly found in programming languages.
///
/// It produces better results than `XorShiftRng` and is faster.
#[derive(Clone, Debug)]
pub struct Xoroshiro128Rng {
    state: [u64; 2],
}

impl Xoroshiro128Rng {
    /// Returns a new `Xoroshiro128Rng` instance with a random seed.
    ///
    /// N.B.: The seed is produced with `OsRng` which can be slow and may fail.
    pub fn new() -> io::Result<Xoroshiro128Rng> {
        OsRng::new().map(|mut r| {
            let mut rng = Xoroshiro128Rng::new_unseeded();
            while rng.state[0] == 0 && rng.state[1] == 0 {
                rng.state = r.gen();
            }
            rng.next_u64();
            rng
        })
    }

    /// Returns a new `Xoroshiro128Rng` instance with a constant seed.
    ///
    /// The initial values of this RNG are constants, so all generators created by this function
    /// will yield the same stream of random numbers. It is highly recommended that this is created
    /// through `SeedableRng` instead of this function.
    pub fn new_unseeded() -> Xoroshiro128Rng {
        Xoroshiro128Rng::from_seed([0, 0])
    }

    /// Jumps ahead 2^64 outputs. It is equivalent to 2^64 calls to next(); it can be used to
    /// generate 2^64 non-overlapping subsequences for parallel computations.
    pub fn jump(&mut self) {
        let mut s0 = 0;
        let mut s1 = 0;

        let jump: [u64; 2] = [0xbeac0467eba5facb, 0xd86b048b86aa9922];
        for v in jump.into_iter() {
            for b in 0..64 {
                if v & (1 << b) != 0 {
                    s0 ^= self.state[0];
                    s1 ^= self.state[1];
                }
                self.next_u64();
            }
        }

        self.state[0] = s0;
        self.state[1] = s1;
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
        let result = s0.wrapping_add(s1);

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
    fn new() {
        let rng = Xoroshiro128Rng::new();
        assert!(rng.map(|mut r| r.gen::<u8>()).is_ok());
    }

    #[test]
    fn output() {
        let mut rng = Xoroshiro128Rng::new_unseeded();
        rng.reseed([0xaeecf86f7878dd75, 0x1cd153642e72622]);

        let v: Vec<u32> = rng.gen_iter().take(6).collect();

        assert_eq!(
            v,
            vec![
                3143631767, 3860126924, 1781643561, 1911529541, 113917100, 2025972731,
            ]
        );
    }

    #[test]
    fn overflow() {
        let mut rng = Xoroshiro128Rng::from_seed([!0, 54]);
        rng.next_u32();
    }

    #[test]
    fn jump() {
        let mut rng = Xoroshiro128Rng::new_unseeded();
        rng.reseed([0xaeecf86f7878dd75, 0x1cd153642e72622]);
        rng.jump();

        let v: Vec<u32> = rng.gen_iter().take(6).collect();

        assert_eq!(
            v,
            vec![
                3564949728, 3479480372, 1003893697, 3066975437, 1909106551, 3084299971,
            ]
        );
    }
}
