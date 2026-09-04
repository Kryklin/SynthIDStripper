use crate::lexicon::{Synset, STOPWORDS};
use crate::morphology::Lemmatizer;
use crate::tokenization::Token;
use std::collections::HashSet;

pub struct DisambiguationResult {
    pub best_synset: Option<&'static Synset>,
    pub confidence: f64,
    pub overlap_terms: Vec<String>,
}

pub struct SenseDisambiguator {
    lemmatizer: Lemmatizer,
}

impl Default for SenseDisambiguator {
    fn default() -> Self {
        Self::new()
    }
}

impl SenseDisambiguator {
    pub fn new() -> Self {
        Self {
            lemmatizer: Lemmatizer::new(),
        }
    }

    /// Computes contextual sense disambiguation using an enhanced Lesk algorithm.
    pub fn disambiguate(
        &self,
        tokens: &[Token],
        target_idx: usize,
        candidate_synsets: &[&'static Synset],
        context_window_size: usize,
    ) -> DisambiguationResult {
        if candidate_synsets.is_empty() {
            return DisambiguationResult {
                best_synset: None,
                confidence: 0.0,
                overlap_terms: Vec::new(),
            };
        }

        // 1. Extract context words from window surrounding target_idx
        let start = target_idx.saturating_sub(context_window_size);
        let end = (target_idx + context_window_size + 1).min(tokens.len());

        let mut context_lemmas: HashSet<String> = HashSet::new();
        for (idx, tok) in tokens[start..end].iter().enumerate() {
            let actual_idx = start + idx;
            if actual_idx == target_idx {
                continue;
            }
            if tok.is_word() {
                let lemma = self.lemmatizer.lemmatize(&tok.text, tok.pos);
                let lower = lemma.to_lowercase();
                if !STOPWORDS.contains(lower.as_str()) && lower.len() > 2 {
                    context_lemmas.insert(lower);
                }
            }
        }

        let mut best_synset = candidate_synsets[0];
        let mut best_score = -1.0;
        let mut best_overlaps = Vec::new();

        for &synset in candidate_synsets {
            // Tokenize gloss
            let gloss_words: Vec<&str> = synset
                .gloss
                .split(|c: char| !c.is_alphanumeric())
                .filter(|s| !s.is_empty() && s.len() > 2)
                .collect();

            let mut overlaps = Vec::new();
            let mut raw_score = 0.0;

            for &gword in &gloss_words {
                let g_lower = gword.to_lowercase();
                if !STOPWORDS.contains(g_lower.as_str()) && context_lemmas.contains(&g_lower) {
                    overlaps.push(g_lower);
                    raw_score += 1.5;
                }
            }

            // Also check overlap with synset lemmas
            for &lemma in synset.lemmas {
                let l_lower = lemma.to_lowercase();
                if context_lemmas.contains(&l_lower) {
                    overlaps.push(l_lower);
                    raw_score += 1.0;
                }
            }

            // Length-normalized scoring with ambiguity-calibrated base confidence
            let normalized_score = if gloss_words.is_empty() {
                if candidate_synsets.len() == 1 {
                    0.75
                } else {
                    0.50
                }
            } else {
                let base = if candidate_synsets.len() == 1 {
                    0.75 // Unambiguous vetted synset
                } else {
                    0.45 // Polysemous word requiring context overlap confirmation
                };
                let bonus = (raw_score / (1.0 + (gloss_words.len() as f64).sqrt())) * 0.55;
                (base + bonus).min(1.0)
            };

            if normalized_score > best_score {
                best_score = normalized_score;
                best_synset = synset;
                best_overlaps = overlaps;
            }
        }

        DisambiguationResult {
            best_synset: Some(best_synset),
            confidence: best_score.max(0.4),
            overlap_terms: best_overlaps,
        }
    }
}
