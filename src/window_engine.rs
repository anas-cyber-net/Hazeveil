use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;
use crate::ai_pattern::BehaviorPattern;
use std::time::Duration;

pub struct WindowEngine { rng: SmallRng }

impl WindowEngine {
    pub fn new() -> Self { Self { rng: SmallRng::from_entropy() } }

    pub fn focus_delay(&mut self, pattern: &BehaviorPattern) -> Duration {
        let base_ms = (pattern.pause_frequency * 1000.0) as u64;
        let jitter = self.rng.gen_range(0..500);
        Duration::from_millis(base_ms + jitter)
    }

    pub fn should_switch(&mut self, pattern: &BehaviorPattern) -> bool {
        self.rng.gen::<f64>() < pattern.window_switch_rate
    }

    pub fn distraction_pause(&mut self) -> Duration {
        Duration::from_millis(self.rng.gen_range(200..2000))
    }
}