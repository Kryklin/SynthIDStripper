use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Part of Speech tags (Penn Treebank compatible).
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PosTag {
    // Nouns
    NN,   // Noun, singular or mass
    NNS,  // Noun, plural
    NNP,  // Proper noun, singular
    NNPS, // Proper noun, plural

    // Verbs
    VB,  // Verb, base form
    VBD, // Verb, past tense
    VBG, // Verb, gerund or present participle
    VBN, // Verb, past participle
    VBP, // Verb, non-3rd person singular present
    VBZ, // Verb, 3rd person singular present
    MD,  // Modal auxiliary (can, could, will, would, etc.)

    // Adjectives
    JJ,  // Adjective
    JJR, // Adjective, comparative
    JJS, // Adjective, superlative

    // Adverbs
    RB,  // Adverb
    RBR, // Adverb, comparative
    RBS, // Adverb, superlative

    // Closed-class / Functional / Structural
    IN,       // Preposition or subordinating conjunction
    DT,       // Determiner
    CC,       // Coordinating conjunction
    CD,       // Cardinal number
    PRP,      // Personal pronoun
    PRP_POSS, // Possessive pronoun
    POS,      // Possessive ending ('s)
    TO,       // to
    UH,       // Interjection
    EX,       // Existential there
    RP,       // Particle
    SYM,      // Symbol
    PUNCT,    // Punctuation
    UNKNOWN,
}

impl PosTag {
    /// Returns the coarse grammatical category.
    pub fn coarse_pos(&self) -> CoarsePos {
        match self {
            PosTag::NN | PosTag::NNS => CoarsePos::Noun,
            PosTag::NNP | PosTag::NNPS => CoarsePos::ProperNoun,
            PosTag::VB | PosTag::VBD | PosTag::VBG | PosTag::VBN | PosTag::VBP | PosTag::VBZ => {
                CoarsePos::Verb
            }
            PosTag::JJ | PosTag::JJR | PosTag::JJS => CoarsePos::Adjective,
            PosTag::RB | PosTag::RBR | PosTag::RBS => CoarsePos::Adverb,
            _ => CoarsePos::Other,
        }
    }

    /// Whether this POS is generally an open-class content word eligible for lexical substitution.
    pub fn is_content_word(&self) -> bool {
        matches!(
            self,
            PosTag::NN
                | PosTag::NNS
                | PosTag::VB
                | PosTag::VBD
                | PosTag::VBG
                | PosTag::VBN
                | PosTag::VBP
                | PosTag::VBZ
                | PosTag::JJ
                | PosTag::JJR
                | PosTag::JJS
                | PosTag::RB
                | PosTag::RBR
                | PosTag::RBS
        )
    }

    /// Display string for Penn Treebank tag
    pub fn as_str(&self) -> &'static str {
        match self {
            PosTag::NN => "NN",
            PosTag::NNS => "NNS",
            PosTag::NNP => "NNP",
            PosTag::NNPS => "NNPS",
            PosTag::VB => "VB",
            PosTag::VBD => "VBD",
            PosTag::VBG => "VBG",
            PosTag::VBN => "VBN",
            PosTag::VBP => "VBP",
            PosTag::VBZ => "VBZ",
            PosTag::MD => "MD",
            PosTag::JJ => "JJ",
            PosTag::JJR => "JJR",
            PosTag::JJS => "JJS",
            PosTag::RB => "RB",
            PosTag::RBR => "RBR",
            PosTag::RBS => "RBS",
            PosTag::IN => "IN",
            PosTag::DT => "DT",
            PosTag::CC => "CC",
            PosTag::CD => "CD",
            PosTag::PRP => "PRP",
            PosTag::PRP_POSS => "PRP$",
            PosTag::POS => "POS",
            PosTag::TO => "TO",
            PosTag::UH => "UH",
            PosTag::EX => "EX",
            PosTag::RP => "RP",
            PosTag::SYM => "SYM",
            PosTag::PUNCT => "PUNCT",
            PosTag::UNKNOWN => "UNK",
        }
    }
}

impl fmt::Display for PosTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Coarse grammatical categories for lookup and synset organization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CoarsePos {
    Noun,
    Verb,
    Adjective,
    Adverb,
    ProperNoun,
    Other,
}

