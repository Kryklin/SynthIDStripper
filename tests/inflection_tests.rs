use lexicon_stripper::morphology::{Inflector, Lemmatizer};
use lexicon_stripper::tokenization::Casing;
use lexicon_stripper::types::PosTag;

#[test]
fn test_noun_plural_inflection() {
    let inflector = Inflector::new();
    let lemmatizer = Lemmatizer::new();

    // Regular
    assert_eq!(inflector.inflect("cat", PosTag::NNS, Casing::Lower), "cats");
    assert_eq!(
        inflector.inflect("box", PosTag::NNS, Casing::Lower),
        "boxes"
    );
    assert_eq!(
        inflector.inflect("city", PosTag::NNS, Casing::Lower),
        "cities"
    );
    assert_eq!(inflector.inflect("day", PosTag::NNS, Casing::Lower), "days");

    // Irregular
    assert_eq!(
        inflector.inflect("child", PosTag::NNS, Casing::Lower),
        "children"
    );
    assert_eq!(
        inflector.inflect("person", PosTag::NNS, Casing::Lower),
        "people"
    );
    assert_eq!(
        inflector.inflect("criterion", PosTag::NNS, Casing::Lower),
        "criteria"
    );
    assert_eq!(
        inflector.inflect("analysis", PosTag::NNS, Casing::Lower),
        "analyses"
    );

    // Lemmatization reverse
    assert_eq!(lemmatizer.lemmatize("children", PosTag::NNS), "child");
    assert_eq!(lemmatizer.lemmatize("cities", PosTag::NNS), "city");
    assert_eq!(lemmatizer.lemmatize("criteria", PosTag::NNS), "criterion");
}

#[test]
fn test_verb_tense_inflection() {
    let inflector = Inflector::new();
    let lemmatizer = Lemmatizer::new();

    // 3rd singular present (VBZ)
    assert_eq!(inflector.inflect("run", PosTag::VBZ, Casing::Lower), "runs");
    assert_eq!(
        inflector.inflect("create", PosTag::VBZ, Casing::Lower),
        "creates"
    );
    assert_eq!(
        inflector.inflect("study", PosTag::VBZ, Casing::Lower),
        "studies"
    );
    assert_eq!(inflector.inflect("go", PosTag::VBZ, Casing::Lower), "goes");

    // Past tense (VBD)
    assert_eq!(
        inflector.inflect("modify", PosTag::VBD, Casing::Lower),
        "modified"
    );
    assert_eq!(
        inflector.inflect("transform", PosTag::VBD, Casing::Lower),
        "transformed"
    );
    assert_eq!(
        inflector.inflect("make", PosTag::VBD, Casing::Lower),
        "made"
    );
    assert_eq!(
        inflector.inflect("take", PosTag::VBD, Casing::Lower),
        "took"
    );
    assert_eq!(
        inflector.inflect("show", PosTag::VBD, Casing::Lower),
        "showed"
    );

    // Gerund (VBG)
    assert_eq!(
        inflector.inflect("run", PosTag::VBG, Casing::Lower),
        "running"
    );
    assert_eq!(
        inflector.inflect("transform", PosTag::VBG, Casing::Lower),
        "transforming"
    );
    assert_eq!(
        inflector.inflect("create", PosTag::VBG, Casing::Lower),
        "creating"
    );
    assert_eq!(
        inflector.inflect("lie", PosTag::VBG, Casing::Lower),
        "lying"
    );

    // Lemmatizer reverse
    assert_eq!(
        lemmatizer.lemmatize("transformed", PosTag::VBD),
        "transform"
    );
    assert_eq!(lemmatizer.lemmatize("creating", PosTag::VBG), "create");
    assert_eq!(lemmatizer.lemmatize("went", PosTag::VBD), "go");
}

#[test]
fn test_casing_preservation() {
    let inflector = Inflector::new();

    assert_eq!(
        inflector.inflect("transform", PosTag::VBD, Casing::Lower),
        "transformed"
    );
    assert_eq!(
        inflector.inflect("transform", PosTag::VBD, Casing::Title),
        "Transformed"
    );
    assert_eq!(
        inflector.inflect("transform", PosTag::VBD, Casing::Upper),
        "TRANSFORMED"
    );
}
