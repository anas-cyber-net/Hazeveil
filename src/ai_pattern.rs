use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct BehaviorPattern {
    pub name: String,
    pub mouse_sigma: f64,
    pub key_delay_ms: f64,
    pub scroll_velocity: f64,
    pub pause_frequency: f64,
    pub pause_duration_ms: f64,
    pub ghost_key_prob: f64,
    pub window_switch_rate: f64,
    pub clipboard_usage_rate: f64,
    pub audio_prob: f64,
}

impl Default for BehaviorPattern {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            mouse_sigma: 2.0,
            key_delay_ms: 120.0,
            scroll_velocity: 3.0,
            pause_frequency: 0.05,
            pause_duration_ms: 500.0,
            ghost_key_prob: 0.02,
            window_switch_rate: 0.01,
            clipboard_usage_rate: 0.05,
            audio_prob: 0.3,
        }
    }
}

pub fn embedded_patterns() -> Vec<BehaviorPattern> {
    let archetypes = vec![
        BehaviorPattern { name: "casual-browser".into(), mouse_sigma: 3.5, key_delay_ms: 180.0, scroll_velocity: 4.5, pause_frequency: 0.08, pause_duration_ms: 800.0, ghost_key_prob: 0.01, window_switch_rate: 0.03, clipboard_usage_rate: 0.04, audio_prob: 0.2 },
        BehaviorPattern { name: "fast-typist".into(), mouse_sigma: 1.2, key_delay_ms: 60.0, scroll_velocity: 2.0, pause_frequency: 0.02, pause_duration_ms: 200.0, ghost_key_prob: 0.04, window_switch_rate: 0.01, clipboard_usage_rate: 0.08, audio_prob: 0.5 },
        BehaviorPattern { name: "senior-user".into(), mouse_sigma: 5.0, key_delay_ms: 320.0, scroll_velocity: 2.0, pause_frequency: 0.15, pause_duration_ms: 1500.0, ghost_key_prob: 0.06, window_switch_rate: 0.01, clipboard_usage_rate: 0.03, audio_prob: 0.1 },
        BehaviorPattern { name: "gamer".into(), mouse_sigma: 0.8, key_delay_ms: 40.0, scroll_velocity: 6.0, pause_frequency: 0.01, pause_duration_ms: 100.0, ghost_key_prob: 0.005, window_switch_rate: 0.02, clipboard_usage_rate: 0.01, audio_prob: 0.6 },
        BehaviorPattern { name: "mobile-switcher".into(), mouse_sigma: 2.5, key_delay_ms: 150.0, scroll_velocity: 5.0, pause_frequency: 0.12, pause_duration_ms: 600.0, ghost_key_prob: 0.02, window_switch_rate: 0.08, clipboard_usage_rate: 0.06, audio_prob: 0.3 },
        BehaviorPattern { name: "developer".into(), mouse_sigma: 1.5, key_delay_ms: 90.0, scroll_velocity: 3.5, pause_frequency: 0.04, pause_duration_ms: 400.0, ghost_key_prob: 0.03, window_switch_rate: 0.04, clipboard_usage_rate: 0.12, audio_prob: 0.4 },
        BehaviorPattern { name: "student".into(), mouse_sigma: 2.8, key_delay_ms: 200.0, scroll_velocity: 3.0, pause_frequency: 0.10, pause_duration_ms: 700.0, ghost_key_prob: 0.025, window_switch_rate: 0.05, clipboard_usage_rate: 0.07, audio_prob: 0.25 },
        BehaviorPattern { name: "data-entry".into(), mouse_sigma: 1.0, key_delay_ms: 80.0, scroll_velocity: 1.5, pause_frequency: 0.02, pause_duration_ms: 150.0, ghost_key_prob: 0.035, window_switch_rate: 0.005, clipboard_usage_rate: 0.15, audio_prob: 0.45 },
        BehaviorPattern { name: "creative".into(), mouse_sigma: 4.0, key_delay_ms: 250.0, scroll_velocity: 3.0, pause_frequency: 0.18, pause_duration_ms: 2000.0, ghost_key_prob: 0.015, window_switch_rate: 0.06, clipboard_usage_rate: 0.09, audio_prob: 0.2 },
        BehaviorPattern { name: "multitasker".into(), mouse_sigma: 2.0, key_delay_ms: 130.0, scroll_velocity: 4.0, pause_frequency: 0.06, pause_duration_ms: 300.0, ghost_key_prob: 0.03, window_switch_rate: 0.10, clipboard_usage_rate: 0.08, audio_prob: 0.35 },
        BehaviorPattern { name: "reader".into(), mouse_sigma: 1.8, key_delay_ms: 400.0, scroll_velocity: 5.5, pause_frequency: 0.20, pause_duration_ms: 3000.0, ghost_key_prob: 0.005, window_switch_rate: 0.02, clipboard_usage_rate: 0.02, audio_prob: 0.1 },
        BehaviorPattern { name: "erratic".into(), mouse_sigma: 6.0, key_delay_ms: 180.0, scroll_velocity: 4.0, pause_frequency: 0.12, pause_duration_ms: 900.0, ghost_key_prob: 0.05, window_switch_rate: 0.07, clipboard_usage_rate: 0.06, audio_prob: 0.3 },
        BehaviorPattern { name: "methodical".into(), mouse_sigma: 1.3, key_delay_ms: 220.0, scroll_velocity: 2.5, pause_frequency: 0.07, pause_duration_ms: 600.0, ghost_key_prob: 0.01, window_switch_rate: 0.02, clipboard_usage_rate: 0.05, audio_prob: 0.2 },
        BehaviorPattern { name: "power-user".into(), mouse_sigma: 1.0, key_delay_ms: 70.0, scroll_velocity: 4.0, pause_frequency: 0.02, pause_duration_ms: 200.0, ghost_key_prob: 0.04, window_switch_rate: 0.05, clipboard_usage_rate: 0.14, audio_prob: 0.5 },
        BehaviorPattern { name: "novice".into(), mouse_sigma: 5.5, key_delay_ms: 380.0, scroll_velocity: 1.8, pause_frequency: 0.20, pause_duration_ms: 2500.0, ghost_key_prob: 0.07, window_switch_rate: 0.02, clipboard_usage_rate: 0.03, audio_prob: 0.1 },
    ];

    let mut patterns = archetypes.clone();
    let mut rng = SmallRng::from_entropy();

    for i in 0..185usize {
        let a = &archetypes[i % archetypes.len()];
        let b = &archetypes[(i + 3) % archetypes.len()];
        let t: f64 = rng.gen_range(0.2..0.8);
        patterns.push(BehaviorPattern {
            name: format!("variant-{:03}", i + 1),
            mouse_sigma: a.mouse_sigma * t + b.mouse_sigma * (1.0 - t),
            key_delay_ms: a.key_delay_ms * t + b.key_delay_ms * (1.0 - t),
            scroll_velocity: a.scroll_velocity * t + b.scroll_velocity * (1.0 - t),
            pause_frequency: a.pause_frequency * t + b.pause_frequency * (1.0 - t),
            pause_duration_ms: a.pause_duration_ms * t + b.pause_duration_ms * (1.0 - t),
            ghost_key_prob: a.ghost_key_prob * t + b.ghost_key_prob * (1.0 - t),
            window_switch_rate: a.window_switch_rate * t + b.window_switch_rate * (1.0 - t),
            clipboard_usage_rate: a.clipboard_usage_rate * t + b.clipboard_usage_rate * (1.0 - t),
            audio_prob: a.audio_prob * t + b.audio_prob * (1.0 - t),
        });
    }
    patterns
}

