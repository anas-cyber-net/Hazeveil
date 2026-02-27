use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;
use std::time::Duration;

pub struct TimingEngine {
    rng: SmallRng,
    level: u8,
}

impl TimingEngine {
    pub fn new(level: u8) -> Self {
        Self { rng: SmallRng::from_entropy(), level }
    }

    pub fn base_delay(&mut self) -> Duration {
        let base_us = match self.level {
            1 => 10u64,
            2 => 30,
            3 => 80,
            4 => 150,
            5 => 280,
            _ => 80,
        };
        let jitter = self.rng.gen_range(0..base_us / 2);
        Duration::from_micros(base_us + jitter)
    }

    pub fn poisson_delay(&mut self) -> Duration {
        let lambda = match self.level {
            1 => 5.0f64,
            2 => 15.0,
            3 => 40.0,
            4 => 80.0,
            5 => 150.0,
            _ => 40.0,
        };
        let u: f64 = self.rng.gen_range(0.001..0.999);
        let delay_us = (-lambda * u.ln()) as u64;
        Duration::from_micros(delay_us)
    }

    pub fn gaussian_jitter_us(&mut self, sigma: f64) -> f64 {
        let u1: f64 = self.rng.gen_range(0.001..0.999);
        let u2: f64 = self.rng.gen_range(0.001..0.999);
        let z = (-2.0_f64 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        z * sigma
    }

    pub fn level(&self) -> u8 { self.level }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_timing_variance() {
        let mut engine = TimingEngine::new(3);
        let delays: Vec<u64> = (0..100)
            .map(|_| engine.base_delay().as_micros() as u64)
            .collect();
        let mean = delays.iter().sum::<u64>() as f64 / delays.len() as f64;
        let variance = delays.iter()
            .map(|&d| (d as f64 - mean).powi(2))
            .sum::<f64>() / delays.len() as f64;
        assert!(variance > 0.0, "Timing must have variance");
    }
    #[test]
    fn test_gaussian_jitter() {
        let mut engine = TimingEngine::new(3);
        let samples: Vec<f64> = (0..100).map(|_| engine.gaussian_jitter_us(10.0)).collect();
        let has_positive = samples.iter().any(|&s| s > 0.0);
        let has_negative = samples.iter().any(|&s| s < 0.0);
        assert!(has_positive && has_negative);
    }
}