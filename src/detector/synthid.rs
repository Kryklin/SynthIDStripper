use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Configuration for SynthID Text watermarking and detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthIdConfig {
    /// Secret key (developer seed)
    pub key: u64,
    /// Context length k (n-gram history used for pseudo-random hash, typically 2 or 3)
    pub context_length: usize,
    /// Minimum z-score threshold for positive watermark detection (default: 3.0)
    pub detection_threshold: f64,
}

impl Default for SynthIdConfig {
    fn default() -> Self {
        Self {
            key: 428917492,
            context_length: 2,
            detection_threshold: 3.0,
        }
    }
}

/// Detailed result of a SynthID watermark detection scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthIdDetectionResult {
    /// Whether the watermark was statistically detected (z-score >= threshold)
    pub is_watermarked: bool,
    /// Boolean flag indicating whether a statistically significant watermark signal was detected
    pub watermark_signal_detected: bool,
    /// Statistical classification: "SIGNIFICANT", "BORDERLINE", "NOT_SIGNIFICANT"
    pub classification: String,
    /// Standardized test statistic Z-score
    pub z_score: f64,
    /// p-value under the null hypothesis (unwatermarked uniform g-values)
    pub p_value: f64,
    /// Alpha significance level for decision boundary
    pub alpha: f64,
    /// Critical Z-score threshold for detection
    pub threshold_z: f64,
    /// Mean observed g-value across evaluated token windows (null expectation = 0.500)
    pub mean_g_value: f64,
    /// Number of n-gram context windows evaluated
    pub evaluated_windows: usize,
    /// Context length k used
    pub context_length: usize,
}

/// Pure Rust implementation of Google DeepMind's SynthID Text detection and watermarking algorithms.
pub struct SynthIdDetector {
    config: SynthIdConfig,
}

impl SynthIdDetector {
    pub fn new(config: SynthIdConfig) -> Self {
        Self { config }
    }

    /// Computes the pseudo-random g-value in [0.0, 1.0] for a token given its preceding context.
    pub fn compute_g_value(&self, context: &[&str], token: &str) -> f64 {
        let mut hasher = Sha256::new();
        hasher.update(self.config.key.to_be_bytes());
        for c in context {
            hasher.update(c.to_lowercase().as_bytes());
            hasher.update(b"|");
        }
        hasher.update(token.to_lowercase().as_bytes());
        let hash = hasher.finalize();

        // Use the first 8 bytes of SHA-256 to produce uniform float in [0.0, 1.0]
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&hash[0..8]);
        let val = u64::from_be_bytes(bytes);
        (val as f64) / (u64::MAX as f64)
    }

    /// Detects SynthID watermark presence in a text sequence by evaluating cumulative g-values and Z-score.
    pub fn detect(&self, text: &str) -> SynthIdDetectionResult {
        let words: Vec<&str> = text
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|w| !w.is_empty())
            .collect();

        let k = self.config.context_length;
        if words.len() <= k {
            return SynthIdDetectionResult {
                is_watermarked: false,
                watermark_signal_detected: false,
                classification: "INSUFFICIENT_LENGTH".to_string(),
                z_score: 0.0,
                p_value: 1.0,
                alpha: 0.05,
                threshold_z: self.config.detection_threshold,
                mean_g_value: 0.5,
                evaluated_windows: 0,
                context_length: k,
            };
        }

        let mut g_values = Vec::new();
        for i in k..words.len() {
            let context = &words[i - k..i];
            let token = words[i];
            let g = self.compute_g_value(context, token);
            g_values.push(g);
        }

        let n = g_values.len() as f64;
        let sum_g: f64 = g_values.iter().sum();
        let mean_g = sum_g / n;

        // Under null hypothesis (unwatermarked text), g ~ Uniform(0, 1) => mean = 0.5, var = 1/12
        let mu = 0.5;
        let sigma = (1.0 / (12.0 * n)).sqrt();
        let z_score = (mean_g - mu) / sigma;

        // One-tailed p-value using complementary error function approximation
        let p_value = 0.5 * erfc(z_score / std::f64::consts::SQRT_2);

        let (is_watermarked, classification) = if z_score >= self.config.detection_threshold {
            (true, "STRONG_SIGNAL".to_string())
        } else if z_score >= 1.645 {
            (false, "BORDERLINE".to_string())
        } else {
            (false, "NOT_SIGNIFICANT".to_string())
        };

        SynthIdDetectionResult {
            is_watermarked,
            watermark_signal_detected: is_watermarked,
            classification,
            z_score,
            p_value,
            alpha: 0.05,
            threshold_z: self.config.detection_threshold,
            mean_g_value: mean_g,
            evaluated_windows: g_values.len(),
            context_length: k,
        }
    }
}

/// Reference SynthID Text Watermarker that biases candidate selection toward high g-values.
pub struct SynthIdWatermarker {
    pub detector: SynthIdDetector,
}

impl SynthIdWatermarker {
    pub fn new(config: SynthIdConfig) -> Self {
        Self {
            detector: SynthIdDetector::new(config),
        }
    }

    pub fn compute_g_value(&self, context: &[&str], word: &str) -> f64 {
        self.detector.compute_g_value(context, word)
    }

    /// Embeds a SynthID watermark into a word stream by choosing among candidate synonyms with highest g-value.
    pub fn watermark_text(
        &self,
        words: &[&str],
        synonyms_provider: impl Fn(&str) -> Vec<String>,
    ) -> String {
        let k = self.detector.config.context_length;
        let mut result_words: Vec<String> = Vec::new();

        for (i, &word) in words.iter().enumerate() {
            let candidates = synonyms_provider(word);
            if candidates.is_empty() || i < k {
                result_words.push(word.to_string());
                continue;
            }

            let context_slices: Vec<&str> = result_words[result_words.len() - k..]
                .iter()
                .map(|s| s.as_str())
                .collect();

            // Tournament selection: pick candidate with maximum g-value
            let mut best_cand = word.to_string();
            let mut max_g = self.detector.compute_g_value(&context_slices, word);

            for cand in &candidates {
                let g = self.detector.compute_g_value(&context_slices, cand);
                if g > max_g {
                    max_g = g;
                    best_cand = cand.clone();
                }
            }

            result_words.push(best_cand);
        }

        result_words.join(" ")
    }
}

/// Complementary error function approximation for Gaussian p-value calculation.
fn erfc(x: f64) -> f64 {
    if x < 0.0 {
        return 2.0 - erfc(-x);
    }
    // Abramowitz and Stegun formula 7.1.26
    let p = 0.3275911;
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;

    let t = 1.0 / (1.0 + p * x);
    let poly = t * (a1 + t * (a2 + t * (a3 + t * (a4 + t * a5))));
    poly * (-x * x).exp()
}
