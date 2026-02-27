use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;
use log::info;
use crate::ai_pattern::{blend_patterns, embedded_patterns, BehaviorPattern};
use crate::mouse_engine::MouseEngine;
use crate::keyboard_engine::KeyboardEngine;
use crate::touchpad_engine::TouchpadEngine;
use crate::scroll_engine::ScrollEngine;
use crate::timing_engine::TimingEngine;
use crate::window_engine::WindowEngine;
use crate::clipboard_engine::ClipboardEngine;
use crate::audio_engine::AudioEngine;
use crate::context_detector::ContextDetector;

pub struct CoreEngine {
    level: u8,
    pattern_name: String,
    running: Arc<AtomicBool>,
    rng: SmallRng,
    mouse: MouseEngine,
    keyboard: KeyboardEngine,
    touchpad: TouchpadEngine,
    scroll: ScrollEngine,
    timing: TimingEngine,
    window: WindowEngine,
    clipboard: ClipboardEngine,
    audio: AudioEngine,
}

impl CoreEngine {
    pub fn new(level: u8, pattern: &str, running: Arc<AtomicBool>) -> Self {
        Self {
            level,
            pattern_name: pattern.to_string(),
            running,
            rng: SmallRng::from_entropy(),
            mouse: MouseEngine::new(),
            keyboard: KeyboardEngine::new(),
            touchpad: TouchpadEngine::new(level),
            scroll: ScrollEngine::new(),
            timing: TimingEngine::new(level),
            window: WindowEngine::new(),
            clipboard: ClipboardEngine::new(),
            audio: AudioEngine::new(),
        }
    }

    pub async fn run(&mut self) {
        let context = ContextDetector::detect();
        let multiplier = context.multiplier();
        let all_patterns = embedded_patterns();
        let mut active_blend = self.build_blend(&all_patterns);
        let mut cycle: u64 = 0;
        info!("HazeVeil running at level {}. Ctrl+C to stop.", self.level);

        while self.running.load(Ordering::SeqCst) {
            cycle += 1;

            if cycle % 50 == 0 {
                self.drift_blend(&all_patterns, &mut active_blend);
            }
            if cycle % 100 == 0 {
                active_blend = self.build_blend(&all_patterns);
            }

            self.mouse.simulate(&active_blend);
            self.keyboard.next_event(&active_blend);
            self.touchpad.simulate_gesture();
            self.scroll.simulate(&active_blend);
            self.clipboard.simulate_clipboard(&active_blend);

            if self.window.should_switch(&active_blend) {
                let _pause = self.window.distraction_pause();
            }

            if self.audio.should_emit_sound(active_blend.audio_prob) {
                let _delay = self.audio.keystroke_sound_delay_us();
            }

            if cycle % 600 == 0 {
                let blend_name = active_blend.name.clone();
                let op_result = self.rng.gen::<f64>();
                let random_op_result = op_result;
                info!(
                    "Heartbeat: cycle={} pattern={} op={:.4}",
                    cycle, blend_name, random_op_result
                );
            }

            let delay = self.timing.base_delay();
            let adjusted = std::time::Duration::from_micros(
                (delay.as_micros() as f64 * multiplier) as u64
            );
            tokio::time::sleep(adjusted).await;
        }

        info!("CoreEngine stopped after {} cycles.", cycle);
    }

    pub fn stop(&self) { self.running.store(false, Ordering::SeqCst); }

    fn build_blend(&mut self, all: &[BehaviorPattern]) -> BehaviorPattern {
        let count = self.rng.gen_range(3..=5);
        let mut selected = Vec::with_capacity(count);
        let mut weights = Vec::with_capacity(count);

        if self.pattern_name != "random" {
            if let Some(p) = all.iter().find(|p| p.name == self.pattern_name) {
                selected.push(p.clone());
                weights.push(self.rng.gen_range(0.4..0.7));
            }
        }

        while selected.len() < count {
            let idx = self.rng.gen_range(0..all.len());
            selected.push(all[idx].clone());
            weights.push(self.rng.gen_range(0.1..1.0));
        }

        blend_patterns(&selected, &weights)
    }

    fn drift_blend(&mut self, all: &[BehaviorPattern], current: &mut BehaviorPattern) {
        let idx = self.rng.gen_range(0..all.len());
        let drift_weight = self.rng.gen_range(0.05..0.2);
        let new_blend = blend_patterns(
            &[current.clone(), all[idx].clone()],
            &[1.0 - drift_weight, drift_weight],
        );
        *current = new_blend;
    }
}