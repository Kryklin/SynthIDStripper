use lexicon_stripper::{DistributionMode, GptZeroPredictResponse, LexiconStripper};

#[test]
fn test_gptzero_response_deserialization() {
    let raw_json = r#"{
        "version": "2024-01-01",
        "scan_id": "test-scan-123",
        "documents": [
            {
                "completely_generated_prob": 0.99,
                "overall_burstiness": 18.5,
                "document_classification": "AI",
                "confidence_category": "high",
                "class_probabilities": {
                    "ai": 0.99,
                    "human": 0.01,
                    "mixed": 0.0
                },
                "sentences": [
                    {
                        "sentence": "The old station stood near the coast.",
                        "perplexity": 15.2,
                        "generated_prob": 0.95,
                        "highlight_sentence_for_ai": true
                    }
                ]
            }
        ]
    }"#;

    let parsed: GptZeroPredictResponse =
        serde_json::from_str(raw_json).expect("Parse valid GPTZero JSON");
    assert_eq!(parsed.documents.len(), 1);
    let doc = &parsed.documents[0];
    assert_eq!(doc.completely_generated_prob, Some(0.99));
    assert_eq!(doc.document_classification, Some("AI".to_string()));
    assert_eq!(doc.sentences.len(), 1);
    assert_eq!(doc.sentences[0].highlight_sentence_for_ai, Some(true));
}

#[test]
fn test_ablation_matrix_generation() {
    let stripper = LexiconStripper::new();
    let text = "Although the station was small, it remained a comforting place. Every morning, commuters arrived.";

    let report = stripper.run_ablation_matrix(text, DistributionMode::Human, Some(42), None, None);
    assert_eq!(report.runs.len(), 9);

    // Run A: Baseline
    assert_eq!(report.runs[0].label, "A");
    assert_eq!(report.runs[0].transform_result.report.replaced_words, 0);

    // Run B: WordNet Lexical Only (No Terminology)
    assert_eq!(report.runs[1].label, "B");
    assert!(report.runs[1].lexical_enabled);
    assert!(!report.runs[1].terminology_enabled);

    // Run C: WordNet + IATE/EuroVoc Terminology
    assert_eq!(report.runs[2].label, "C");
    assert!(report.runs[2].lexical_enabled);
    assert!(report.runs[2].terminology_enabled);

    // Run D: Typing Noise Only
    assert_eq!(report.runs[3].label, "D");
    assert!(!report.runs[3].lexical_enabled);
    assert!(report.runs[3].typing_enabled);

    // Run I: Full Pipeline + Terminology
    assert_eq!(report.runs[8].label, "I");
    assert!(report.runs[8].lexical_enabled);
    assert!(report.runs[8].terminology_enabled);
    assert!(report.runs[8].syntax_enabled);
    assert!(report.runs[8].cadence_enabled);
    assert!(report.runs[8].function_words_enabled);
    assert!(report.runs[8].typing_enabled);
}
