pub mod errors;
pub mod keyboard;

pub use errors::{
    apply_duplication, apply_insertion, apply_omission, apply_substitution,
    apply_temporal_transition, apply_transposition, ErrorMutationResult,
};
pub use keyboard::{Key, KeyboardModel};

use crate::tokenization::Token;
pub use crate::types::TypingErrorClass;
use crate::types::{TypingMutationRecord, TypingNoiseConfig, TypingNoiseMetrics};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

/// Human typing noise simulator engine
#[derive(Debug, Clone)]
pub struct TypingNoiseEngine {
    keyboard: KeyboardModel,
}

impl Default for TypingNoiseEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TypingNoiseEngine {
    pub fn new() -> Self {
        Self {
            keyboard: KeyboardModel::default(),
        }
    }

    /// Determines if a token is eligible for human typing noise
    pub fn is_token_eligible(&self, tokens: &[Token], i: usize) -> bool {
        let token = &tokens[i];
        if !token.is_word() {
            return false;
        }

        let text = token.text.trim();
        if text.is_empty() || text.len() < 2 {
            return false;
        }

        // Must not be already transformed if fixed-point protection applies
        if token.is_transformed {
            return false;
        }

        // Named entity / proper noun protection
        if token.pos == crate::types::PosTag::NNP || token.pos == crate::types::PosTag::NNPS {
            return false;
        }

        // Preceding hyphen/dash indicates CLI option/flag
        if i > 0 && (tokens[i - 1].text == "--" || tokens[i - 1].text == "-") {
            return false;
        }

        // Text protection filter
        if is_protected_format(text) {
            return false;
        }

        // All characters must be letters or standard hyphens/apostrophes
        if text.chars().any(|c| c.is_numeric()) {
            return false;
        }

        true
    }

    /// Simulates human typing noise across an array of tokens
    pub fn apply_typing_noise(
        &self,
        tokens: &mut [Token],
        config: &TypingNoiseConfig,
        master_seed: Option<u64>,
    ) -> TypingNoiseMetrics {
        let mut rng = match config.seed.or(master_seed) {
            Some(s) => StdRng::seed_from_u64(s.wrapping_add(987654321)),
            None => StdRng::from_entropy(),
        };

        let mut eligible_tokens = 0;
        let mut corrupted_tokens = 0;
        let mut error_class_counts: HashMap<String, usize> = HashMap::new();
        let mut mutations: Vec<TypingMutationRecord> = Vec::new();
        let mut total_keyboard_distance = 0.0;
        let mut distance_sample_count = 0;

        
        let mut word_edit_distance = 0;

        let original_full_text: String = tokens.iter().map(|t| t.text.as_str()).collect();

        // 1. Iterate over tokens and apply stochastic error selection
        for i in 0..tokens.len() {
            if !self.is_token_eligible(tokens, i) {
                continue;
            }

            eligible_tokens += 1;

            // Check rate threshold
            if config.rate <= 0.0 || !rng.gen_bool(config.rate.min(1.0)) {
                continue;
            }

            // Select error class according to configured weights
            let error_class =
                sample_error_class(&config.allowed_errors, &config.error_weights, &mut rng);

            let original_word = tokens[i].text.clone();
            let mutation_opt = match error_class {
                TypingErrorClass::Substitution => {
                    apply_substitution(&original_word, &self.keyboard, &mut rng)
                }
                TypingErrorClass::Transposition => {
                    apply_transposition(&original_word, &self.keyboard, &mut rng)
                }
                TypingErrorClass::Omission => apply_omission(&original_word, &mut rng),
                TypingErrorClass::Insertion => {
                    apply_insertion(&original_word, &self.keyboard, &mut rng)
                }
                TypingErrorClass::Duplication => apply_duplication(&original_word, &mut rng),
                TypingErrorClass::Temporal => {
                    apply_temporal_transition(&original_word, &self.keyboard, &mut rng)
                }
            };

            if let Some(mutation) = mutation_opt {
                // Quality control: don't apply empty or identical mutations
                if mutation.mutated_word == original_word || mutation.mutated_word.trim().is_empty()
                {
                    continue;
                }

                tokens[i].text = mutation.mutated_word.clone();
                tokens[i].is_transformed = true;
                tokens[i]
                    .transformation_history
                    .push(mutation.mutated_word.clone());

                corrupted_tokens += 1;
                word_edit_distance += 1;
                *error_class_counts
                    .entry(mutation.error_class.as_str().to_string())
                    .or_insert(0) += 1;

                if mutation.keyboard_distance > 0.0 {
                    total_keyboard_distance += mutation.keyboard_distance;
                    distance_sample_count += 1;
                }

                mutations.push(TypingMutationRecord {
                    token_index: i,
                    original_word,
                    mutated_word: mutation.mutated_word,
                    error_class: mutation.error_class,
                    char_position: mutation.char_position,
                    original_char: mutation.original_char,
                    mutated_char: mutation.mutated_char,
                    keyboard_distance: mutation.keyboard_distance,
                });
            }
        }

        let mutated_full_text: String = tokens.iter().map(|t| t.text.as_str()).collect();
        let total_char_edit_distance = levenshtein_distance(&original_full_text, &mutated_full_text);

        let actual_error_rate = if eligible_tokens > 0 {
            corrupted_tokens as f64 / eligible_tokens as f64
        } else {
            0.0
        };

        let mean_keyboard_distance = if distance_sample_count > 0 {
            total_keyboard_distance / distance_sample_count as f64
        } else {
            0.0
        };

        TypingNoiseMetrics {
            rate_configured: config.rate,
            eligible_tokens,
            corrupted_tokens,
            actual_error_rate,
            error_class_counts,
            mean_keyboard_distance,
            character_edit_distance: total_char_edit_distance,
            word_edit_distance,
            mutations,
            seed_used: config.seed.or(master_seed),
        }
    }
}