impl fmt::Display for CoarsePos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoarsePos::Noun => write!(f, "noun"),
            CoarsePos::Verb => write!(f, "verb"),
            CoarsePos::Adjective => write!(f, "adjective"),
            CoarsePos::Adverb => write!(f, "adverb"),
            CoarsePos::ProperNoun => write!(f, "proper_noun"),
            CoarsePos::Other => write!(f, "other"),
        }
    }
}

/// Distribution sampling strategies for candidate selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
pub enum DistributionMode {
    /// Equal-weight sampling among vetted semantic alternatives
    Neutral,
    /// Zipf-weighted human baseline distribution (prefers natural colloquial frequency)
    Human,
    /// Model-favored token distribution (favors high-register LLM prior tokens)
    Ai,
    /// Uniform pseudo-random sampling
    Random,
}

impl fmt::Display for DistributionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DistributionMode::Neutral => write!(f, "neutral"),
            DistributionMode::Human => write!(f, "human"),
            DistributionMode::Ai => write!(f, "ai"),
            DistributionMode::Random => write!(f, "random"),
        }
    }
}

/// Advanced selection strategy policies for transformation efficiency optimization.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum, Default,
)]
#[serde(rename_all = "kebab-case")]
pub enum SelectionStrategy {
    /// Strategy A: Existing Baseline (Zipf human frequency-weighted)
    #[default]
    Baseline,
    /// Strategy B: Uniform Valid Sampling among all vetted candidates passing guardrails
    Uniform,
    /// Strategy C: Context-Neutral Sampling (lexical diversity + morphological form balance)
    ContextNeutral,
    /// Strategy D: Efficiency-Guided Sampling (minimizes local detector g-statistic per semantic cost)
    EfficiencyGuided,
    /// Strategy E: Conservative Efficiency-Guided (stricter semantic confidence floor c >= 0.65)
    ConservativeEfficiency,
}

impl fmt::Display for SelectionStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelectionStrategy::Baseline => write!(f, "baseline"),
            SelectionStrategy::Uniform => write!(f, "uniform"),
            SelectionStrategy::ContextNeutral => write!(f, "context-neutral"),
            SelectionStrategy::EfficiencyGuided => write!(f, "efficiency-guided"),
            SelectionStrategy::ConservativeEfficiency => write!(f, "conservative-efficiency"),
        }
    }
}

/// Per-transformation audit record tracking local detector effects and linguistic disruption costs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationAuditRecord {
    pub original_token: String,
    pub replacement_token: String,
    pub lemma: String,
    pub pos: PosTag,
    pub domain_classification: String,
    pub is_protected_terminology_span: bool,
    pub semantic_confidence: f64,
    pub candidate_pool_size: usize,
    pub selected_candidate_rank: usize,
    pub local_context_before: String,
    pub local_context_after: String,
    pub detector_contribution_before: f64,
    pub detector_contribution_after: f64,
    pub local_delta_g: f64,
    pub local_contextual_disruption: f64,
    pub edit_distance: usize,
    pub is_accepted: bool,
    pub detector_effect_per_replacement: f64,
    pub detector_effect_per_edit: f64,
    pub detector_effect_per_semantic_cost: f64,
}

/// Distinct classification of transformation layers applied to the text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransformationClass {
    /// Semantic lexical substitution (POS-preserving synonym replacement)
    Lexical,
    /// Syntactic restructuring (clause order inversion, coordination/subordination)
    Syntax,
    /// Repetitive cadence & template variation
    Cadence,
    /// Controlled function-word & discourse connector alternation
    FunctionWord,
    /// Simulated human typing noise (keyboard-proximity perturbations)
    TypingNoise,
}

impl fmt::Display for TransformationClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransformationClass::Lexical => write!(f, "lexical"),
            TransformationClass::Syntax => write!(f, "syntax"),
            TransformationClass::Cadence => write!(f, "cadence"),
            TransformationClass::FunctionWord => write!(f, "function_word"),
            TransformationClass::TypingNoise => write!(f, "typing_noise"),
        }
    }
}

/// Imported result from an external black-box AI classifier (e.g. GPTZero).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectorImportRecord {
    pub detector: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub ai_probability: Option<f64>,
    #[serde(default)]
    pub human_probability: Option<f64>,
    #[serde(default)]
    pub mixed_probability: Option<f64>,
    #[serde(default)]
    pub classification: Option<String>,
}

