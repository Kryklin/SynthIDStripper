use crate::types::{CoarsePos, PosTag};
use std::collections::HashSet;

lazy_static::lazy_static! {
    /// Verbs that license a "from + [gerund/NP]" prepositional complement
    static ref FROM_LICENSED_VERBS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "keep", "prevent", "stop", "deter", "shield", "protect", "save",
            "block", "prohibit", "restrain", "separate", "withhold", "discourage",
            "hide", "refrain", "cease", "abstain", "derive", "recover", "escape",
            "differ", "emerge", "arise",
        ] {
            s.insert(w);
        }
        s
    };

    /// Verbs that are notorious false synonyms for "keep" but DO NOT license "from + gerund"
    static ref FROM_DISALLOWED_VERBS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "maintain", "preserve", "sustain", "uphold", "continue", "conserve",
            "operate", "run", "create", "construct", "build", "elect", "retain",
            "manage", "support", "foster",
        ] {
            s.insert(w);
        }
        s
    };

    /// Verbs that cannot take "point" in the sense of reaching a state or threshold
    static ref POINT_DISALLOWED_VERBS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &["achieve", "accomplish", "attain", "execute", "fulfill", "complete"] {
            s.insert(w);
        }
        s
    };

    /// Verbs that cannot take "word" as direct object in lexical selection context
    static ref WORD_DISALLOWED_VERBS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &["elect", "single", "designate"] {
            s.insert(w);
        }
        s
    };

    /// Verbs that are false substitutions for "keep" in causative/resultative state constructions
    static ref KEEP_STATE_DISALLOWED_VERBS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &["sustain", "preserve", "uphold", "conserve", "protect", "safeguard", "maintain", "retain"] {
            s.insert(w);
        }
        s
    };

    /// Participial or predicative adjective complements commonly governed by "keep"
    static ref STATE_PREDICATE_COMPLEMENTS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "entertained", "happy", "interested", "engaged", "informed",
            "updated", "calm", "safe", "afloat", "alive", "open", "running",
            "occupied", "satisfied", "alert", "awake", "clean", "quiet",
        ] {
            s.insert(w);
        }
        s
    };
}

/// Validates syntactic valency, verb-particle bindings, and complement structure compatibility.
#[derive(Debug, Clone, Default)]
pub struct ValencyValidator;

impl ValencyValidator {
    pub fn new() -> Self {
        Self
    }

    /// Checks if a candidate lemma is syntactically and valency-compatible with local prepositional and complement structures.
    pub fn is_valency_compatible(
        &self,
        candidate_lemma: &str,
        original_lemma: &str,
        pos: PosTag,
        words_after: &[&str],
    ) -> bool {
        let cand_lower = candidate_lemma.to_lowercase();
        let orig_lower = original_lemma.to_lowercase();

        // Verb valency checks
        if pos.coarse_pos() == CoarsePos::Verb {
            let lookahead_limit = words_after.len().min(8);
            let lookahead = &words_after[..lookahead_limit];

            // 1. "from + [gerund / NP]" frame check
            let has_from = lookahead.iter().any(|&w| w.eq_ignore_ascii_case("from"));
            if has_from {
                if FROM_DISALLOWED_VERBS.contains(cand_lower.as_str()) {
                    return false;
                }
                if FROM_LICENSED_VERBS.contains(orig_lower.as_str())
                    && !FROM_LICENSED_VERBS.contains(cand_lower.as_str())
                {
                    return false;
                }
            }

            // 2. Direct object "point" in lookahead
            let has_point = lookahead.iter().any(|&w| {
                let clean = w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
                clean == "point" || clean == "stage" || clean == "threshold"
            });
            if has_point && POINT_DISALLOWED_VERBS.contains(cand_lower.as_str()) {
                return false;
            }

            // 3. Direct object "word" in lookahead
            let has_word = lookahead.iter().any(|&w| {
                let clean = w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
                clean == "word" || clean == "token" || clean == "term"
            });
            if has_word && WORD_DISALLOWED_VERBS.contains(cand_lower.as_str()) {
                return false;
            }

            // 4. "keep [NP] [adjective complement]" resultative state frame check
            if orig_lower == "keep" {
                let has_state_complement = lookahead.iter().any(|&w| {
                    let clean = w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
                    STATE_PREDICATE_COMPLEMENTS.contains(clean.as_str())
                });
                if has_state_complement && KEEP_STATE_DISALLOWED_VERBS.contains(cand_lower.as_str()) {
                    return false;
                }
            }
        }

        true
    }
}