/// Samples an error class based on configured weights
fn sample_error_class<R: Rng>(
    allowed: &[TypingErrorClass],
    weights_map: &HashMap<TypingErrorClass, f64>,
    rng: &mut R,
) -> TypingErrorClass {
    if allowed.is_empty() {
        return TypingErrorClass::Substitution;
    }
    if allowed.len() == 1 {
        return allowed[0];
    }

    let weights: Vec<f64> = allowed
        .iter()
        .map(|cls| *weights_map.get(cls).unwrap_or(&1.0))
        .collect();

    let total: f64 = weights.iter().sum();
    if total <= 0.0 {
        return allowed[0];
    }

    let mut r = rng.gen_range(0.0..total);
    for (i, &w) in weights.iter().enumerate() {
        r -= w;
        if r <= 0.0 {
            return allowed[i];
        }
    }

    *allowed.last().unwrap()
}

/// Protects URLs, emails, file paths, numbers, code, and technical formats
fn is_protected_format(text: &str) -> bool {
    // URL prefixes
    if text.starts_with("http://")
        || text.starts_with("https://")
        || text.starts_with("www.")
        || text.starts_with("ftp://")
    {
        return true;
    }

    // Email
    if text.contains('@') && text.contains('.') {
        return true;
    }

    // File path or extensions
    if text.contains('/')
        || text.contains('\\')
        || text.ends_with(".rs")
        || text.ends_with(".txt")
        || text.ends_with(".json")
    {
        return true;
    }

    // CLI flags
    if text.starts_with("--") || text.starts_with('-') {
        return true;
    }

    // Code symbols / camelCase / snake_case
    if text.contains('_') || text.contains("::") || text.contains("()") {
        return true;
    }

    // All uppercase acronyms (e.g. NASA, API, SHA)
    if text.len() >= 2 && text.chars().all(|c| c.is_uppercase()) {
        return true;
    }

    false
}

/// Computes character-level Levenshtein edit distance between two strings
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let v1: Vec<char> = s1.chars().collect();
    let v2: Vec<char> = s2.chars().collect();

    let len1 = v1.len();
    let len2 = v2.len();

    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        dp[i][0] = i;
    }
    for j in 0..=len2 {
        dp[0][j] = j;
    }

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if v1[i - 1] == v2[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[len1][len2]
}
