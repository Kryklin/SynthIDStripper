use lexicon_stripper::lexicon::SynsetDb;
use lexicon_stripper::{
    CollocationDb, DistributionMode, LexiconStripper, PosTag, SelectionStrategy, StripperConfig,
    ValencyValidator,
};

#[test]
fn test_valency_validator_rejects_incompatible_from_preposition() {
    let validator = ValencyValidator::new();
    let words_after = vec!["the", "internet", "from", "completely", "drowning"];

    // "keep" is valid with "from"
    assert!(validator.is_valency_compatible("keep", "keep", PosTag::VB, &words_after));
    assert!(validator.is_valency_compatible("prevent", "keep", PosTag::VB, &words_after));
    assert!(validator.is_valency_compatible("stop", "keep", PosTag::VB, &words_after));

    // "maintain", "preserve", "sustain" MUST be rejected when followed by "from"
    assert!(!validator.is_valency_compatible("maintain", "keep", PosTag::VB, &words_after));
    assert!(!validator.is_valency_compatible("preserve", "keep", PosTag::VB, &words_after));
    assert!(!validator.is_valency_compatible("sustain", "keep", PosTag::VB, &words_after));
}

#[test]
fn test_valency_validator_rejects_achieve_point_collocation() {
    let validator = ValencyValidator::new();
    let words_after = vec!["a", "point", "where", "we", "have"];

    // "reach", "arrive", "get" are valid for reaching a point/state
    assert!(validator.is_valency_compatible("reach", "reach", PosTag::VBD, &words_after));
    assert!(validator.is_valency_compatible("arrive", "reach", PosTag::VBD, &words_after));
    assert!(validator.is_valency_compatible("get", "reach", PosTag::VBD, &words_after));

    // "achieve", "accomplish", "execute" MUST be rejected for reaching a point
    assert!(!validator.is_valency_compatible("achieve", "reach", PosTag::VBD, &words_after));
    assert!(!validator.is_valency_compatible("accomplish", "reach", PosTag::VBD, &words_after));
    assert!(!validator.is_valency_compatible("execute", "reach", PosTag::VBD, &words_after));
}

#[test]
fn test_collocation_db_identifies_anti_collocations() {
    let db = CollocationDb::new();

    // Anti-collocations score 0.0
    let score1 = db.evaluate_collocation_affinity("maintain", "maintain", &[], &["from", "drowning"]);
    assert_eq!(score1, 0.0);

    let score2 = db.evaluate_collocation_affinity("outcome", "outcome", &["raw"], &["into"]);
    assert_eq!(score2, 0.0);

    let score3 = db.evaluate_collocation_affinity("electing", "elect", &["instead", "of"], &["the", "word"]);
    assert_eq!(score3, 0.0);

    // Natural affinity collocations score > 1.0
    let score_reach = db.evaluate_collocation_affinity("reached", "reach", &["we", "have"], &["a", "point"]);
    assert!(score_reach > 1.0);

    let score_pick = db.evaluate_collocation_affinity("picking", "pick", &["instead", "of"], &["the", "word"]);
    assert!(score_pick > 1.0);

    let score_raw_output = db.evaluate_collocation_affinity("output", "output", &["raw"], &["into"]);
    assert!(score_raw_output > 1.0);
}

#[test]
fn test_curated_synset_purity() {
    let synset_db = SynsetDb::new();

    // "pick" should not include "elect"
    let pick_synsets = synset_db.find_synsets("pick", lexicon_stripper::types::CoarsePos::Verb);
    for syn in pick_synsets {
        assert!(!syn.lemmas.contains(&"elect"));
        assert!(!syn.lemmas.contains(&"single"));
    }

    // "people" should not include "persons" or "citizens"
    let people_synsets = synset_db.find_synsets("people", lexicon_stripper::types::CoarsePos::Noun);
    for syn in people_synsets {
        assert!(!syn.lemmas.contains(&"persons"));
        assert!(!syn.lemmas.contains(&"citizens"));
    }
}

#[test]
fn test_end_to_end_fluency_preservation() {
    let stripper = LexiconStripper::new();
    let config = StripperConfig {
        mode: DistributionMode::Human,
        selection_strategy: SelectionStrategy::ConservativeEfficiency,
        replacement_probability: 1.0, // test under maximum substitution pressure
        seed: Some(42),
        ..Default::default()
    };

    let input = "We must keep the system from failing. Instead of picking the wrong word, people should check the raw output once they have reached a point of confidence.";
    let result = stripper.transform(input, &config);

    // Verify ungrammatical / robotic phrases are strictly absent
    assert!(!result.transformed_text.contains("maintain the system from"));
    assert!(!result.transformed_text.contains("preserve the system from"));
    assert!(!result.transformed_text.contains("electing the"));
    assert!(!result.transformed_text.contains("persons"));
    assert!(!result.transformed_text.contains("raw outcome"));
    assert!(!result.transformed_text.contains("achieved a point"));
}
