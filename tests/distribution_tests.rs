use lexicon_stripper::{DistributionMode, LexiconStripper, StripperConfig};

#[test]
fn test_distribution_modes() {
    let stripper = LexiconStripper::new();
    let text = "This is an important objective to improve the system and examine the problem.";

    let modes = [
        DistributionMode::Neutral,
        DistributionMode::Human,
        DistributionMode::Ai,
        DistributionMode::Random,
    ];

    for &mode in &modes {
        let config = StripperConfig {
            mode,
            seed: Some(999),
            ..Default::default()
        };

        let result = stripper.transform(text, &config);
        assert_eq!(result.report.mode_used, mode);
        assert!(result.report.replaced_words > 0);
        assert!(!result.transformed_text.is_empty());
    }
}
