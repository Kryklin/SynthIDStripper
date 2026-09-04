use lexicon_stripper::{
    DomainDetector, LexiconStripper, StripperConfig, TerminologyConfig, TerminologyDb, Tokenizer,
};

#[test]
fn test_embedded_terminology_database_integrity() {
    let db = TerminologyDb::new_embedded();
    assert!(!db.terms.is_empty(), "Embedded database should not be empty");
    assert_eq!(db.version, "IATE-EuroVoc-2026.1");

    let hash = db.get_hash();
    assert_eq!(hash.len(), 64, "SHA-256 hash must be 64 hexadecimal chars");
}

#[test]
fn test_longest_match_multiword_scanning() {
    let db = TerminologyDb::new_embedded();
    let tokenizer = Tokenizer::new();
    let text = "The company reported a high market capitalization and improved price-to-earnings ratio.";
    let tokens = tokenizer.tokenize(text);

    let spans = db.scan_spans(&tokens, Some("finance"));
    assert!(!spans.is_empty(), "Should detect finance multiword terms");

    let market_cap_span = spans.iter().find(|s| s.canonical_term == "market capitalization");
    assert!(market_cap_span.is_some(), "Must match 'market capitalization'");
    let span = market_cap_span.unwrap();
    assert_eq!(span.domain, "finance");
    assert!(span.acceptable_variants.contains(&"market cap".to_string()));
}

#[test]
fn test_domain_term_locking_and_protection() {
    let stripper = LexiconStripper::new();
    let text = "The financial analyst reviewed the market capitalization and the EBITDA of the firm.";

    let config = StripperConfig {
        enable_lexical: true,
        terminology_config: Some(TerminologyConfig {
            enabled: true,
            domain_filter: Some("finance".into()),
            protect_domain_terms: true,
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = stripper.transform(text, &config);
    let term_metrics = result.report.terminology_metrics.expect("Terminology metrics must exist");

    assert!(term_metrics.matched_terms_count >= 2, "Must match market capitalization and EBITDA");
    assert!(term_metrics.domain_locked_terms_count >= 1, "Must identify domain-locked terms");

    // Ensure EBITDA is NOT transformed into generic nonsense
    assert!(result.transformed_text.contains("EBITDA") || result.transformed_text.contains("earnings before interest"));
}

#[test]
fn test_prevention_of_unauthorized_generic_substitutions() {
    let stripper = LexiconStripper::new();
    let text = "The market capitalization of the technology enterprise expanded.";

    let config = StripperConfig {
        enable_lexical: true,
        terminology_config: Some(TerminologyConfig {
            enabled: true,
            domain_filter: Some("finance".into()),
            protect_domain_terms: false,
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = stripper.transform(text, &config);

    // "market capitalization" should ONLY be "market capitalization" or "market cap", never "company value" or "worth"
    assert!(
        result.transformed_text.contains("market capitalization")
            || result.transformed_text.contains("market cap"),
        "Phrase must not become generic English: {}",
        result.transformed_text
    );
}

#[test]
fn test_multi_domain_continuous_scoring() {
    let db = TerminologyDb::new_embedded();
    let detector = DomainDetector::new();
    let tokenizer = Tokenizer::new();

    let finance_text = "The quarterly balance sheet recorded severe liquidity strain, high debt-to-equity ratio, and declining EBITDA.";
    let tokens = tokenizer.tokenize(finance_text);
    let scores = detector.score_document(&tokens, &db);

    let finance_score = scores.get("finance").copied().unwrap_or(0.0);
    let cs_score = scores.get("computer_science").copied().unwrap_or(0.0);

    assert!(finance_score > 0.3, "Finance score should be substantial: {:.3}", finance_score);
    assert!(finance_score > cs_score, "Finance score ({:.3}) should exceed CS score ({:.3})", finance_score, cs_score);
    assert_eq!(detector.primary_domain(&scores), "finance");
}

#[test]
fn test_turnover_stratification_dialect() {
    let stripper = LexiconStripper::new();
    let text = "The corporation observed a strong gross domestic product growth alongside notable equity volatility.";

    let config = StripperConfig {
        enable_lexical: true,
        terminology_config: Some(TerminologyConfig {
            enabled: true,
            domain_filter: Some("economics".into()),
            protect_domain_terms: true,
            ..Default::default()
        }),
        ..Default::default()
    };

    let result = stripper.transform(text, &config);
    let metrics = result.report.terminology_metrics.unwrap();

    // Domain lexical turnover should be constrained relative to general turnover
    assert!(metrics.domain_lexical_turnover_pct <= metrics.general_lexical_turnover_pct || metrics.domain_lexical_turnover_pct == 0.0);
}
