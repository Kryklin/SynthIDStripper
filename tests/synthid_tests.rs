use lexicon_stripper::{
    DistributionMode, LexiconStripper, StripperConfig, SynthIdConfig, SynthIdDetector,
    SynthIdWatermarker,
};

#[test]
fn test_synthid_watermarking_and_stripping_pipeline() {
    let config = SynthIdConfig {
        key: 987654321,
        context_length: 2,
        detection_threshold: 3.0,
    };

    let detector = SynthIdDetector::new(config.clone());
    let watermarker = SynthIdWatermarker::new(config);

    let original_text =
        "The old station stood quietly at the edge of the small coastal town with large buildings.";
    let words: Vec<&str> = original_text.split_whitespace().collect();

    // 1. Watermark the text by selecting synonyms with high g-values
    let watermarked_text =
        watermarker.watermark_text(&words, |w| match w.to_lowercase().as_str() {
            "small" => vec![
                "little".into(),
                "tiny".into(),
                "modest".into(),
                "compact".into(),
            ],
            "large" => vec![
                "big".into(),
                "massive".into(),
                "spacious".into(),
                "broad".into(),
            ],
            "buildings" => vec!["structures".into(), "edifices".into()],
            "stood" => vec!["remained".into(), "rested".into()],
            "quietly" => vec!["peacefully".into(), "calmly".into(), "silently".into()],
            _ => vec![],
        });

    // 2. Detect watermark in the watermarked text
    let wm_scan = detector.detect(&watermarked_text);
    assert!(
        wm_scan.z_score >= 1.645,
        "Expected elevated Z-score in watermarked text, got Z={:.3}",
        wm_scan.z_score
    );

    // 3. Strip the watermark using LexiconStripper
    let stripper = LexiconStripper::new();
    let stripper_cfg = StripperConfig {
        mode: DistributionMode::Human,
        enable_lexical: true,
        enable_syntax: true,
        enable_cadence: true,
        enable_function_words: true,
        seed: Some(101),
        ..Default::default()
    };

    let stripped_res = stripper.transform(&watermarked_text, &stripper_cfg);

    // 4. Detect watermark in the stripped text
    let stripped_scan = detector.detect(&stripped_res.transformed_text);

    // The Z-score must drop significantly below detection threshold
    assert!(
        stripped_scan.z_score < wm_scan.z_score,
        "Stripper must reduce SynthID Z-score: before={:.3}, after={:.3}",
        wm_scan.z_score,
        stripped_scan.z_score
    );
    assert!(
        !stripped_scan.is_watermarked,
        "Stripped text must not trigger positive watermark detection"
    );
}

#[test]
fn test_unwatermarked_null_distribution() {
    let detector = SynthIdDetector::new(SynthIdConfig::default());
    let plain_text =
        "This is a regular natural sentence written by a human without any watermarking signal.";
    let scan = detector.detect(plain_text);

    assert!(!scan.is_watermarked);
    assert!(scan.z_score < 3.0);
    assert!((scan.mean_g_value - 0.5).abs() < 0.3);
}
