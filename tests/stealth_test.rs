use hazeveil::timing_engine::TimingEngine;
use hazeveil::mouse_engine::MouseEngine;
use hazeveil::ai_pattern::BehaviorPattern;

#[test]
fn test_timing_has_sufficient_entropy() {
    let mut engine = TimingEngine::new(5);
    let delays: Vec<u64> = (0..200)
        .map(|_| engine.base_delay().as_micros() as u64)
        .collect();
    let unique: std::collections::HashSet<u64> = delays.iter().cloned().collect();
    assert!(unique.len() > 10, "Timing must have sufficient entropy");
}

#[test]
fn test_mouse_positions_not_deterministic() {
    let mut engine = MouseEngine::new();
    let pattern = BehaviorPattern::default();
    let positions: Vec<(i64, i64)> = (0..100)
        .map(|_| {
            let (x, y) = engine.simulate(&pattern);
            (x as i64, y as i64)
        })
        .collect();
    let unique: std::collections::HashSet<(i64, i64)> = positions.iter().cloned().collect();
    assert!(unique.len() > 5);
}

#[test]
fn test_level_5_higher_variance_than_level_1() {
    let mut engine1 = TimingEngine::new(1);
    let mut engine5 = TimingEngine::new(5);
    let delays1: Vec<u64> = (0..100).map(|_| engine1.base_delay().as_micros() as u64).collect();
    let delays5: Vec<u64> = (0..100).map(|_| engine5.base_delay().as_micros() as u64).collect();
    let mean1 = delays1.iter().sum::<u64>() as f64 / 100.0;
    let mean5 = delays5.iter().sum::<u64>() as f64 / 100.0;
    assert!(mean5 > mean1, "Level 5 must have higher base delay than level 1");
}