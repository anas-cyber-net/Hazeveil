use hazeveil::mouse_engine::MouseEngine;
use hazeveil::keyboard_engine::KeyboardEngine;
use hazeveil::touchpad_engine::TouchpadEngine;
use hazeveil::scroll_engine::ScrollEngine;
use hazeveil::timing_engine::TimingEngine;
use hazeveil::ai_pattern::BehaviorPattern;

#[test]
fn test_mouse_engine_produces_variance() {
    let mut engine = MouseEngine::new();
    let pattern = BehaviorPattern::default();
    let results: Vec<(f64, f64)> = (0..50).map(|_| engine.simulate(&pattern)).collect();
    let all_same = results.windows(2).all(|w| w[0] == w[1]);
    assert!(!all_same);
}

#[test]
fn test_keyboard_engine_events() {
    let mut engine = KeyboardEngine::new();
    let pattern = BehaviorPattern::default();
    for _ in 0..20 { engine.next_event(&pattern); }
}

#[test]
fn test_touchpad_pressure_valid() {
    let mut engine = TouchpadEngine::new(3);
    let curve = engine.pressure_curve();
    assert!(curve.iter().all(|&v| v >= 0.0 && v <= 1.0));
}

#[test]
fn test_scroll_velocity_positive() {
    let mut engine = ScrollEngine::new();
    let pattern = BehaviorPattern::default();
    for _ in 0..20 {
        let event = engine.simulate(&pattern);
        assert!(event.velocity > 0.0);
    }
}

#[test]
fn test_timing_engine_base_delay() {
    let mut engine = TimingEngine::new(3);
    let delay = engine.base_delay();
    assert!(delay.as_micros() > 0);
}