use super::db::TerminologyDb;
use crate::tokenization::Token;
use std::collections::HashMap;

/// Domain detection and continuous multi-domain scoring engine
#[derive(Debug, Clone, Default)]
pub struct DomainDetector;

impl DomainDetector {
    pub fn new() -> Self {
        Self
    }

    /// Computes multi-domain continuous density scores for the given token sequence
    pub fn score_document(&self, tokens: &[Token], db: &TerminologyDb) -> HashMap<String, f64> {
        let matched_spans = db.scan_spans(tokens, None);
        let mut domain_counts: HashMap<String, usize> = HashMap::new();
        let total_words = tokens.iter().filter(|t| t.is_word()).count().max(1);

        for span in &matched_spans {
            *domain_counts.entry(span.domain.clone()).or_insert(0) += 1;
        }

        let mut scores = HashMap::new();
        let total_domain_hits: usize = domain_counts.values().sum();
        let mut max_count = 0usize;

        for (domain, &count) in &domain_counts {
            if count > max_count {
                max_count = count;
            }
            let density = (count as f64 * 15.0 / total_words as f64).min(1.0);
            let dominance = if total_domain_hits > 0 {
                count as f64 / total_domain_hits as f64
            } else {
                0.0
            };
            let combined = (density * 0.7 + dominance * 0.3).min(1.0);
            scores.insert(domain.clone(), (combined * 100.0).round() / 100.0);
        }

        // Compute general baseline score inversely related to domain specificity
        let domain_ratio = (total_domain_hits as f64 / total_words as f64).min(1.0);
        let general_score = (1.0 - (domain_ratio * 3.0).min(0.85)).max(0.10);
        scores.insert("general".to_string(), (general_score * 100.0).round() / 100.0);

        scores
    }

    /// Returns the primary detected domain (or "general" if below confidence threshold)
    pub fn primary_domain(&self, scores: &HashMap<String, f64>) -> String {
        let mut best_domain = "general".to_string();
        let mut best_score = 0.30f64; // minimum confidence threshold

        for (domain, &score) in scores {
            if domain != "general" && score > best_score {
                best_score = score;
                best_domain = domain.clone();
            }
        }

        best_domain
    }
}
