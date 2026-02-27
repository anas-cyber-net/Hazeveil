use hazeveil::ai_pattern::{embedded_patterns, blend_patterns, BehaviorPattern};

#[test]
fn test_library_has_200_plus_patterns() {
    let patterns = embedded_patterns();
    assert!(patterns.len() >= 200);
}

#[test]
fn test_blend_weights_normalized() {
    let patterns = embedded_patterns();
    let selected = &patterns[..4];
    let weights = vec![1.0, 2.0, 3.0, 4.0];
    let blended = blend_patterns(selected, &weights);
    assert!(blended.mouse_sigma > 0.0);
    assert!(blended.key_delay_ms > 0.0);
}

#[test]
fn test_all_patterns_valid() {
    let patterns = embedded_patterns();
    for p in &patterns {
        assert!(p.mouse_sigma > 0.0, "Pattern {} has invalid mouse_sigma", p.name);
        assert!(p.key_delay_ms > 0.0, "Pattern {} has invalid key_delay_ms", p.name);
        assert!(p.ghost_key_prob >= 0.0 && p.ghost_key_prob <= 1.0);
    }
}

#[test]
fn test_variant_names() {
    let patterns = embedded_patterns();
    let variants: Vec<&BehaviorPattern> = patterns.iter()
        .filter(|p| p.name.starts_with("variant-"))
        .collect();
    assert!(variants.len() >= 185);
}

#[test]
fn test_archetype_names_present() {
    let patterns = embedded_patterns();
    let names: Vec<&str> = patterns.iter().map(|p| p.name.as_str()).collect();
    assert!(names.contains(&"casual-browser"));
    assert!(names.contains(&"developer"));
    assert!(names.contains(&"gamer"));
    assert!(names.contains(&"methodical"));
}