use lexicon_stripper::{DistributionMode, LexiconStripper, StripperConfig};

#[test]
fn test_semantic_lexical_transformation() {
    let stripper = LexiconStripper::new();
    let config = StripperConfig {
        mode: DistributionMode::Neutral,
        seed: Some(42),
        ..Default::default()
    };

    let input = "The primary objective was to modify the system quickly.";
    let result = stripper.transform(input, &config);

    assert_eq!(result.report.total_words, 9);
    assert!(result.report.eligible_words > 0);
    assert!(result.report.replaced_words > 0);
    assert!(result.report.mean_confidence > 0.0);

    // Verify tense and grammar preservation
    assert!(result.transformed_text.starts_with("The"));
    assert!(result.transformed_text.ends_with('.'));
}

#[test]
fn test_transformation_report_rejections() {
    let stripper = LexiconStripper::new();
    let config = StripperConfig::default();

    let input = "The xyz123qwerty was exceptionally unfindable.";
    let result = stripper.transform(input, &config);

    // Stopwords and unknown words should produce rejection records
    assert!(!result.report.rejected_candidates.is_empty());
}

#[test]
fn test_deterministic_seed() {
    let stripper = LexiconStripper::new();
    let config1 = StripperConfig {
        mode: DistributionMode::Neutral,
        seed: Some(12345),
        ..Default::default()
    };
    let config2 = StripperConfig {
        mode: DistributionMode::Neutral,
        seed: Some(12345),
        ..Default::default()
    };

    let input = "We need to create a fast method to solve the problem.";
    let res1 = stripper.transform(input, &config1);
    let res2 = stripper.transform(input, &config2);

    assert_eq!(res1.transformed_text, res2.transformed_text);
    assert_eq!(res1.report.replaced_words, res2.report.replaced_words);
}
