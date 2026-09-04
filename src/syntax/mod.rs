pub mod valency;
pub use valency::ValencyValidator;

use crate::types::{PosTag, TransformationClass, WordTransform};
use regex::Regex;

/// Conservative, rule-based syntactic restructurer that preserves propositions and semantic roles.
pub struct SyntaxRestructurer {
    re_although: Regex,
    re_despite: Regex,
    re_some_but: Regex,
}

impl Default for SyntaxRestructurer {
    fn default() -> Self {
        Self::new()
    }
}

impl SyntaxRestructurer {
    pub fn new() -> Self {
        Self {
            // Matches: "Although [subclause], [mainclause]."
            re_although: Regex::new(r"(?i)\bAlthough\s+([^,]+),\s*([^.]+)\.").expect("Regex valid"),
            // Matches: "Despite [subclause], [mainclause]."
            re_despite: Regex::new(r"(?i)\bDespite\s+([^,]+),\s*([^.]+)\.").expect("Regex valid"),
            // Matches: "Some [clause1], but others [clause2]." -> "While some [clause1], others [clause2]."
            re_some_but: Regex::new(r"(?i)\bSome\s+([^,]+),\s*but others\s+([^.]+)\.")
                .expect("Regex valid"),
        }
    }

    /// Restructures coordination, subordination, and clause order while preserving semantic roles.
    pub fn restructure_syntax(
        &self,
        text: &str,
        transforms: &mut Vec<WordTransform>,
        pass_num: usize,
    ) -> String {
        let mut result = text.to_string();

        // 1. Invert "Although [A], [B]." -> "[B], although [A]."
        if self.re_although.is_match(&result) {
            result = self.re_although.replace_all(&result, |caps: &regex::Captures| {
                let subclause = caps.get(1).unwrap().as_str().trim();
                let mainclause = caps.get(2).unwrap().as_str().trim();

                let capitalized_main = capitalize_first(mainclause);
                let lower_sub = uncapitalize_first(subclause);

                transforms.push(WordTransform {
                    token_index: 0,
                    original: format!("Although {}, {}.", subclause, mainclause),
                    replacement: format!("{}, although {}.", capitalized_main, lower_sub),
                    original_lemma: "although".into(),
                    replacement_lemma: "although".into(),
                    pos: PosTag::IN,
                    transformation_class: TransformationClass::Syntax,
                    sense_gloss: "subordinate concessive clause inversion with semantic role conservation".into(),
                    confidence: 1.0,
                    candidates_considered: Vec::new(),
                    pass_number: pass_num,
                });

                format!("{}, although {}.", capitalized_main, lower_sub)
            }).to_string();
        }

        // 2. Invert "Despite [A], [B]." -> "[B], despite [A]."
        if self.re_despite.is_match(&result) {
            result = self
                .re_despite
                .replace_all(&result, |caps: &regex::Captures| {
                    let subclause = caps.get(1).unwrap().as_str().trim();
                    let mainclause = caps.get(2).unwrap().as_str().trim();

                    let capitalized_main = capitalize_first(mainclause);
                    let lower_sub = uncapitalize_first(subclause);

                    transforms.push(WordTransform {
                        token_index: 0,
                        original: format!("Despite {}, {}.", subclause, mainclause),
                        replacement: format!("{}, despite {}.", capitalized_main, lower_sub),
                        original_lemma: "despite".into(),
                        replacement_lemma: "despite".into(),
                        pos: PosTag::IN,
                        transformation_class: TransformationClass::Syntax,
                        sense_gloss: "prepositional concessive clause inversion".into(),
                        confidence: 1.0,
                        candidates_considered: Vec::new(),
                        pass_number: pass_num,
                    });

                    format!("{}, despite {}.", capitalized_main, lower_sub)
                })
                .to_string();
        }

        // 3. Coordination -> Subordination: "Some [A], but others [B]." -> "While some [A], others [B]."
        if self.re_some_but.is_match(&result) {
            result = self
                .re_some_but
                .replace_all(&result, |caps: &regex::Captures| {
                    let c1 = caps.get(1).unwrap().as_str().trim();
                    let c2 = caps.get(2).unwrap().as_str().trim();

                    transforms.push(WordTransform {
                        token_index: 0,
                        original: format!("Some {}, but others {}.", c1, c2),
                        replacement: format!("While some {}, others {}.", c1, c2),
                        original_lemma: "some...but".into(),
                        replacement_lemma: "while...others".into(),
                        pos: PosTag::IN,
                        transformation_class: TransformationClass::Syntax,
                        sense_gloss: "contrastive coordination to subordination restructuring"
                            .into(),
                        confidence: 1.0,
                        candidates_considered: Vec::new(),
                        pass_number: pass_num,
                    });

                    format!("While some {}, others {}.", c1, c2)
                })
                .to_string();
        }

        result
    }
}