/// Detailed recorded association between external classifier observations and all transformation metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectorImpactAnalysis {
    pub detector_name: String,
    pub detector_version: String,
    pub classification: Option<String>,
    pub ai_probability: Option<f64>,
    pub human_probability: Option<f64>,
    pub mixed_probability: Option<f64>,

    // Multi-metric associations
    pub total_jsd: f64,
    pub content_words_jsd: f64,
    pub nouns_jsd: f64,
    pub verbs_jsd: f64,
    pub adjectives_jsd: f64,
    pub adverbs_jsd: f64,
    pub function_words_jsd: f64,
    pub lexical_turnover: f64,
    pub semantic_confidence: f64,
}

/// Standard human typing error classes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypingErrorClass {
    Substitution,
    Transposition,
    Omission,
    Insertion,
    Duplication,
    Temporal,
}

/// Linguistic domain classification for domain-constrained lexical regions
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum, Default,
)]
#[serde(rename_all = "snake_case")]
pub enum Domain {
    #[default]
    Auto,
    General,
    Finance,
    Biomedical,
    ComputerScience,
    Legal,
    Culinary,
    CreativeArts,
    Science,
    Politics,
    Sports,
    Education,
    PhilosophyPsychology,
    Military,
    JournalismMedia,
    TravelTourism,
    AviationAerospace,
    ArchitectureConstruction,
    GamingEsports,
    AgricultureBotany,
    FashionTextiles,
    TheologyReligion,
    AudioEngineering,
    MaritimeNautical,
    AutomotiveMotorsport,
    FilmTelevision,
    LinguisticsPhilology,
    RealEstateProperty,
    LogisticsSupplyChain,
    FitnessKinesiology,
    OccultAstrology,
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Domain::Auto => write!(f, "auto"),
            Domain::General => write!(f, "general"),
            Domain::Finance => write!(f, "finance"),
            Domain::Biomedical => write!(f, "biomedical"),
            Domain::ComputerScience => write!(f, "computer_science"),
            Domain::Legal => write!(f, "legal"),
            Domain::Culinary => write!(f, "culinary"),
            Domain::CreativeArts => write!(f, "creative_arts"),
            Domain::Science => write!(f, "science"),
            Domain::Politics => write!(f, "politics"),
            Domain::Sports => write!(f, "sports"),
            Domain::Education => write!(f, "education"),
            Domain::PhilosophyPsychology => write!(f, "philosophy_psychology"),
            Domain::Military => write!(f, "military"),
            Domain::JournalismMedia => write!(f, "journalism_media"),
            Domain::TravelTourism => write!(f, "travel_tourism"),
            Domain::AviationAerospace => write!(f, "aviation_aerospace"),
            Domain::ArchitectureConstruction => write!(f, "architecture_construction"),
            Domain::GamingEsports => write!(f, "gaming_esports"),
            Domain::AgricultureBotany => write!(f, "agriculture_botany"),
            Domain::FashionTextiles => write!(f, "fashion_textiles"),
            Domain::TheologyReligion => write!(f, "theology_religion"),
            Domain::AudioEngineering => write!(f, "audio_engineering"),
            Domain::MaritimeNautical => write!(f, "maritime_nautical"),
            Domain::AutomotiveMotorsport => write!(f, "automotive_motorsport"),
            Domain::FilmTelevision => write!(f, "film_television"),
            Domain::LinguisticsPhilology => write!(f, "linguistics_philology"),
            Domain::RealEstateProperty => write!(f, "real_estate_property"),
            Domain::LogisticsSupplyChain => write!(f, "logistics_supply_chain"),
            Domain::FitnessKinesiology => write!(f, "fitness_kinesiology"),
            Domain::OccultAstrology => write!(f, "occult_astrology"),
        }
    }
}

/// Lexical classification defining perturbation permissions per domain slot
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LexicalClass {
    General,
    DomainLocked,
    DomainPreferred,
    TechnicalTerm,
    ProperEntity,
}

impl TypingErrorClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            TypingErrorClass::Substitution => "substitution",
            TypingErrorClass::Transposition => "transposition",
            TypingErrorClass::Omission => "omission",
            TypingErrorClass::Insertion => "insertion",
            TypingErrorClass::Duplication => "duplication",
            TypingErrorClass::Temporal => "temporal",
        }
    }
}

