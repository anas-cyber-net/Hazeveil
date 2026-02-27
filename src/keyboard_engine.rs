use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;
use crate::ai_pattern::BehaviorPattern;

#[derive(Debug, Clone)]
pub enum KeyEvent {
    Press { keycode: u32, delay_us: u64 },
    Release { keycode: u32, dwell_us: u64 },
    Ghost { keycode: u32 },
}

pub struct KeyboardEngine { rng: SmallRng }

impl KeyboardEngine {
    pub fn new() -> Self { Self { rng: SmallRng::from_entropy() } }

    pub fn next_event(&mut self, pattern: &BehaviorPattern) -> KeyEvent {
        let roll: f64 = self.rng.gen();
        if roll < pattern.ghost_key_prob {
            return KeyEvent::Ghost { keycode: self.rng.gen_range(1..128) };
        }
        let base_delay = (pattern.key_delay_ms * 1000.0) as u64;
        let jitter = self.rng.gen_range(0..base_delay / 4 + 1);
        let delay_us = base_delay + jitter;
        let dwell_us = self.rng.gen_range(20_000..80_000);
        if self.rng.gen_bool(0.5) {
            KeyEvent::Press { keycode: self.rng.gen_range(1..128), delay_us }
        } else {
            KeyEvent::Release { keycode: self.rng.gen_range(1..128), dwell_us }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_pattern::BehaviorPattern;
    #[test]
    fn test_keyboard_events() {
        let mut engine = KeyboardEngine::new();
        let pattern = BehaviorPattern::default();
        let events: Vec<KeyEvent> = (0..20).map(|_| engine.next_event(&pattern)).collect();
        assert_eq!(events.len(), 20);
    }
}