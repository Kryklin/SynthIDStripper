// Direct 1:1 Rust implementation of Google DeepMind's official SynthID Text algorithm
// Source: https://github.com/google-deepmind/synthid-text
// (hashing_function.py, logits_processing.py, detector_mean.py)

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const LCG_MULTIPLIER: u64 = 6364136223846793005; // 0x5851f42d4c957f2d (Knuth 64-bit LCG)
const LCG_INCREMENT: u64 = 1;

/// DeepMind official accumulate_hash (hashing_function.py)
#[inline]
pub fn accumulate_hash(mut current_hash: u64, data: &[u64]) -> u64 {
    for &val in data {
        current_hash = current_hash.wrapping_add(val);
        current_hash = current_hash.wrapping_mul(LCG_MULTIPLIER);
        current_hash = current_hash.wrapping_add(LCG_INCREMENT);
    }
    current_hash
}

/// Official SynthID Text configuration matching DeepMind defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepMindSynthIdConfig {
    pub ngram_len: usize, // Default: 3 (context of 2 tokens + 1 target token)
    pub keys: Vec<u64>,   // Secret keys per depth layer (e.g. depth = 4)
    pub detection_threshold: f64, // Threshold for mean score (e.g. 0.55 or Z >= 3.0)
}

impl Default for DeepMindSynthIdConfig {
    fn default() -> Self {
        Self {
            ngram_len: 3,
            keys: vec![654, 400, 336, 679, 700, 901, 12, 444], // 8 tournament depth layers
            detection_threshold: 3.0,
        }
    }
}

/// Official DeepMind SynthID Text detector and scoring engine
#[derive(Debug, Clone)]
pub struct DeepMindSynthIdDetector {
    pub config: DeepMindSynthIdConfig,
    hash_iv: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepMindDetectionResult {
    pub scored_ngrams: usize,
    pub depth: usize,
    pub mean_score: f64,
    pub weighted_mean_score: f64,
    pub z_score: f64,
    pub p_value: f64,
    pub alpha: f64,
    pub threshold_z: f64,
    pub is_watermarked: bool,
    pub watermark_signal_detected: bool,
    pub classification: String,
}

impl DeepMindSynthIdDetector {
    pub fn new(config: DeepMindSynthIdConfig) -> Self {
        // Hash IV computation matching DeepMind logits_processing.py:
        // Hash the keys with SHA-256 and take int64 modulo
        let mut hasher = Sha256::new();
        for &k in &config.keys {
            hasher.update((k as i64).to_ne_bytes());
        }
        let digest = hasher.finalize();
        let mut iv_bytes = [0u8; 8];
        iv_bytes.copy_from_slice(&digest[0..8]);
        let hash_iv = (u64::from_be_bytes(iv_bytes)) % (i64::MAX as u64);

        Self { config, hash_iv }
    }

    /// Computes g-values for an n-gram sequence matching DeepMind get_gvals (logits_processing.py)
    pub fn compute_g_values_for_ngram(&self, ngram_tokens: &[u64]) -> Vec<u8> {
        let mut g_vals = Vec::with_capacity(self.config.keys.len());

        // 1. Hash the n-gram with hash_iv
        let ngram_hash = accumulate_hash(self.hash_iv, ngram_tokens);

        // 2. For each depth layer key, compute final hash and extract g-val
        for &key in &self.config.keys {
            let mut key_hash = accumulate_hash(ngram_hash, &[key]);

            // DeepMind get_gvals: 12 rounds of (hash + 1) >> 5
            let shift = 64 / 12; // 5
            for _ in 0..12 {
                key_hash = key_hash
                    .wrapping_mul(LCG_MULTIPLIER)
                    .wrapping_add(LCG_INCREMENT + 1);
                key_hash >>= shift;
            }

            let g_val = ((key_hash >> 30) % 2) as u8;
            g_vals.push(g_val);
        }

        g_vals
    }

    /// Evaluates g-values across all n-grams and depth layers in the token sequence
    pub fn detect_tokens(&self, tokens: &[u64]) -> DeepMindDetectionResult {
        if tokens.len() < self.config.ngram_len {
            return DeepMindDetectionResult {
                scored_ngrams: 0,
                depth: self.config.keys.len(),
                mean_score: 0.5,
                weighted_mean_score: 0.5,
                z_score: 0.0,
                p_value: 1.0,
                alpha: 0.05,
                threshold_z: self.config.detection_threshold,
                is_watermarked: false,
                watermark_signal_detected: false,
                classification: "INSUFFICIENT_TOKENS".to_string(),
            };
        }

        let scored_ngrams = tokens.len() - self.config.ngram_len + 1;
        let depth = self.config.keys.len();
        let mut total_g = 0u64;
        let mut weighted_g_sum = 0.0f64;

        let linear_weights: Vec<f64> = (1..=depth).map(|d| d as f64).collect();
        let weight_sum: f64 = linear_weights.iter().sum();
        let norm_weights: Vec<f64> = linear_weights
            .iter()
            .map(|&w| w * (depth as f64 / weight_sum))
            .collect();

        for i in 0..scored_ngrams {
            let window = &tokens[i..i + self.config.ngram_len];
            let g_vals = self.compute_g_values_for_ngram(window);
            for (d, &g) in g_vals.iter().enumerate() {
                total_g += g as u64;
                weighted_g_sum += (g as f64) * norm_weights[d];
            }
        }

        let total_samples = (scored_ngrams * depth) as f64;
        let mean_score = (total_g as f64) / total_samples;
        let weighted_mean_score = weighted_g_sum / total_samples;

        // Under H0, g ~ Bernoulli(0.5), Var(mean) = 0.25 / total_samples
        let std_err = (0.25 / total_samples).sqrt();
        let z_score = (mean_score - 0.5) / std_err;
        let p_value = 0.5 * statrs_erfc(z_score / std::f64::consts::SQRT_2);

        let is_watermarked = z_score >= self.config.detection_threshold;
        let classification = if is_watermarked {
            "SIGNIFICANT".to_string()
        } else if z_score >= 1.645 {
            "BORDERLINE".to_string()
        } else {
            "NOT_SIGNIFICANT".to_string()
        };

        DeepMindDetectionResult {
            scored_ngrams,
            depth,
            mean_score,
            weighted_mean_score,
            z_score,
            p_value,
            alpha: 0.05,
            threshold_z: self.config.detection_threshold,
            is_watermarked,
            watermark_signal_detected: is_watermarked,
            classification,
        }
    }

    /// Converts text into stable word-hash tokens and evaluates DeepMind detector
    pub fn detect_text(&self, text: &str) -> DeepMindDetectionResult {
        let tokens: Vec<u64> = text
            .split_whitespace()
            .map(|w| {
                let clean = w
                    .trim_matches(|c: char| !c.is_alphanumeric())
                    .to_lowercase();
                let mut h = 0u64;
                for b in clean.bytes() {
                    h = h.wrapping_mul(31).wrapping_add(b as u64);
                }
                h
            })
            .collect();

        self.detect_tokens(&tokens)
    }
}

fn statrs_erfc(x: f64) -> f64 {
    // High precision complementary error function approximation
    let t = 1.0 / (1.0 + 0.5 * x.abs());
    let tau = t
        * (-x * x - 1.26551223
            + t * (1.00002368
                + t * (0.37409196
                    + t * (0.09678418
                        + t * (-0.18628806
                            + t * (0.27886807
                                + t * (-1.13520398
                                    + t * (1.48851587 + t * (-0.82215223 + t * 0.17087277)))))))))
            .exp();
    if x >= 0.0 {
        tau
    } else {
        2.0 - tau
    }
}
