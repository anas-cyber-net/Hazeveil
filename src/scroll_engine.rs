use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;
use crate::ai_pattern::BehaviorPattern;

#[derive(Debug, Clone)]
pub struct ScrollEvent { pub dx: f64, pub dy: f64, pub velocity: f64 }

pub struct ScrollEngine { rng: SmallRng }

impl ScrollEngine {
    pub fn new() -> Self { Self { rng: SmallRng::from_entropy() } }

    pub fn simulate(&mut self, pattern: &BehaviorPattern) -> ScrollEvent {
        let base_vel = pattern.scroll_velocity;
        let u1: f64 = self.rng.gen_range(0.001..0.999);
        let u2: f64 = self.rng.gen_range(0.001..0.999);
        let drift = (-2.0_f64 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos() * 0.3;
        let velocity = (base_vel + drift).max(0.1);
        let dy = self.rng.gen_range(-velocity..velocity);
        let dx = self.rng.gen_range(-velocity * 0.1..velocity * 0.1);
        ScrollEvent { dx, dy, velocity }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_pattern::BehaviorPattern;
    #[test]
    fn test_scroll_variance() {
        let mut engine = ScrollEngine::new();
        let pattern = BehaviorPattern::default();
        let events: Vec<ScrollEvent> = (0..30).map(|_| engine.simulate(&pattern)).collect();
        let velocities: Vec<f64> = events.iter().map(|e| e.velocity).collect();
        let mean = velocities.iter().sum::<f64>() / velocities.len() as f64;
        let variance = velocities.iter().map(|&v| (v - mean).powi(2)).sum::<f64>() / velocities.len() as f64;
        assert!(variance > 0.0);
    }
}