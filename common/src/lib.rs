use std::sync::Mutex;
use std::time::Instant;

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

pub struct TimeKeeper {
    start_time: Instant,
    threshold: u128,
}

impl TimeKeeper {
    pub fn new(threshold: u128) -> TimeKeeper {
        TimeKeeper {
            start_time: Instant::now(),
            threshold,
        }
    }

    pub fn is_time_over(&self) -> bool {
        (Instant::now() - self.start_time).as_millis() >= self.threshold
    }
}