/// Recorded individual character-level typing mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingMutationRecord {
    pub token_index: usize,
    pub original_word: String,
    pub mutated_word: String,
    pub error_class: TypingErrorClass,
    pub char_position: usize,
    pub original_char: String,
    pub mutated_char: String,
    pub keyboard_distance: f64,
}

/// Configuration for the human typing noise simulation layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingNoiseConfig {
    /// Noise rate / probability of applying an error to an eligible token (0.0 to 1.0)
    pub rate: f64,
    /// Deterministic RNG seed for typing noise reproducibility
    pub seed: Option<u64>,
    /// Allowed error classes to sample from
    pub allowed_errors: Vec<TypingErrorClass>,
    /// Relative weights for sampling each error class
    pub error_weights: HashMap<TypingErrorClass, f64>,
}

impl Default for TypingNoiseConfig {
    fn default() -> Self {
        let mut error_weights = HashMap::new();
        error_weights.insert(TypingErrorClass::Substitution, 0.35);
        error_weights.insert(TypingErrorClass::Transposition, 0.20);
        error_weights.insert(TypingErrorClass::Omission, 0.15);
        error_weights.insert(TypingErrorClass::Insertion, 0.15);
        error_weights.insert(TypingErrorClass::Duplication, 0.05);
        error_weights.insert(TypingErrorClass::Temporal, 0.10);

        Self {
            rate: 0.0,
            seed: None,
            allowed_errors: vec![
                TypingErrorClass::Substitution,
                TypingErrorClass::Transposition,
                TypingErrorClass::Omission,
                TypingErrorClass::Insertion,
                TypingErrorClass::Duplication,
                TypingErrorClass::Temporal,
            ],
            error_weights,
        }
    }
}

/// Comprehensive metrics for human typing noise layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingNoiseMetrics {
    pub rate_configured: f64,
    pub eligible_tokens: usize,
    pub corrupted_tokens: usize,
    pub actual_error_rate: f64,
    pub error_class_counts: HashMap<String, usize>,
    pub mean_keyboard_distance: f64,
    pub character_edit_distance: usize,
    pub word_edit_distance: usize,
    pub mutations: Vec<TypingMutationRecord>,
    pub seed_used: Option<u64>,
}

/// Engine configuration with modular transformation class toggles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripperConfig {
    /// Distribution sampling mode
    pub mode: DistributionMode,
    /// Probability of attempting to replace an eligible word (0.0 to 1.0)
    pub replacement_probability: f64,
    /// Minimum confidence threshold for contextual compatibility (0.0 to 1.0)
    pub min_confidence: f64,
    /// Preserve named entities and proper nouns
    pub preserve_named_entities: bool,
    /// Preserve technical terms, acronyms, and code keywords
    pub preserve_technical_terms: bool,
    /// Deterministic RNG seed (optional)
    pub seed: Option<u64>,
    /// Temperature for softmax/probability shaping (higher = more uniform, lower = peaked)
    pub temperature: f64,
    /// Maximum candidates considered per word
    pub max_candidates_per_word: usize,
    /// Use external computational linguistics Thesaurus API (Datamuse)
    pub use_thesaurus_api: bool,
    /// Allow re-replacing already transformed tokens in subsequent passes
    pub allow_re_replacement: bool,
    /// Linguistic domain mode for domain-locked registers
    pub domain: Domain,
    /// Selection strategy policy for candidate selection
    pub selection_strategy: SelectionStrategy,

    // --- Modular Transformation Flags ---
    /// Enable semantic lexical substitution
    pub enable_lexical: bool,
    /// Enable conservative syntactic restructuring (clause coordination/subordination)
    pub enable_syntax: bool,
    /// Enable repetitive cadence & template variation
    pub enable_cadence: bool,
    /// Enable controlled function-word and discourse connector alternatives
    pub enable_function_words: bool,
    /// Configuration for simulated human typing noise (None = disabled / rate 0)
    pub typing_noise_config: Option<TypingNoiseConfig>,

    /// Configuration for authoritative domain terminology layer (IATE / EuroVoc)
    pub terminology_config: Option<crate::terminology::TerminologyConfig>,

    /// Configuration for token-position sensitivity heatmap and equal-budget sampling
    pub sensitivity_config: Option<SensitivityConfig>,

    /// Optional external detector import data
    pub detector_import: Option<DetectorImportRecord>,
}

