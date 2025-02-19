use rand::distr::uniform::{SampleRange, SampleUniform};
use rand::{Rng as _, SeedableRng as _};

pub struct Random {
    rng: rand::rngs::SmallRng,
    die_value_stat: [usize; 6],
}

impl Random {
    pub fn new() -> Self {
        let seed = pd::system::time_since_epoch().as_micros() as u64;

        Self {
            rng: rand::rngs::SmallRng::seed_from_u64(seed),
            die_value_stat: [0; 6],
        }
    }

    pub fn in_range<T: SampleUniform, R: SampleRange<T>>(&mut self, range: R) -> T {
        self.rng.random_range(range)
    }

    pub fn element<'a, T>(&mut self, slice: &'a [T]) -> &'a T {
        &slice[self.in_range(0..slice.len())]
    }

    pub fn die_value(&mut self) -> u8 {
        let value = self.in_range(1..=6);
        self.die_value_stat[value as usize - 1] += 1;
        value
    }

    pub fn die_value_stat(&self) -> [usize; 6] {
        self.die_value_stat
    }
}
