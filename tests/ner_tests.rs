use lexicon_stripper::{LexiconStripper, StripperConfig};

#[test]
fn test_proper_nouns_and_acronyms_protected() {
    let stripper = LexiconStripper::new();
    let config = StripperConfig::default();

    let input = "John Smith visited Microsoft and NASA in New York.";
    let result = stripper.transform(input, &config);

    // Named entities must remain intact
    assert!(result.transformed_text.contains("John"));
    assert!(result.transformed_text.contains("Smith"));
    assert!(result.transformed_text.contains("Microsoft"));
    assert!(result.transformed_text.contains("NASA"));
    assert!(result.transformed_text.contains("New"));
    assert!(result.transformed_text.contains("York"));
}

#[test]
fn test_urls_and_technical_terms_protected() {
    let stripper = LexiconStripper::new();
    let config = StripperConfig::default();

    let input = "Check https://example.com/api for JSON endpoints in Rust.";
    let result = stripper.transform(input, &config);

    assert!(result.transformed_text.contains("https://example.com/api"));
    assert!(result.transformed_text.contains("JSON"));
    assert!(result.transformed_text.contains("Rust"));
}
