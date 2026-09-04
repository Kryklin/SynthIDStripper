use crate::detector::{SynthIdConfig, SynthIdDetector};
use crate::lexicon::{CollocationDb, FrequencyDb};
use crate::morphology::Inflector;
use crate::syntax::ValencyValidator;
use crate::tokenization::Casing;
use crate::types::{
    CandidateScore, DistributionMode, PosTag, SelectionStrategy, StripperConfig,
    TransformationAuditRecord,
};
use rand::rngs::StdRng;
use rand::Rng;

pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();
    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }
    dp[m][n]
}

pub struct DistributionSampler {
    frequency_db: FrequencyDb,
    inflector: Inflector,
    detector: SynthIdDetector,
    valency_validator: ValencyValidator,
    collocation_db: CollocationDb,
}

impl Default for DistributionSampler {
    fn default() -> Self {
        Self::new()
    }
}

impl DistributionSampler {
    pub fn new() -> Self {
        Self {
            frequency_db: FrequencyDb::new(),
            inflector: Inflector::new(),
            detector: SynthIdDetector::new(SynthIdConfig::default()),
            valency_validator: ValencyValidator::new(),
            collocation_db: CollocationDb::new(),
        }
    }

    /// Evaluates candidate lemmas, computes weights and probabilities according to distribution mode and selection strategy,
    /// measures local detector g-value contributions, and samples a candidate with full audit instrumentation.
    pub fn sample_candidate_with_audit(
        &self,
        candidates: &[&str],
        original_token: &str,
        target_pos: PosTag,
        casing: Casing,
        semantic_confidence: f64,
        words_before: &[&str],
        words_after: &[&str],
        domain_name: &str,
        is_protected_term: bool,
        config: &StripperConfig,
        rng: &mut StdRng,
    ) -> Option<(
        String,
        String,
        f64,
        Vec<CandidateScore>,
        TransformationAuditRecord,
    )> {
        if candidates.is_empty() {
            return None;
        }

        // 1. Compute baseline detector contribution for original token in local context windows
        let orig_clean = original_token
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();
        let g_orig = self.compute_local_g_sum(words_before, &orig_clean, words_after);

        let mut candidate_scores: Vec<CandidateScore> = Vec::new();
        let mut candidate_audits = Vec::new();
        let mut weights: Vec<f64> = Vec::new();

        for (rank, &lemma) in candidates.iter().enumerate() {
            let inflected = self.inflector.inflect(lemma, target_pos, casing);
            let inflected_clean = inflected
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase();

            // Syntactic valency and preposition binding verification
            if !self.valency_validator.is_valency_compatible(
                lemma,
                &orig_clean,
                target_pos,
                words_after,
            ) {
                continue;
            }

            // Collocation and n-gram affinity verification
            let colloc_affinity = self.collocation_db.evaluate_collocation_affinity(
                &inflected_clean,
                lemma,
                words_before,
                words_after,
            );
            if colloc_affinity <= 0.0 {
                continue;
            }

            let freq_weight =
                self.frequency_db
                    .get_candidate_weight(lemma, config.mode, config.temperature);

            // Compute candidate local detector contribution
            let g_cand = self.compute_local_g_sum(words_before, &inflected_clean, words_after);
            let delta_g = g_cand - g_orig;
            let edit_dist = levenshtein_distance(original_token, &inflected);
            let semantic_cost = (1.0 - semantic_confidence).max(0.0);
            let contextual_disruption = delta_g / (edit_dist as f64 + 1.0);

            let eff_per_rep = -delta_g;
            let eff_per_edit = -delta_g / (edit_dist as f64 + 1.0);
            let eff_per_sem_cost = -delta_g / (semantic_cost + 0.05);

            // Strategy-specific weight calculation
            let strategy_weight = match config.selection_strategy {
                SelectionStrategy::Baseline => {
                    // Production Zipf human baseline weight
                    freq_weight * semantic_confidence * colloc_affinity
                }
                SelectionStrategy::Uniform => {
                    // Uniform random over valid candidates
                    1.0 * colloc_affinity
                }
                SelectionStrategy::ContextNeutral => {
                    // Balances lexical diversity and length difference without frequency bias
                    let len_diff = (inflected.len() as f64 - original_token.len() as f64).abs();
                    (semantic_confidence / (1.0 + len_diff * 0.15)) * colloc_affinity
                }
                SelectionStrategy::EfficiencyGuided => {
                    // Softmax temperature weighting favoring negative delta_g (disrupting detector signal)
                    let tau = 0.5;
                    let exp_factor = (-delta_g / tau).exp().clamp(0.01, 100.0);
                    exp_factor * semantic_confidence * colloc_affinity
                }
                SelectionStrategy::ConservativeEfficiency => {
                    // Stricter semantic threshold required before efficiency boost
                    if semantic_confidence >= 0.65 {
                        let tau = 0.5;
                        let exp_factor = (-delta_g / tau).exp().clamp(0.01, 100.0);
                        exp_factor * semantic_confidence * colloc_affinity
                    } else {
                        freq_weight * semantic_confidence * 0.2 * colloc_affinity
                    }
                }
            };

            weights.push(strategy_weight.max(1e-6));

            candidate_scores.push(CandidateScore {
                lemma: lemma.to_string(),
                inflected: inflected.clone(),
                semantic_score: semantic_confidence,
                frequency_weight: freq_weight,
                final_probability: 0.0,
            });

            candidate_audits.push((
                rank,
                inflected.clone(),
                g_orig,
                g_cand,
                delta_g,
                contextual_disruption,
                edit_dist,
                eff_per_rep,
                eff_per_edit,
                eff_per_sem_cost,
            ));
        }

        if candidate_scores.is_empty() {
            return None;
        }

        let total_weight: f64 = weights.iter().sum();
        if total_weight <= 0.0 {
            return None;
        }

        // Normalize probabilities
        for (i, cs) in candidate_scores.iter_mut().enumerate() {
            cs.final_probability = weights[i] / total_weight;
        }

        // Perform sampling
        let selected_idx = match config.selection_strategy {
            SelectionStrategy::Uniform => rng.gen_range(0..candidate_scores.len()),
            _ => match config.mode {
                DistributionMode::Random => rng.gen_range(0..candidate_scores.len()),
                _ => {
                    let p: f64 = rng.gen_range(0.0..1.0);
                    let mut cumulative = 0.0;
                    let mut chosen = 0;
                    for (idx, cs) in candidate_scores.iter().enumerate() {
                        cumulative += cs.final_probability;
                        if p <= cumulative {
                            chosen = idx;
                            break;
                        }
                    }
                    chosen
                }
            },
        };

        let selected = &candidate_scores[selected_idx];
        let chosen_lemma = selected.lemma.clone();
        let chosen_inflected = selected.inflected.clone();
        let confidence = selected.semantic_score;

        let (
            rank,
            rep_text,
            g_before,
            g_after,
            delta_g,
            ctx_disrupt,
            edit_dist,
            eff_rep,
            eff_edit,
            eff_sem,
        ) = &candidate_audits[selected_idx];

        let ctx_before_str = format!(
            "{} [{}] {}",
            words_before.join(" "),
            original_token,
            words_after.join(" ")
        )
        .trim()
        .to_string();

        let ctx_after_str = format!(
            "{} [{}] {}",
            words_before.join(" "),
            rep_text,
            words_after.join(" ")
        )
        .trim()
        .to_string();

        let audit_rec = TransformationAuditRecord {
            original_token: original_token.to_string(),
            replacement_token: chosen_inflected.clone(),
            lemma: chosen_lemma.clone(),
            pos: target_pos,
            domain_classification: domain_name.to_string(),
            is_protected_terminology_span: is_protected_term,
            semantic_confidence: confidence,
            candidate_pool_size: candidates.len(),
            selected_candidate_rank: *rank,
            local_context_before: ctx_before_str,
            local_context_after: ctx_after_str,
            detector_contribution_before: *g_before,
            detector_contribution_after: *g_after,
            local_delta_g: *delta_g,
            local_contextual_disruption: *ctx_disrupt,
            edit_distance: *edit_dist,
            is_accepted: true,
            detector_effect_per_replacement: *eff_rep,
            detector_effect_per_edit: *eff_edit,
            detector_effect_per_semantic_cost: *eff_sem,
        };

        Some((
            chosen_lemma,
            chosen_inflected,
            confidence,
            candidate_scores,
            audit_rec,
        ))
    }

    /// Computes the sum of g-values for all overlapping context windows involving the target token.
    fn compute_local_g_sum(&self, before: &[&str], token: &str, after: &[&str]) -> f64 {
        let mut total_g = 0.0;

        // Window 1: (w_{i-2}, w_{i-1} -> token)
        if before.len() >= 2 {
            let ctx = [before[before.len() - 2], before[before.len() - 1]];
            total_g += self.detector.compute_g_value(&ctx, token);
        }

        // Window 2: (w_{i-1}, token -> w_{i+1})
        if !before.is_empty() && !after.is_empty() {
            let ctx = [before[before.len() - 1], token];
            total_g += self.detector.compute_g_value(&ctx, after[0]);
        }

        // Window 3: (token, w_{i+1} -> w_{i+2})
        if after.len() >= 2 {
            let ctx = [token, after[0]];
            total_g += self.detector.compute_g_value(&ctx, after[1]);
        }

        total_g
    }
}