impl Default for StripperConfig {
    fn default() -> Self {
        Self {
            mode: DistributionMode::Human,
            replacement_probability: 0.30,
            min_confidence: 0.55,
            preserve_named_entities: true,
            preserve_technical_terms: true,
            seed: None,
            temperature: 1.0,
            max_candidates_per_word: 10,
            use_thesaurus_api: false,
            allow_re_replacement: false,
            domain: Domain::Auto,
            selection_strategy: SelectionStrategy::ConservativeEfficiency,
            enable_lexical: true,
            enable_syntax: false,
            enable_cadence: false,
            enable_function_words: false,
            typing_noise_config: None,
            terminology_config: None,
            sensitivity_config: None,
            detector_import: None,
        }
    }
}

/// Information about an alternative candidate considered during selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateScore {
    pub lemma: String,
    pub inflected: String,
    pub semantic_score: f64,
    pub frequency_weight: f64,
    pub final_probability: f64,
}

/// Details of a single word replacement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordTransform {
    pub token_index: usize,
    pub original: String,
    pub replacement: String,
    pub original_lemma: String,
    pub replacement_lemma: String,
    pub pos: PosTag,
    pub transformation_class: TransformationClass,
    pub sense_gloss: String,
    pub confidence: f64,
    pub candidates_considered: Vec<CandidateScore>,
    pub pass_number: usize,
}

/// Reason why a candidate or word was rejected from transformation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RejectionReason {
    NamedEntityOrProperNoun,
    ClosedClassOrStopword,
    TechnicalOrProtectedTerm,
    DomainLockedTerm { domain: String },
    CrossDomainMismatch { domain: String, candidate: String },
    DomainTerminologyMismatch { domain: String, reason: String },
    InvalidTermRelation { term: String, relation: String },
    NoSynsetFound,
    LowContextConfidence { confidence: f64, threshold: f64 },
    SemanticDivergence { score: f64 },
    GrammaticalMismatch { expected: String, got: String },
    InflectionFailure { lemma: String, pos: String },
    IdenticalToOriginal,
    AntonymOrOpposite,
    ProbabilitySkipped,
    AlreadyTransformed,
    ValencyMismatch { complement: String },
    CollocationMismatch { partner: String },
}

impl fmt::Display for RejectionReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RejectionReason::NamedEntityOrProperNoun => {
                write!(f, "Protected named entity / proper noun")
            }
            RejectionReason::ClosedClassOrStopword => {
                write!(f, "Closed-class / function word / stopword")
            }
            RejectionReason::TechnicalOrProtectedTerm => {
                write!(f, "Protected technical / code / format term")
            }
            RejectionReason::DomainLockedTerm { domain } => {
                write!(f, "Protected immutable domain term ({})", domain)
            }
            RejectionReason::CrossDomainMismatch { domain, candidate } => {
                write!(
                    f,
                    "Candidate '{}' violates domain register ({})",
                    candidate, domain
                )
            }
            RejectionReason::DomainTerminologyMismatch { domain, reason } => {
                write!(f, "Terminology mismatch in domain {}: {}", domain, reason)
            }
            RejectionReason::InvalidTermRelation { term, relation } => {
                write!(
                    f,
                    "Term '{}' has non-substitutable relation: {}",
                    term, relation
                )
            }
            RejectionReason::NoSynsetFound => write!(f, "No viable synonym synsets found"),
            RejectionReason::LowContextConfidence {
                confidence,
                threshold,
            } => {
                write!(
                    f,
                    "Low context confidence ({:.3} < {:.3})",
                    confidence, threshold
                )
            }
            RejectionReason::SemanticDivergence { score } => {
                write!(f, "Semantic divergence score ({:.3}) exceeds limit", score)
            }
            RejectionReason::GrammaticalMismatch { expected, got } => {
                write!(
                    f,
                    "Grammatical mismatch (expected {}, got {})",
                    expected, got
                )
            }
            RejectionReason::InflectionFailure { lemma, pos } => {
                write!(f, "Failed to inflect lemma '{}' for POS {}", lemma, pos)
            }
            RejectionReason::IdenticalToOriginal => {
                write!(f, "Candidate is identical to original word")
            }
            RejectionReason::AntonymOrOpposite => {
                write!(f, "Candidate is an antonym or semantic opposite")
            }
            RejectionReason::ProbabilitySkipped => {
                write!(f, "Skipped by replacement probability setting")
            }
            RejectionReason::AlreadyTransformed => write!(
                f,
                "Token already transformed in previous pass (fixed-point protection)"
            ),
            RejectionReason::ValencyMismatch { complement } => write!(
                f,
                "Candidate violates syntactic valency / complement structure ('{}')",
                complement
            ),
            RejectionReason::CollocationMismatch { partner } => write!(
                f,
                "Candidate forms unnatural collocation with '{}'",
                partner
            ),
        }
    }
}