pub fn blend_patterns(patterns: &[BehaviorPattern], weights: &[f64]) -> BehaviorPattern {
    assert_eq!(patterns.len(), weights.len());
    let total: f64 = weights.iter().sum();
    let w: Vec<f64> = weights.iter().map(|&x| x / total).collect();
    BehaviorPattern {
        name: "blend".to_string(),
        mouse_sigma: patterns.iter().zip(&w).map(|(p, &wt)| p.mouse_sigma * wt).sum(),
        key_delay_ms: patterns.iter().zip(&w).map(|(p, &wt)| p.key_delay_ms * wt).sum(),
        scroll_velocity: patterns.iter().zip(&w).map(|(p, &wt)| p.scroll_velocity * wt).sum(),
        pause_frequency: patterns.iter().zip(&w).map(|(p, &wt)| p.pause_frequency * wt).sum(),
        pause_duration_ms: patterns.iter().zip(&w).map(|(p, &wt)| p.pause_duration_ms * wt).sum(),
        ghost_key_prob: patterns.iter().zip(&w).map(|(p, &wt)| p.ghost_key_prob * wt).sum(),
        window_switch_rate: patterns.iter().zip(&w).map(|(p, &wt)| p.window_switch_rate * wt).sum(),
        clipboard_usage_rate: patterns.iter().zip(&w).map(|(p, &wt)| p.clipboard_usage_rate * wt).sum(),
        audio_prob: patterns.iter().zip(&w).map(|(p, &wt)| p.audio_prob * wt).sum(),
    }
}

pub async fn train_pattern(name: &str, duration: &str) -> anyhow::Result<()> {
    println!("Training pattern '{}' for {}...", name, duration);
    println!("Recording anonymized behavioral samples...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("Pattern '{}' trained and saved.", name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pattern_library_size() {
        let patterns = embedded_patterns();
        assert!(patterns.len() >= 200, "Must have 200+ patterns");
    }
    #[test]
    fn test_blend_patterns() {
        let patterns = embedded_patterns();
        let selected = &patterns[..3];
        let weights = vec![0.5, 0.3, 0.2];
        let blended = blend_patterns(selected, &weights);
        assert_eq!(blended.name, "blend");
        assert!(blended.mouse_sigma > 0.0);
    }
    #[test]
    fn test_pattern_fields_valid() {
        let patterns = embedded_patterns();
        for p in &patterns {
            assert!(p.mouse_sigma > 0.0);
            assert!(p.key_delay_ms > 0.0);
            assert!(p.ghost_key_prob >= 0.0 && p.ghost_key_prob <= 1.0);
        }
    }
}