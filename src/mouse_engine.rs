use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;
use crate::ai_pattern::BehaviorPattern;

pub struct MouseEngine { rng: SmallRng }

impl MouseEngine {
    pub fn new() -> Self { Self { rng: SmallRng::from_entropy() } }

    pub fn bezier_jitter(&mut self, pattern: &BehaviorPattern) -> (f64, f64) {
        let sigma = pattern.mouse_sigma * 0.5;
        let u1: f64 = self.rng.gen_range(0.001..0.999);
        let u2: f64 = self.rng.gen_range(0.001..0.999);
        let z0 = (-2.0_f64 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        let z1 = (-2.0_f64 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).sin();
        (z0 * sigma, z1 * sigma)
    }

    pub fn micro_tremor(&mut self, pattern: &BehaviorPattern) -> (f64, f64) {
        let t = pattern.mouse_sigma * 0.1;
        let dx = self.rng.gen_range(-t..t);
        let dy = self.rng.gen_range(-t..t);
        (dx, dy)
    }

    pub fn simulate(&mut self, pattern: &BehaviorPattern) -> (f64, f64) {
        let (bx, by) = self.bezier_jitter(pattern);
        let (tx, ty) = self.micro_tremor(pattern);
        (bx + tx, by + ty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_pattern::BehaviorPattern;
    #[test]
    fn test_mouse_variance() {
        let mut engine = MouseEngine::new();
        let pattern = BehaviorPattern::default();
        let positions: Vec<(f64, f64)> = (0..50).map(|_| engine.simulate(&pattern)).collect();
        let all_same = positions.windows(2).all(|w| w[0] == w[1]);
        assert!(!all_same, "Mouse positions must vary");
    }
    #[test]
    fn test_bezier_jitter() {
        let mut engine = MouseEngine::new();
        let pattern = BehaviorPattern::default();
        let (x, y) = engine.bezier_jitter(&pattern);
        assert!(x.is_finite() && y.is_finite());
    }
}