/// Log of a rejected candidate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectionRecord {
    pub word: String,
    pub candidate: Option<String>,
    pub pos: PosTag,
    pub reason: RejectionReason,
}

/// Metrics tracking stability across iterative transformation passes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassStabilityReport {
    pub total_passes: usize,
    pub pass_turnovers: Vec<f64>,
    pub pass_jsd: Vec<f64>,
    pub pass_replacements: Vec<usize>,
    pub convergence_pass: Option<usize>,
    pub fixed_point_reached: bool,
}

/// Fine-grained Jensen-Shannon Divergence breakdown across grammatical categories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryJsdMetrics {
    /// Complete token vocabulary JSD (in bits [0, 1])
    pub total_jsd: f64,
    /// Content words only JSD (Nouns, Verbs, Adjectives, Adverbs)
    pub content_words_jsd: f64,
    /// Nouns only JSD
    pub nouns_jsd: f64,
    /// Verbs only JSD
    pub verbs_jsd: f64,
    /// Adjectives only JSD
    pub adjectives_jsd: f64,
    /// Adverbs only JSD
    pub adverbs_jsd: f64,
    /// Function / closed-class words JSD
    pub function_words_jsd: f64,
}

/// Comprehensive structural and syntactic metrics of the text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralMetrics {
    /// Total sentence count
    pub sentence_count: usize,
    /// Mean sentence length in words
    pub mean_sentence_length: f64,
    /// Sentence length variance (sigma^2) across sentences
    pub sentence_length_variance: f64,
    /// POS distribution breakdown (count per POS tag)
    pub pos_distribution: HashMap<String, usize>,
    /// Repeated sentence openings count (identical first 2 words)
    pub repeated_sentence_openings: usize,
    /// Repeated syntactic template pattern count
    pub repeated_syntactic_patterns: usize,
    /// Frequency count of conjunctions (CC, IN)
    pub conjunction_distribution: HashMap<String, usize>,
    /// Frequency count of adjectives (JJ, JJR, JJS)
    pub adjective_distribution: HashMap<String, usize>,
    /// Content word count
    pub content_word_count: usize,
    /// Function word count
    pub function_word_count: usize,
    /// Content-to-function word ratio
    pub content_function_ratio: f64,
}

