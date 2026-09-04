use lexicon_stripper::typing::{
    apply_duplication, apply_insertion, apply_omission, apply_substitution,
    apply_temporal_transition, apply_transposition, KeyboardModel, TypingErrorClass,
};
use lexicon_stripper::{LexiconStripper, StripperConfig, TypingNoiseConfig};
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::collections::HashMap;

#[test]
fn test_keyboard_geometry_and_distances() {
    let kb = KeyboardModel::default();

    // 1. Check coordinates on QWERTY layout
    let key_q = kb.get_key('q').expect("Key Q exists");
    let key_w = kb.get_key('w').expect("Key W exists");
    let key_a = kb.get_key('a').expect("Key A exists");

    assert_eq!(key_q.row, 1);
    assert_eq!(key_w.row, 1);
    assert_eq!(key_a.row, 2);

    // 2. Horizontal distance along same row
    let dist_qw = kb.distance('q', 'w').expect("Distance Q-W");
    assert!((dist_qw - 1.0).abs() < 1e-5);

    // 3. Staggered row distance (Q at x=0.5, y=1.0; A at x=0.75, y=2.0)
    let dist_qa = kb.distance('q', 'a').expect("Distance Q-A");
    assert!(dist_qa > 1.0);

    // 4. Distant keys have higher distance than adjacent keys
    let dist_qp = kb.distance('q', 'p').expect("Distance Q-P");
    assert!(dist_qp > dist_qw);
    assert!(dist_qp > dist_qa);
}

#[test]
fn test_neighbor_weighting_proximity() {
    let kb = KeyboardModel::default();
    let neighbors_f = kb.neighbors('f', 1.5);

    let neighbor_chars: Vec<char> = neighbors_f.iter().map(|(c, _)| *c).collect();
    // Keys adjacent to 'F' on QWERTY: 'd', 'g', 'r', 't', 'c', 'v'
    assert!(neighbor_chars.contains(&'d'));
    assert!(neighbor_chars.contains(&'g'));
    assert!(neighbor_chars.contains(&'r'));
    assert!(neighbor_chars.contains(&'v'));

    // Distant keys like 'p' or 'z' must not be in immediate radius <= 1.5
    assert!(!neighbor_chars.contains(&'p'));
    assert!(!neighbor_chars.contains(&'z'));
}

#[test]
fn test_substitution_error_class() {
    let kb = KeyboardModel::default();
    let mut rng = StdRng::seed_from_u64(42);

    let word = "location";
    let res = apply_substitution(word, &kb, &mut rng).expect("Substitution applied");

    assert_eq!(res.error_class, TypingErrorClass::Substitution);
    assert_eq!(res.mutated_word.len(), word.len());
    assert_ne!(res.mutated_word, word);
    assert!(res.keyboard_distance > 0.0);
}

#[test]
fn test_transposition_error_class() {
    let kb = KeyboardModel::default();
    let mut rng = StdRng::seed_from_u64(42);

    let word = "location";
    let res = apply_transposition(word, &kb, &mut rng).expect("Transposition applied");

    assert_eq!(res.error_class, TypingErrorClass::Transposition);
    assert_eq!(res.mutated_word.len(), word.len());
    assert_ne!(res.mutated_word, word);
}

#[test]
fn test_omission_error_class() {
    let mut rng = StdRng::seed_from_u64(42);

    let word = "location";
    let res = apply_omission(word, &mut rng).expect("Omission applied");

    assert_eq!(res.error_class, TypingErrorClass::Omission);
    assert_eq!(res.mutated_word.len(), word.len() - 1);
    assert_ne!(res.mutated_word, word);
}

#[test]
fn test_insertion_error_class() {
    let kb = KeyboardModel::default();
    let mut rng = StdRng::seed_from_u64(42);

    let word = "location";
    let res = apply_insertion(word, &kb, &mut rng).expect("Insertion applied");

    assert_eq!(res.error_class, TypingErrorClass::Insertion);
    assert_eq!(res.mutated_word.len(), word.len() + 1);
    assert_ne!(res.mutated_word, word);
}

#[test]
fn test_duplication_error_class() {
    let mut rng = StdRng::seed_from_u64(42);

    let word = "location";
    let res = apply_duplication(word, &mut rng).expect("Duplication applied");

    assert_eq!(res.error_class, TypingErrorClass::Duplication);
    assert_eq!(res.mutated_word.len(), word.len() + 1);
    assert_ne!(res.mutated_word, word);
}

#[test]
fn test_temporal_transition_error_class() {
    let kb = KeyboardModel::default();
    let mut rng = StdRng::seed_from_u64(42);

    let word = "your";
    let res = apply_temporal_transition(word, &kb, &mut rng).expect("Temporal transition applied");

    assert_eq!(res.error_class, TypingErrorClass::Temporal);
    assert_eq!(res.mutated_word.len(), word.len());
    assert_ne!(res.mutated_word, word);
}

#[test]
fn test_deterministic_seeds_reproducibility() {
    let stripper = LexiconStripper::new();
    let text = "Although the station was small, it remained a comforting place for commuters.";

    let config = StripperConfig {
        enable_lexical: false,
        typing_noise_config: Some(TypingNoiseConfig {
            rate: 0.15,
            seed: Some(12345),
            ..Default::default()
        }),
        seed: Some(12345),
        ..Default::default()
    };

    let res1 = stripper.transform(text, &config);
    let res2 = stripper.transform(text, &config);

    assert_eq!(res1.transformed_text, res2.transformed_text);
    assert_eq!(res1.report.output_sha256, res2.report.output_sha256);
    assert_eq!(res1.report.replaced_words, res2.report.replaced_words);
}

#[test]
fn test_text_protection_filters() {
    let stripper = LexiconStripper::new();
    let text = "Contact support at https://example.com/api or user@test.com on port 8080 with NASA --help.";

    let config = StripperConfig {
        enable_lexical: false,
        typing_noise_config: Some(TypingNoiseConfig {
            rate: 1.0, // Attempt to corrupt everything
            seed: Some(42),
            ..Default::default()
        }),
        ..Default::default()
    };

    let res = stripper.transform(text, &config);

    // Protected formats must remain 100% intact
    assert!(res.transformed_text.contains("https://example.com/api"));
    assert!(res.transformed_text.contains("user@test.com"));
    assert!(res.transformed_text.contains("8080"));
    assert!(res.transformed_text.contains("NASA"));
    assert!(res.transformed_text.contains("--help"));
}

#[test]
fn test_error_distribution_configuration() {
    let stripper = LexiconStripper::new();
    let text = "Every morning the train arrived quietly at the platform.";

    let mut weights = HashMap::new();
    weights.insert(TypingErrorClass::Transposition, 1.0);

    let config = StripperConfig {
        enable_lexical: false,
        typing_noise_config: Some(TypingNoiseConfig {
            rate: 0.8,
            seed: Some(42),
            allowed_errors: vec![TypingErrorClass::Transposition],
            error_weights: weights,
        }),
        ..Default::default()
    };

    let res = stripper.transform(text, &config);
    let tm = res.report.typing_metrics.expect("Typing metrics present");

    for m in &tm.mutations {
        assert_eq!(m.error_class, TypingErrorClass::Transposition);
    }
}
