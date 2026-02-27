use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;
use crate::ai_pattern::BehaviorPattern;

#[derive(Debug, Clone)]
pub enum ClipboardOp {
    Copy { delay_ms: u64 },
    Paste { delay_ms: u64 },
    Cut { delay_ms: u64 },
}

pub struct ClipboardEngine { rng: SmallRng }

impl ClipboardEngine {
    pub fn new() -> Self { Self { rng: SmallRng::from_entropy() } }

    pub fn simulate_clipboard(&mut self, pattern: &BehaviorPattern) -> Option<ClipboardOp> {
        let roll = self.rng.gen::<f64>();
        if roll > pattern.clipboard_usage_rate { return None; }
        let delay_ms = self.rng.gen_range(50..500);
        let op_roll = self.rng.gen::<f64>();
        Some(if op_roll < 0.5 {
            ClipboardOp::Copy { delay_ms }
        } else if op_roll < 0.8 {
            ClipboardOp::Paste { delay_ms }
        } else {
            ClipboardOp::Cut { delay_ms }
        })
    }
}