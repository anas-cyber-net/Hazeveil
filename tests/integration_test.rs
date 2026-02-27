use hazeveil::ai_pattern::{embedded_patterns, blend_patterns};
use hazeveil::context_detector::ContextDetector;

#[test]
fn test_full_pattern_blend_pipeline() {
    let all = embedded_patterns();
    assert!(all.len() >= 200);
    let selected = vec![all[0].clone(), all[5].clone(), all[10].clone()];
    let weights = vec![0.5, 0.3, 0.2];
    let blended = blend_patterns(&selected, &weights);
    assert_eq!(blended.name, "blend");
    assert!(blended.mouse_sigma > 0.0);
    assert!(blended.key_delay_ms > 0.0);
    assert!(blended.scroll_velocity > 0.0);
}

#[test]
fn test_context_detector_runs() {
    let ctx = ContextDetector::detect();
    assert!(ctx.multiplier() > 0.0);
}
