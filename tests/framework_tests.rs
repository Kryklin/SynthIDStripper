use lexicon_stripper::{DistributionMode, LexiconStripper, StripperConfig, TransformationClass};

#[test]
fn test_fixed_point_protection() {
    let stripper = LexiconStripper::new();
    let text = "The old building stood near the small station with large glass windows.";

    let config = StripperConfig {
        mode: DistributionMode::Human,
        seed: Some(12345),
        allow_re_replacement: false,
        enable_lexical: true,
        enable_syntax: false,
        enable_cadence: false,
        enable_function_words: false,
        ..Default::default()
    };

    let result = stripper.transform_multipass(text, &config, 10);
    assert!(result.report.stability_report.is_some());
    let stab = result.report.stability_report.unwrap();
    assert!(stab.fixed_point_reached);
    assert!(stab.convergence_pass.is_some());
    // Passes beyond convergence pass should have 0 replacements
    let conv_pass = stab.convergence_pass.unwrap();
    for p in conv_pass..stab.total_passes {
        assert_eq!(stab.pass_replacements[p], 0);
    }
}

#[test]
fn test_deterministic_seeds() {
    let stripper = LexiconStripper::new();
    let text = "The old railway station stood quietly at the edge of the small coastal town.";

    let config1 = StripperConfig {
        mode: DistributionMode::Neutral,
        seed: Some(42),
        ..Default::default()
    };
    let config2 = StripperConfig {
        mode: DistributionMode::Neutral,
        seed: Some(42),
        ..Default::default()
    };

    let res1 = stripper.transform(text, &config1);
    let res2 = stripper.transform(text, &config2);

    assert_eq!(res1.transformed_text, res2.transformed_text);
    assert_eq!(res1.report.input_sha256, res2.report.input_sha256);
    assert_eq!(res1.report.output_sha256, res2.report.output_sha256);
}

#[test]
fn test_lexical_only_mode() {
    let stripper = LexiconStripper::new();
    let text = "Although the station was important, it remained a comforting place. Some complained, but others watched.";

    let config = StripperConfig {
        mode: DistributionMode::Human,
        enable_lexical: true,
        enable_syntax: false,
        enable_cadence: false,
        enable_function_words: false,
        seed: Some(100),
        ..Default::default()
    };

    let res = stripper.transform(text, &config);
    // Syntax should NOT invert in lexical-only mode
    assert!(res.transformed_text.starts_with("Although"));
    // All recorded transformations must be of class Lexical
    for rep in &res.report.replacements {
        assert_eq!(rep.transformation_class, TransformationClass::Lexical);
    }
}

#[test]
fn test_syntax_only_mode() {
    let stripper = LexiconStripper::new();
    let text = "Although the station was old, it remained standing.";

    let config = StripperConfig {
        mode: DistributionMode::Human,
        enable_lexical: false,
        enable_syntax: true,
        enable_cadence: false,
        enable_function_words: false,
        ..Default::default()
    };

    let res = stripper.transform(text, &config);
    // Clause order must invert: "It remained standing, although the station was old."
    assert!(res
        .transformed_text
        .contains("although the station was old"));
    assert!(res.transformed_text.starts_with("It remained standing"));
    assert!(res
        .report
        .replacements
        .iter()
        .any(|r| r.transformation_class == TransformationClass::Syntax));
}

#[test]
fn test_combined_mode() {
    let stripper = LexiconStripper::new();
    let text = "Although the small building was important, it remained open. In recent years, some complained, but others watched.";

    let config = StripperConfig {
        mode: DistributionMode::Human,
        enable_lexical: true,
        enable_syntax: true,
        enable_cadence: true,
        enable_function_words: true,
        seed: Some(777),
        ..Default::default()
    };

    let res = stripper.transform(text, &config);
    assert!(!res.report.replacements.is_empty());
    // Both lexical and structural/syntax transforms should be recorded
    let classes: std::collections::HashSet<TransformationClass> = res
        .report
        .replacements
        .iter()
        .map(|r| r.transformation_class)
        .collect();
    assert!(classes.contains(&TransformationClass::Lexical));
    assert!(
        classes.contains(&TransformationClass::Syntax)
            || classes.contains(&TransformationClass::FunctionWord)
    );
}

#[test]
fn test_jsd_calculation() {
    let stripper = LexiconStripper::new();
    let text = "The old railway station stood quietly at the edge of the small coastal town with large buildings.";

    let config = StripperConfig {
        mode: DistributionMode::Human,
        enable_lexical: true,
        seed: Some(888),
        ..Default::default()
    };

    let res = stripper.transform(text, &config);
    let jsd = res.report.jsd_metrics;
    assert!(jsd.total_jsd >= 0.0 && jsd.total_jsd <= 1.0);
    assert!(jsd.content_words_jsd >= 0.0 && jsd.content_words_jsd <= 1.0);
    assert!(jsd.nouns_jsd >= 0.0 && jsd.nouns_jsd <= 1.0);
    assert!(jsd.adjectives_jsd >= 0.0 && jsd.adjectives_jsd <= 1.0);
}

#[test]
fn test_transformation_state_tracking() {
    let stripper = LexiconStripper::new();
    let text = "The small station has large windows.";

    let config = StripperConfig {
        mode: DistributionMode::Human,
        enable_lexical: true,
        seed: Some(555),
        ..Default::default()
    };

    let res = stripper.transform(text, &config);
    for rep in &res.report.replacements {
        assert!(!rep.original.is_empty());
        assert!(!rep.replacement.is_empty());
        assert!(rep.confidence > 0.0);
        assert_eq!(rep.pass_number, 1);
    }
}

#[test]
fn test_protected_entities() {
    let stripper = LexiconStripper::new();
    let text = "NASA deployed Rust code to https://example.com/api for JSON parsing.";

    let config = StripperConfig {
        mode: DistributionMode::Human,
        preserve_named_entities: true,
        preserve_technical_terms: true,
        ..Default::default()
    };

    let res = stripper.transform(text, &config);
    assert!(res.transformed_text.contains("NASA"));
    assert!(res.transformed_text.contains("Rust"));
    assert!(res.transformed_text.contains("JSON"));
    assert!(res.transformed_text.contains("https://example.com/api"));
}

#[test]
fn test_low_confidence_rejection() {
    let stripper = LexiconStripper::new();
    let text = "The small station had large windows.";

    // Extremely high threshold to force rejection of uncertain substitutions
    let config = StripperConfig {
        min_confidence: 0.999,
        replacement_probability: 1.0,
        ..Default::default()
    };

    let res = stripper.transform(text, &config);
    assert!(res.report.rejected_candidates.iter().any(|r| {
        matches!(
            r.reason,
            lexicon_stripper::RejectionReason::LowContextConfidence { .. }
        )
    }));
}
