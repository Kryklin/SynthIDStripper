use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Standardized terminology relationship classification (IATE / EuroVoc taxonomy)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TermRelation {
    /// Exact conceptual and domain equivalent
    ExactEquivalent,
    /// Authoritative preferred descriptor in domain standard
    PreferredTerm,
    /// Acceptable synonym or morphological variant in domain register
    AcceptableVariant,
    /// Standardized domain abbreviation or acronym (e.g. mcap, EBITDA, NLP)
    StandardAbbreviation,
    /// Semantically associated term (NOT directly interchangeable)
    RelatedTerm,
    /// Hypernym / broader domain concept (NOT interchangeable)
    BroaderTerm,
    /// Hyponym / narrower domain concept (NOT interchangeable)
    NarrowerTerm,
    /// Specific technical term with restricted usage
    DomainSpecific,
    /// Strictly immutable regulatory, legal, or diagnostic term
    ProtectedTerm,
}

impl TermRelation {
    /// Whether this relationship permits valid lexical substitution in domain prose
    pub fn is_substitution_permitted(&self) -> bool {
        matches!(
            self,
            TermRelation::ExactEquivalent
                | TermRelation::PreferredTerm
                | TermRelation::AcceptableVariant
                | TermRelation::StandardAbbreviation
        )
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TermRelation::ExactEquivalent => "EXACT_EQUIVALENT",
            TermRelation::PreferredTerm => "PREFERRED_TERM",
            TermRelation::AcceptableVariant => "ACCEPTABLE_VARIANT",
            TermRelation::StandardAbbreviation => "STANDARD_ABBREVIATION",
            TermRelation::RelatedTerm => "RELATED_TERM",
            TermRelation::BroaderTerm => "BROADER_TERM",
            TermRelation::NarrowerTerm => "NARROWER_TERM",
            TermRelation::DomainSpecific => "DOMAIN_SPECIFIC",
            TermRelation::ProtectedTerm => "PROTECTED_TERM",
        }
    }
}

impl fmt::Display for TermRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Regulatory / protection status of a terminology entry
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TermStatus {
    /// Fully locked against generic perturbation
    DomainLocked,
    /// Protected by domain rules (only standardized variants allowed)
    Protected,
    /// Active terminological item
    #[default]
    Active,
    /// Deprecated / obsolete terminology
    Deprecated,
}

impl fmt::Display for TermStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TermStatus::DomainLocked => write!(f, "DOMAIN_LOCKED"),
            TermStatus::Protected => write!(f, "PROTECTED"),
            TermStatus::Active => write!(f, "ACTIVE"),
            TermStatus::Deprecated => write!(f, "DEPRECATED"),
        }
    }
}

/// Authoritative domain terminology entry (IATE / EuroVoc schema)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DomainTerm {
    /// Canonical surface form (e.g. "market capitalization", "apoptosis")
    pub term: String,
    /// Lemmatized normalized form (e.g. "market capitalization")
    pub lemma: String,
    /// ISO 639-1 language code (e.g. "en")
    pub language: String,
    /// Domain identifier code (e.g. "1001", "eurovoc:2406")
    pub domain_id: String,
    /// Human-readable domain name (e.g. "finance", "biomedical", "computer_science")
    pub domain_name: String,
    /// Relational taxonomy classification
    pub relation: TermRelation,
    /// Preferred standard term descriptor (if this term is a variant/abbreviation)
    pub preferred_term: Option<String>,
    /// Authorized abbreviation / acronym (if applicable)
    pub abbreviation: Option<String>,
    /// Regulatory protection status
    pub status: TermStatus,
    /// Originating terminology source (e.g. "IATE", "EuroVoc", "Custom")
    pub source: String,
    /// Version identifier of the terminology release
    pub source_version: String,
    /// Coarse part of speech tag
    pub pos: Option<String>,
}

/// Matched multiword or single-token terminology span in input text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedTermSpan {
    /// Starting token index (inclusive)
    pub start_token_idx: usize,
    /// Ending token index (exclusive)
    pub end_token_idx: usize,
    /// Surface text string in original text
    pub matched_text: String,
    /// Matched canonical term from database
    pub canonical_term: String,
    /// Domain name
    pub domain: String,
    /// Term relationship
    pub relation: TermRelation,
    /// Protection status
    pub status: TermStatus,
    /// Available authorized domain-equivalent variants
    pub acceptable_variants: Vec<String>,
    /// Whether this span is strictly locked against modification
    pub is_locked: bool,
}

/// Terminology rejection diagnostic record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminologyRejectionDiagnostic {
    pub original_term: String,
    pub candidate: String,
    pub domain: String,
    pub reason: String,
}

/// Comprehensive metrics for the domain-aware terminology layer
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminologyMetrics {
    /// Whether terminology layer was enabled for this transformation
    pub terminology_enabled: bool,
    /// Source of terminology database (e.g. "local_cache", "api", "fallback_wordnet")
    pub terminology_source: String,
    /// Version string of the active terminology dataset
    pub terminology_version: String,
    /// SHA-256 cryptographic hash of the terminology database
    pub terminology_hash: String,
    /// Continuous domain classification scores (e.g. {"finance": 0.84, "economics": 0.61})
    pub detected_domains: HashMap<String, f64>,

    // --- Granular Terminology Denominators ---
    /// Total domain terminology terms/spans identified in the text
    pub domain_terms_detected: usize,
    /// Total domain terms that were content words eligible for transformation
    pub domain_terms_eligible: usize,
    /// Total domain terms locked or protected against generic modification
    pub domain_terms_protected: usize,
    /// Total domain terms having at least one authorized domain variant/abbreviation
    pub domain_terms_with_variants: usize,
    /// Total domain terms actually replaced with an authorized variant
    pub domain_terms_replaced: usize,

    /// Number of matched domain terminology spans (alias of domain_terms_detected)
    pub matched_terms_count: usize,
    /// Details of matched terminology spans
    pub matched_spans: Vec<MatchedTermSpan>,
    /// Total protected terminology items identified
    pub protected_terms_count: usize,
    /// Total domain-locked terminology items identified
    pub domain_locked_terms_count: usize,
    /// Number of terminology candidate substitutions evaluated
    pub terminology_candidates_considered: usize,
    /// Number of terminology candidate substitutions rejected due to domain/relation mismatch
    pub terminology_candidates_rejected: usize,
    /// Number of successful domain-authorized substitutions applied
    pub terminology_replacements: usize,
    /// Lexical turnover percentage on general English content words
    pub general_lexical_turnover_pct: f64,
    /// Lexical turnover percentage on domain-specific terminology words
    pub domain_lexical_turnover_pct: f64,
    /// Diagnostic records of rejected candidates
    pub diagnostics: Vec<TerminologyRejectionDiagnostic>,
}

/// Configuration for the domain-aware terminology layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminologyConfig {
    /// Enable authoritative terminology validation layer
    pub enabled: bool,
    /// Data source ("local" or "api")
    pub source: String,
    /// Path to external terminology JSON database (optional)
    pub db_path: Option<String>,
    /// Target domain filter (e.g. "finance", "biomedical", "computer_science")
    pub domain_filter: Option<String>,
    /// Strict mode: protect all identified domain terms from perturbation
    pub protect_domain_terms: bool,
    /// Enable detailed diagnostic logging
    pub debug: bool,
}

impl Default for TerminologyConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            source: "local".to_string(),
            db_path: None,
            domain_filter: None,
            protect_domain_terms: false,
            debug: false,
        }
    }
}
