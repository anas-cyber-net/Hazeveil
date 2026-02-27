use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;

pub struct AudioEngine { rng: SmallRng }

impl AudioEngine {
    pub fn new() -> Self { Self { rng: SmallRng::from_entropy() } }

    pub fn keystroke_sound_delay_us(&mut self) -> u64 {
        self.rng.gen_range(1200..4800)
    }

    pub fn should_emit_sound(&mut self, probability: f64) -> bool {
        self.rng.gen::<f64>() < probability
    }
}