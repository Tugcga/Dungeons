use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Debug)]
pub struct PseudoRandom {
    random: ChaCha8Rng
}

impl PseudoRandom {
    pub fn new(seed: u64) -> PseudoRandom {
        return PseudoRandom{ random: rand_chacha::ChaCha8Rng::seed_from_u64(seed) };
    }

    pub fn next(&mut self, in_min: usize, in_max: usize) -> usize {
        return self.random.gen_range(in_min..=in_max);
    }

    pub fn next_odd(&mut self, in_min: usize, in_max: usize) -> usize {
        let next_value: usize =self.next(in_min, in_max);
        if next_value % 2 != 0 {
            return next_value;
        }
        else {
            if next_value < in_max {
                return next_value + 1;
            }
            else if next_value == 0 {
                return 1;
            }
            else{
                return next_value - 1;
            }
        }
    }
}