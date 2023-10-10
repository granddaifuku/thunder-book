use std::sync::Mutex;

use once_cell::sync::Lazy;
use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug)]
struct RandomGenerator {
    rng: StdRng,
}

impl RandomGenerator {
    fn new() -> Self {
        Self {
            rng: SeedableRng::seed_from_u64(0),
        }
    }

    fn set_rng(&mut self, seed: u64) {
        self.rng = rand::SeedableRng::seed_from_u64(seed);
    }

    fn gen_range(&mut self, limit: usize) -> usize {
        self.rng.gen_range(0..limit)
    }
}

static GENERATOR: Lazy<Mutex<RandomGenerator>> = Lazy::new(|| Mutex::new(RandomGenerator::new()));

pub fn init_random_generator(seed: u64) {
    GENERATOR.lock().unwrap().set_rng(seed);
}

pub fn get_random(limit: usize) -> usize {
    GENERATOR.lock().unwrap().gen_range(limit)
}
