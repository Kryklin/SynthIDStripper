use lexicon_stripper::analysis::{
    SensitivityAnalyzer, SensitivityObservationStatus, SensitivitySamplingPolicy,
};
use lexicon_stripper::{
    Domain, LexiconStripper, SensitivityConfig, StripperConfig, SynthIdConfig,
};

const SAMPLE_TEXT: &str = "\
Deep learning models and artificial neural networks are capable of discovering complex patterns in large datasets. \
These algorithms process information through multiple layers of nonlinear computational nodes.\n\n\
In biomedical research, neural networks analyze medical imaging and genomic sequences with high diagnostic precision. \
Market capitalization in financial markets also reflects technological adoption across global industries.";

#[test]
fn test_token_to_position_mapping() {
    let analyzer = SensitivityAnalyzer::new(SynthIdConfig::default());
    let (metas, base_z) = analyzer.extract_token_positions("doc_001", SAMPLE_TEXT, Some(Domain::General));

    assert!(!metas.is_empty(), "Token position metas should not be empty");
    assert!((-10.0..=10.0).contains(&base_z), "Base Z score should be reasonable");

    let first = &metas[0];
    assert_eq!(first.token_index, 0);
    assert_eq!(first.sentence_index, 0);
    assert_eq!(first.paragraph_index, 0);
    assert_eq!(first.character_offset, 0);
    assert_eq!(first.normalized_position, 0.0);

    let last = metas.last().unwrap();
    assert!(last.token_index > 0);
    assert!(last.sentence_index >= 2);
    assert!(last.paragraph_index >= 1);
    assert!(last.normalized_position > 0.9 && last.normalized_position <= 1.0);

    for m in &metas {
        assert!(m.normalized_position >= 0.0 && m.normalized_position <= 1.0);
    }
}

#[test]
fn test_deterministic_sensitivity_profiling() {
    let analyzer = SensitivityAnalyzer::new(SynthIdConfig::default());
    let records1 = analyzer.profile_document("doc_001", SAMPLE_TEXT, Some(Domain::General), 3, 2, 42);
    let records2 = analyzer.profile_document("doc_001", SAMPLE_TEXT, Some(Domain::General), 3, 2, 42);

    assert_eq!(records1.len(), records2.len());
    for (r1, r2) in records1.iter().zip(records2.iter()) {
        assert_eq!(r1.token_index, r2.token_index);
        assert_eq!(r1.token_text, r2.token_text);
        assert!((r1.mean_delta_z - r2.mean_delta_z).abs() < 1e-6);
        assert_eq!(r1.status, r2.status);
    }
}

#[test]
fn test_insufficient_observation_handling() {
    let analyzer = SensitivityAnalyzer::new(SynthIdConfig::default());
    // With samples_per_token = 1 and min_observations = 5, observations must be tagged UnderSampled
    let records = analyzer.profile_document("doc_001", SAMPLE_TEXT, Some(Domain::General), 1, 5, 42);

    assert!(!records.is_empty());
    for r in &records {
        assert_eq!(r.status, SensitivityObservationStatus::UnderSampled);
    }
}

#[test]
fn test_spatial_binning_and_heatmap_serialization() {
    let analyzer = SensitivityAnalyzer::new(SynthIdConfig::default());
    let records = analyzer.profile_document("doc_001", SAMPLE_TEXT, Some(Domain::General), 3, 1, 42);
    assert!(!records.is_empty());

    let model = analyzer.build_heatmap_model(
        records,
        vec!["doc_001".to_string()],
        "sample_corpus_hash_123",
        42,
        1,
    );

    assert_eq!(model.position_bins_10.len(), 10);
    assert_eq!(model.position_bins_20.len(), 20);
    assert_eq!(model.position_bins_50.len(), 50);
    assert!(!model.pos_stats.is_empty());
    assert!(!model.heatmap_hash.is_empty());

    let temp_path = "temp_test_heatmap.json";
    analyzer.save_heatmap_json(&model, temp_path).expect("Save heatmap JSON");
    let loaded = SensitivityAnalyzer::load_heatmap_json(temp_path).expect("Load heatmap JSON");
    let _ = std::fs::remove_file(temp_path);

    assert_eq!(model.heatmap_hash, loaded.heatmap_hash);
    assert_eq!(model.total_tokens_profiled, loaded.total_tokens_profiled);
}

#[test]
fn test_equal_budget_sampling_invariant() {
    let stripper = LexiconStripper::new();
    let budget = 4usize;

    let uniform_cfg = StripperConfig {
        seed: Some(101),
        min_confidence: 0.40,
        sensitivity_config: Some(SensitivityConfig {
            policy: SensitivitySamplingPolicy::Uniform,
            heatmap_path: None,
            composite_model_path: None,
            edit_budget: Some(budget),
            min_observations: 1,
        }),
        ..Default::default()
    };

    let high_cfg = StripperConfig {
        seed: Some(101),
        min_confidence: 0.40,
        sensitivity_config: Some(SensitivityConfig {
            policy: SensitivitySamplingPolicy::HighSensitivity,
            heatmap_path: None,
            composite_model_path: None,
            edit_budget: Some(budget),
            min_observations: 1,
        }),
        ..Default::default()
    };

    let low_cfg = StripperConfig {
        seed: Some(101),
        min_confidence: 0.40,
        sensitivity_config: Some(SensitivityConfig {
            policy: SensitivitySamplingPolicy::LowSensitivity,
            heatmap_path: None,
            composite_model_path: None,
            edit_budget: Some(budget),
            min_observations: 1,
        }),
        ..Default::default()
    };

    let res_uniform = stripper.transform(SAMPLE_TEXT, &uniform_cfg);
    let res_high = stripper.transform(SAMPLE_TEXT, &high_cfg);
    let res_low = stripper.transform(SAMPLE_TEXT, &low_cfg);

    // All three conditions must strictly respect the budget constraint
    assert!(res_uniform.report.replaced_words <= budget);
    assert!(res_high.report.replaced_words <= budget);
    assert!(res_low.report.replaced_words <= budget);
    assert!(res_uniform.report.replaced_words > 0);
    assert!(res_high.report.replaced_words > 0);
    assert!(res_low.report.replaced_words > 0);
}
