use lexicon_stripper::detector::synthid_official::{
    accumulate_hash, DeepMindSynthIdConfig, DeepMindSynthIdDetector,
};

#[test]
fn test_deepmind_official_lcg_hash_properties() {
    // 1. Check LCG single-step accumulation
    let mut h = 0u64;
    h = accumulate_hash(h, &[42]);
    // 0 + 42 = 42
    // 42 * 6364136223846793005 + 1 = 267293721401565306211 + 1 mod 2^64
    let expected = (42u64).wrapping_mul(6364136223846793005).wrapping_add(1);
    assert_eq!(h, expected);

    // 2. Associativity / streaming property: f(x, data[T]) = f(f(x, data[:T-1]), data[T])
    let data = [100u64, 200, 300, 400];
    let all_at_once = accumulate_hash(12345, &data);

    let step1 = accumulate_hash(12345, &data[..2]);
    let step2 = accumulate_hash(step1, &data[2..]);
    assert_eq!(all_at_once, step2);
}

#[test]
fn test_deepmind_official_detector_null_hypothesis() {
    let detector = DeepMindSynthIdDetector::new(DeepMindSynthIdConfig::default());

    // Generate random synthetic sequence
    let tokens: Vec<u64> = (0..500).map(|i| (i * 7919) % 32000).collect();
    let result = detector.detect_tokens(&tokens);

    assert_eq!(result.scored_ngrams, 500 - 3 + 1);
    assert_eq!(result.depth, 8);
    // Under unwatermarked text, mean_score ~ 0.500 (+/- 0.05)
    assert!(
        (result.mean_score - 0.500).abs() < 0.05,
        "Mean score should be ~0.500 for unwatermarked tokens, got {:.4}",
        result.mean_score
    );
    assert!(!result.is_watermarked);
    assert!(result.z_score.abs() < 3.0);
}
