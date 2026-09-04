use lexicon_stripper::{
    CoarsePos, Domain, DomainClassifier, LexiconStripper, StripperConfig,
};

#[test]
fn test_domain_auto_detection() {
    let classifier = DomainClassifier::new();

    // Financial text lemmas
    let finance_lemmas = vec![
        "the".into(),
        "firm".into(),
        "manage".into(),
        "portfolio".into(),
        "liquidity".into(),
        "ebitda".into(),
        "yield".into(),
    ];
    assert_eq!(classifier.detect_domain(&finance_lemmas), Domain::Finance);

    // Biomedical text lemmas
    let bio_lemmas = vec![
        "the".into(),
        "cytokine".into(),
        "trigger".into(),
        "apoptosis".into(),
        "in".into(),
        "pathology".into(),
        "tissue".into(),
    ];
    assert_eq!(classifier.detect_domain(&bio_lemmas), Domain::Biomedical);

    // Computer science lemmas
    let cs_lemmas = vec![
        "the".into(),
        "compiler".into(),
        "optimize".into(),
        "bytecode".into(),
        "to".into(),
        "reduce".into(),
        "latency".into(),
        "throughput".into(),
    ];
    assert_eq!(classifier.detect_domain(&cs_lemmas), Domain::ComputerScience);

    // General text lemmas
    let general_lemmas = vec![
        "the".into(),
        "quick".into(),
        "brown".into(),
        "fox".into(),
        "jump".into(),
        "over".into(),
        "lazy".into(),
        "dog".into(),
    ];
    assert_eq!(classifier.detect_domain(&general_lemmas), Domain::General);
}

#[test]
fn test_domain_locked_terms_protection() {
    let stripper = LexiconStripper::new();

    let text = "The firm analyzed its EBITDA, portfolio liquidity, and securities dividend yield.";
    let config = StripperConfig {
        domain: Domain::Finance,
        replacement_probability: 1.0,
        enable_lexical: true,
        ..Default::default()
    };

    let res = stripper.transform(text, &config);

    // Locked terms must remain untouched in output text
    assert!(res.transformed_text.contains("EBITDA"));
    assert!(res.transformed_text.contains("liquidity"));
    assert!(res.transformed_text.contains("yield"));
    assert!(res.report.domain_locked_tokens_count >= 3);
    assert_eq!(res.report.detected_domain, "finance");
}

#[test]
fn test_domain_preferred_substitution() {
    let classifier = DomainClassifier::new();

    let cands = vec!["establishment".into(), "house".into(), "enterprise".into()];
    let filtered = classifier.filter_candidates("firm", CoarsePos::Noun, Domain::Finance, cands);

    // In finance, "enterprise", "company", "corporation" are domain-preferred equivalents
    assert!(filtered.contains(&"enterprise".to_string()));
    assert!(filtered.contains(&"company".to_string()) || filtered.contains(&"corporation".to_string()) || filtered.contains(&"business".to_string()));
}

#[test]
fn test_effective_replacement_entropy_metric() {
    let classifier = DomainClassifier::new();

    let content_words = vec!["firm".into(), "growth".into(), "strategy".into()];
    let candidate_counts = vec![3, 4, 2];

    let h_eff = classifier.compute_effective_entropy(&content_words, Domain::Finance, &candidate_counts);

    // H_eff = (log2(1+3) + log2(1+4) + log2(1+2)) / 3 = (2.0 + 2.3219 + 1.58496) / 3 = 1.96896
    assert!((h_eff - 1.969).abs() < 0.01);
}
