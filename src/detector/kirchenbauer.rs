use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Configuration for Kirchenbauer et al. (Maryland) Watermarking and Detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KirchenbauerConfig {
    /// Secret key seed
    pub key: u64,
    /// Context length k (n-gram history, default: 1)
    pub context_length: usize,
    /// Green list fraction gamma (default: 0.50)
    pub gamma: f64,
    /// Green list logit bonus delta (default: 2.0)
    pub delta: f64,
    /// Minimum z-score threshold for positive watermark detection (default: 1.645)
    pub detection_threshold: f64,
}

impl Default for KirchenbauerConfig {
    fn default() -> Self {
        Self {
            key: 133742069,
            context_length: 1,
            gamma: 0.50,
            delta: 2.0,
            detection_threshold: 1.645,
        }
    }
}

/// Detailed result of a Kirchenbauer watermark detection scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KirchenbauerDetectionResult {
    /// Whether the watermark was statistically detected (z-score >= threshold)
    pub is_watermarked: bool,
    /// Boolean flag indicating whether a statistically significant watermark signal was detected
    pub watermark_signal_detected: bool,
    /// Statistical classification: "STRONG_SIGNAL", "BORDERLINE", "NOT_SIGNIFICANT"
    pub classification: String,
    /// Standardized test statistic Z-score
    pub z_score: f64,
    /// p-value under the null hypothesis (binomial unwatermarked text with probability gamma)
    pub p_value: f64,
    /// Alpha significance level for decision boundary
    pub alpha: f64,
    /// Critical Z-score threshold for detection
    pub threshold_z: f64,
    /// Total evaluated token count
    pub total_tokens: usize,
    /// Count of tokens falling on the green list
    pub green_tokens: usize,
    /// Observed green token fraction (|S|_G / T)
    pub green_fraction: f64,
    /// Expected green token fraction under null (gamma)
    pub expected_fraction: f64,
    /// Context length k used
    pub context_length: usize,
}

/// Detector for the Kirchenbauer et al. (2023) Red-Green List watermark algorithm.
pub struct KirchenbauerDetector {
    config: KirchenbauerConfig,
}

impl KirchenbauerDetector {
    pub fn new(config: KirchenbauerConfig) -> Self {
        Self { config }
    }

    /// Checks whether a token belongs to the Green List given its preceding context.
    pub fn is_green_token(&self, context: &[&str], token: &str) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(self.config.key.to_be_bytes());
        for c in context {
            hasher.update(c.to_lowercase().as_bytes());
            hasher.update(b"|");
        }
        hasher.update(token.to_lowercase().as_bytes());
        let hash = hasher.finalize();

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&hash[0..8]);
        let val = u64::from_be_bytes(bytes);
        let float_val = (val as f64) / (u64::MAX as f64);
        float_val < self.config.gamma
    }

    /// Evaluates text and computes the Kirchenbauer Z-score and p-value.
    pub fn detect(&self, text: &str) -> KirchenbauerDetectionResult {
        let words: Vec<&str> = text
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|w| !w.is_empty())
            .collect();

        let k = self.config.context_length;
        if words.len() <= k {
            return KirchenbauerDetectionResult {
                is_watermarked: false,
                watermark_signal_detected: false,
                classification: "INSUFFICIENT_LENGTH".to_string(),
                z_score: 0.0,
                p_value: 1.0,
                alpha: 0.05,
                threshold_z: self.config.detection_threshold,
                total_tokens: words.len(),
                green_tokens: 0,
                green_fraction: 0.0,
                expected_fraction: self.config.gamma,
                context_length: k,
            };
        }

        let mut green_count = 0;
        let mut total_evaluated = 0;

        for i in k..words.len() {
            let context = &words[i - k..i];
            let token = words[i];
            if self.is_green_token(context, token) {
                green_count += 1;
            }
            total_evaluated += 1;
        }

        let gamma = self.config.gamma;
        let n = total_evaluated as f64;
        let expected_green = n * gamma;
        let std_dev = (n * gamma * (1.0 - gamma)).sqrt();

        let z_score = if std_dev > 0.0 {
            (green_count as f64 - expected_green) / std_dev
        } else {
            0.0
        };

        // One-tailed standard normal p-value
        let p_value = 0.5 * (1.0 - libm_erf(z_score / std::f64::consts::SQRT_2));

        let is_watermarked = z_score >= self.config.detection_threshold;
        let classification = if z_score >= 3.0 {
            "STRONG_SIGNAL".to_string()
        } else if z_score >= 1.645 {
            "BORDERLINE".to_string()
        } else {
            "NOT_SIGNIFICANT".to_string()
        };

        KirchenbauerDetectionResult {
            is_watermarked,
            watermark_signal_detected: is_watermarked,
            classification,
            z_score,
            p_value,
            alpha: 0.05,
            threshold_z: self.config.detection_threshold,
            total_tokens: total_evaluated,
            green_tokens: green_count,
            green_fraction: if total_evaluated > 0 {
                green_count as f64 / total_evaluated as f64
            } else {
                0.0
            },
            expected_fraction: gamma,
            context_length: k,
        }
    }
}

/// Watermarker that embeds Kirchenbauer signals into natural text by selecting green-list candidates.
pub struct KirchenbauerWatermarker {
    config: KirchenbauerConfig,
}

impl KirchenbauerWatermarker {
    pub fn new(config: KirchenbauerConfig) -> Self {
        Self { config }
    }

    /// Watermarks a sequence of words using Kirchenbauer green-list preference.
    pub fn watermark_text(
        &self,
        words: &[&str],
        candidate_generator: impl Fn(&str) -> Vec<String>,
    ) -> String {
        let k = self.config.context_length;
        if words.is_empty() {
            return String::new();
        }

        let detector = KirchenbauerDetector::new(self.config.clone());
        let mut result_words = Vec::new();

        for (i, &word) in words.iter().enumerate() {
            if i < k {
                result_words.push(word.to_string());
                continue;
            }

            let context: Vec<&str> = result_words[i - k..i].iter().map(|s| s.as_str()).collect();
            let mut candidates = candidate_generator(word);
            if !candidates.contains(&word.to_string()) {
                candidates.push(word.to_string());
            }

            // Find candidates that land on the green list
            let green_candidates: Vec<String> = candidates
                .iter()
                .filter(|c| detector.is_green_token(&context, c))
                .cloned()
                .collect();

            if let Some(best) = green_candidates.first() {
                result_words.push(best.clone());
            } else {
                result_words.push(word.to_string());
            }
        }

        result_words.join(" ")
    }
}

/// Approximation of the Gauss error function erf(x).
fn libm_erf(x: f64) -> f64 {
    // Horner approximation for erf(x) with maximum error < 1.5e-7
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let abs_x = x.abs();

    let t = 1.0 / (1.0 + p * abs_x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-abs_x * abs_x).exp();

    sign * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kirchenbauer_green_token_consistency() {
        let detector = KirchenbauerDetector::new(KirchenbauerConfig::default());
        let context = ["the"];
        let is_green_1 = detector.is_green_token(&context, "model");
        let is_green_2 = detector.is_green_token(&context, "model");
        assert_eq!(is_green_1, is_green_2);
    }

    #[test]
    fn test_kirchenbauer_detection_unwatermarked() {
        let detector = KirchenbauerDetector::new(KirchenbauerConfig::default());
        let text = "This is a completely normal natural language sentence written by a human without any watermark bias.";
        let res = detector.detect(text);
        // Short natural text should not have an extreme z-score >= 3.0
        assert!(res.z_score < 3.0);
    }
}