/// Comprehensive Transformation Report with reproducibility hashes and multi-dimensional metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformReport {
    /// SHA-256 hash of the input text for experiment reproducibility
    pub input_sha256: String,
    /// SHA-256 hash of the transformed output text
    pub output_sha256: String,
    /// Random seed used
    pub seed_used: Option<u64>,
    /// Configured distribution mode used
    pub mode_used: DistributionMode,
    /// Enabled transformation classes
    pub enabled_classes: Vec<TransformationClass>,
    /// Minimum confidence threshold
    pub confidence_threshold: f64,
    /// Detected or configured domain of the text
    pub detected_domain: String,
    /// Effective replacement entropy H_eff in bits/word
    pub effective_replacement_entropy: f64,
    /// Count of domain-locked immutable tokens identified
    pub domain_locked_tokens_count: usize,

    /// Total number of tokens in the input text
    pub total_tokens: usize,
    /// Total word tokens (excluding punctuation and whitespace)
    pub total_words: usize,
    /// Number of words eligible for transformation (content words not protected)
    pub eligible_words: usize,
    /// Number of words actually transformed
    pub replaced_words: usize,
    /// Replacement rate percentage (replaced / eligible)
    pub replacement_rate: f64,
    /// Breakdown of replacements by transformation class
    pub replacements_by_class: HashMap<String, usize>,

    /// Unique original content tokens
    pub unique_original_tokens: usize,
    /// Unique original tokens that were modified
    pub unique_original_tokens_changed: usize,
    /// Lexical turnover (unique_changed / unique_original)
    pub lexical_turnover: f64,

    /// Multi-category Jensen-Shannon Divergence metrics (in bits [0, 1])
    pub jsd_metrics: CategoryJsdMetrics,

    /// Input text structural metrics
    pub original_structural_metrics: StructuralMetrics,
    /// Transformed text structural metrics
    pub transformed_structural_metrics: StructuralMetrics,

    /// Multi-pass resampling stability report
    pub stability_report: Option<PassStabilityReport>,
    /// Breakdown of grammatical categories affected
    pub categories_affected: HashMap<String, usize>,
    /// Mean confidence score of applied replacements
    pub mean_confidence: f64,
    /// Detailed record of each applied transformation
    pub replacements: Vec<WordTransform>,
    /// Candidates and words rejected due to grammatical or semantic incompatibility
    pub rejected_candidates: Vec<RejectionRecord>,

    /// Human typing noise metrics (if enabled)
    pub typing_metrics: Option<TypingNoiseMetrics>,

    /// Domain terminology metrics (if terminology layer enabled)
    pub terminology_metrics: Option<crate::terminology::TerminologyMetrics>,

    /// External detector correlation and impact analysis (if detector data provided)
    pub detector_impact: Option<DetectorImpactAnalysis>,
    /// Per-transformation audit records tracking local detector effects and efficiency
    pub audit_records: Vec<TransformationAuditRecord>,
}

/// Result of the transformation containing transformed text and comprehensive report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformResult {
    pub transformed_text: String,
    pub report: TransformReport,
}

/// Experiment tracking metadata for automated batch evaluation and response curves
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentMetadata {
    /// Unique experiment ID (e.g. "lexical_015_seed_12345")
    pub id: String,
    /// SHA-256 hash of the unperturbed parent baseline text
    pub parent_input_sha256: String,
    /// Deterministic random seed used
    pub random_seed: Option<u64>,
    /// Replicate number in batch sequence
    pub replicate: usize,
    /// Whether domain terminology layer was active
    #[serde(default)]
    pub terminology_enabled: bool,
    /// Source of terminology database (e.g. "local", "api")
    #[serde(default)]
    pub terminology_source: String,
    /// Terminology database version
    #[serde(default)]
    pub terminology_database_version: String,
    /// SHA-256 cryptographic hash of the terminology database
    #[serde(default)]
    pub terminology_hash: String,
}

/// Complete empirical record written to disk for auditing and research analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpiricalLog {
    /// Experiment tracking block
    pub experiment: ExperimentMetadata,
    pub timestamp_utc: String,
    pub input_sha256: String,
    pub output_sha256: String,
    pub seed: Option<u64>,
    pub distribution_mode: String,
    pub detected_domain: String,
    pub effective_replacement_entropy: f64,
    pub domain_locked_tokens_count: usize,
    pub active_layers: Vec<String>,
    pub total_tokens: usize,
    pub total_words: usize,
    pub eligible_words: usize,
    pub replaced_words: usize,
    pub replacement_rate_pct: f64,
    pub lexical_turnover_pct: f64,
    pub unique_content_tokens_changed: usize,
    pub unique_content_tokens_total: usize,
    pub mean_semantic_confidence: f64,
    pub jsd_metrics: CategoryJsdMetrics,
    pub original_structural_metrics: StructuralMetrics,
    pub transformed_structural_metrics: StructuralMetrics,
    pub replacements_by_layer: HashMap<String, usize>,
    pub typing_noise_metrics: Option<TypingNoiseMetrics>,
    pub terminology_metrics: Option<crate::terminology::TerminologyMetrics>,
    pub synthid_verification: Option<crate::detector::SynthIdDetectionResult>,
    pub transformations: Vec<WordTransform>,
    pub audit_records: Vec<TransformationAuditRecord>,
}

