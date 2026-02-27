use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct TouchPoint { pub id: u8, pub x: f64, pub y: f64, pub pressure: f64 }

#[derive(Debug, Clone)]
pub enum Gesture {
    SingleTap(TouchPoint),
    TwoFingerScroll { dx: f64, dy: f64, pressure: f64 },
    Pinch { scale: f64, center: (f64, f64) },
    ThreeFingerSwipe { dx: f64, dy: f64 },
}

pub struct TouchpadEngine { rng: SmallRng, level: u8 }

impl TouchpadEngine {
    pub fn new(level: u8) -> Self { Self { rng: SmallRng::from_entropy(), level } }

    pub fn pressure_curve(&mut self) -> Vec<f64> {
        let steps = 10usize;
        let mut curve = Vec::with_capacity(steps);
        for i in 0..steps {
            let t = i as f64 / steps as f64;
            let base = (-((t - 0.5) * 4.0).powi(2)).exp();
            let noise = self.rng.gen_range(-0.05..0.05);
            curve.push((base + noise).clamp(0.0, 1.0));
        }
        curve
    }

    pub fn simulate_gesture(&mut self) -> Gesture {
        match self.rng.gen_range(0..4) {
            0 => Gesture::SingleTap(TouchPoint {
                id: 0,
                x: self.rng.gen_range(0.0..1.0),
                y: self.rng.gen_range(0.0..1.0),
                pressure: self.rng.gen_range(0.3..0.9),
            }),
            1 => Gesture::TwoFingerScroll {
                dx: self.rng.gen_range(-5.0..5.0),
                dy: self.rng.gen_range(-15.0..15.0),
                pressure: self.rng.gen_range(0.4..0.8),
            },
            2 => Gesture::Pinch {
                scale: self.rng.gen_range(0.8..1.2),
                center: (self.rng.gen_range(0.3..0.7), self.rng.gen_range(0.3..0.7)),
            },
            _ => Gesture::ThreeFingerSwipe {
                dx: self.rng.gen_range(-20.0..20.0),
                dy: self.rng.gen_range(-5.0..5.0),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pressure_curve() {
        let mut engine = TouchpadEngine::new(3);
        let curve = engine.pressure_curve();
        assert_eq!(curve.len(), 10);
        assert!(curve.iter().all(|&v| v >= 0.0 && v <= 1.0));
    }
    #[test]
    fn test_gesture_variety() {
        let mut engine = TouchpadEngine::new(3);
        let gestures: Vec<Gesture> = (0..20).map(|_| engine.simulate_gesture()).collect();
        assert_eq!(gestures.len(), 20);
    }
}