/// Detects repetitive sentence structures and templates and introduces controlled cadence variation.
pub struct CadenceModulator {
    re_initial_adverbial: Regex,
}

impl Default for CadenceModulator {
    fn default() -> Self {
        Self::new()
    }
}

impl CadenceModulator {
    pub fn new() -> Self {
        Self {
            re_initial_adverbial: Regex::new(
                r"(?i)\b(On|During|At)\s+([a-zA-Z]+)\s+afternoons,\s*",
            )
            .expect("Regex valid"),
        }
    }

    /// Modulates repetitive cadence and opening symmetries across sentences.
    pub fn modulate_cadence(
        &self,
        text: &str,
        transforms: &mut Vec<WordTransform>,
        pass_num: usize,
    ) -> String {
        let mut result = text.to_string();

        if self.re_initial_adverbial.is_match(&result) {
            result = self
                .re_initial_adverbial
                .replace_all(&result, |caps: &regex::Captures| {
                    let prep = caps.get(1).unwrap().as_str();
                    let time_adj = caps.get(2).unwrap().as_str();

                    let rep = if prep.eq_ignore_ascii_case("On") {
                        format!("During {} afternoons, ", time_adj)
                    } else {
                        format!("On {} afternoons, ", time_adj)
                    };

                    transforms.push(WordTransform {
                        token_index: 0,
                        original: format!("{} {} afternoons, ", prep, time_adj),
                        replacement: rep.clone(),
                        original_lemma: prep.to_lowercase(),
                        replacement_lemma: "during".into(),
                        pos: PosTag::IN,
                        transformation_class: TransformationClass::Cadence,
                        sense_gloss: "temporal adverbial cadence variation".into(),
                        confidence: 1.0,
                        candidates_considered: Vec::new(),
                        pass_number: pass_num,
                    });

                    rep
                })
                .to_string();
        }

        result
    }
}

/// Controlled function-word and discourse connector alternatives with strict logical equivalence.
pub struct FunctionWordModulator;

impl Default for FunctionWordModulator {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionWordModulator {
    pub fn new() -> Self {
        Self
    }

    /// Modulates discourse connectors and conjunctions with semantic equivalence.
    pub fn modulate_function_words(
        &self,
        text: &str,
        transforms: &mut Vec<WordTransform>,
        pass_num: usize,
    ) -> String {
        let mut result = text.to_string();

        let patterns = [
            (
                "In recent years,",
                "Over the past few years,",
                "in recent years",
                "over the past few years",
                "discourse temporal connector",
            ),
            (
                "in recent years,",
                "over the past few years,",
                "in recent years",
                "over the past few years",
                "discourse temporal connector",
            ),
            (
                "Every morning,",
                "Each morning,",
                "every morning",
                "each morning",
                "distributive determiner connector",
            ),
            (
                "every morning,",
                "each morning,",
                "every morning",
                "each morning",
                "distributive determiner connector",
            ),
            (
                "was not",
                "wasn't",
                "was not",
                "wasn't",
                "negative auxiliary contraction",
            ),
            (
                "could not",
                "couldn't",
                "could not",
                "couldn't",
                "modal negative contraction",
            ),
            (
                "would not",
                "wouldn't",
                "would not",
                "wouldn't",
                "modal negative contraction",
            ),
        ];

        for (from, to, orig_lem, rep_lem, gloss) in &patterns {
            if result.contains(from) {
                result = result.replace(from, to);
                transforms.push(WordTransform {
                    token_index: 0,
                    original: (*from).into(),
                    replacement: (*to).into(),
                    original_lemma: (*orig_lem).into(),
                    replacement_lemma: (*rep_lem).into(),
                    pos: PosTag::IN,
                    transformation_class: TransformationClass::FunctionWord,
                    sense_gloss: (*gloss).into(),
                    confidence: 1.0,
                    candidates_considered: Vec::new(),
                    pass_number: pass_num,
                });
            }
        }

        result
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn uncapitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
    }
}