/// Controlled sampling policy for equal-budget sensitivity experiments
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum, Default,
)]
#[serde(rename_all = "snake_case")]
pub enum SensitivitySamplingPolicy {
    #[default]
    Uniform,
    HighSensitivity,
    LowSensitivity,
    ShuffledSpatial,
    CompositeModel,
    ShuffledComposite,
    LowComposite,
}

impl fmt::Display for SensitivitySamplingPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SensitivitySamplingPolicy::Uniform => write!(f, "uniform"),
            SensitivitySamplingPolicy::HighSensitivity => write!(f, "high_sensitivity"),
            SensitivitySamplingPolicy::LowSensitivity => write!(f, "low_sensitivity"),
            SensitivitySamplingPolicy::ShuffledSpatial => write!(f, "shuffled_spatial"),
            SensitivitySamplingPolicy::CompositeModel => write!(f, "composite_model"),
            SensitivitySamplingPolicy::ShuffledComposite => write!(f, "shuffled_composite"),
            SensitivitySamplingPolicy::LowComposite => write!(f, "low_composite"),
        }
    }
}

/// Linear composite sensitivity model weights and scaling parameters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompositeModelWeights {
    pub model_hash: String,
    pub intercept: f64,
    pub feature_names: Vec<String>,
    pub weights: Vec<f64>,
    pub feature_means: Vec<f64>,
    pub feature_stds: Vec<f64>,
}

/// Sensitivity configuration for transformation engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityConfig {
    pub policy: SensitivitySamplingPolicy,
    pub heatmap_path: Option<String>,
    pub composite_model_path: Option<String>,
    pub edit_budget: Option<usize>,
    pub min_observations: usize,
}

impl Default for SensitivityConfig {
    fn default() -> Self {
        Self {
            policy: SensitivitySamplingPolicy::Uniform,
            heatmap_path: None,
            composite_model_path: None,
            edit_budget: None,
            min_observations: 3,
        }
    }
}

/// Status of an empirical observation at a token position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SensitivityObservationStatus {
    Valid,
    UnderSampled,
}

/// Individual token-level empirical sensitivity record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSensitivityRecord {
    pub document_id: String,
    pub token_index: usize,
    pub sentence_index: usize,
    pub paragraph_index: usize,
    pub token_text: String,
    pub lemma: String,
    pub pos: PosTag,
    pub domain_class: String,
    pub character_offset: usize,
    pub normalized_position: f64,
    pub mean_delta_z: f64,
    pub median_delta_z: f64,
    pub std_delta_z: f64,
    pub mean_abs_delta_z: f64,
    pub clean_transition_rate: f64,
    pub standard_error: f64,
    pub ci95_low: f64,
    pub ci95_high: f64,
    pub n_trials: usize,
    pub status: SensitivityObservationStatus,
}

/// Positional or category bin aggregated statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionalBinStats {
    pub bin_id: String,
    pub bin_index: usize,
    pub start_normalized: f64,
    pub end_normalized: f64,
    pub token_count: usize,
    pub mean_delta_z: f64,
    pub median_delta_z: f64,
    pub std_delta_z: f64,
    pub mean_abs_delta_z: f64,
    pub clean_transition_rate: f64,
    pub standard_error: f64,
    pub ci95_low: f64,
    pub ci95_high: f64,
    pub n_trials: usize,
    pub status: SensitivityObservationStatus,
}

/// Frozen empirical token-position sensitivity heatmap model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityHeatmapModel {
    pub source_corpus_hash: String,
    pub source_document_ids: Vec<String>,
    pub random_seed: u64,
    pub configuration: String,
    pub detector_configuration: String,
    pub creation_timestamp: String,
    pub heatmap_hash: String,
    pub min_observations: usize,
    pub total_tokens_profiled: usize,
    pub total_trials_executed: usize,
    pub position_bins_10: Vec<PositionalBinStats>,
    pub position_bins_20: Vec<PositionalBinStats>,
    pub position_bins_50: Vec<PositionalBinStats>,
    pub sentence_position_bins: Vec<PositionalBinStats>,
    pub paragraph_position_bins: Vec<PositionalBinStats>,
    pub pos_stats: HashMap<String, PositionalBinStats>,
    pub domain_stats: HashMap<String, PositionalBinStats>,
    pub token_records: Vec<TokenSensitivityRecord>,
}
