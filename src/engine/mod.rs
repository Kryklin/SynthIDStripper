pub mod ablation;
use crate::lexicon::{DomainClassifier, SynsetDb, ThesaurusApiClient};
use crate::morphology::Lemmatizer;
use crate::ner::EntityClassifier;
use crate::pos::PosTagger;
use crate::sampler::DistributionSampler;
use crate::syntax::{CadenceModulator, FunctionWordModulator, SyntaxRestructurer};
use crate::terminology::{
    DomainDetector, MatchedTermSpan, TermRelation, TermStatus, TerminologyApiClient,
    TerminologyDb, TerminologyMetrics, TerminologyRejectionDiagnostic,
};
use crate::tokenization::Tokenizer;
use crate::analysis::{SensitivityAnalyzer, SensitivityHeatmapModel};
use crate::types::{
    CandidateScore, CategoryJsdMetrics, CoarsePos, CompositeModelWeights, DetectorImpactAnalysis,
    Domain, LexicalClass, PassStabilityReport, RejectionReason, RejectionRecord,
    SensitivitySamplingPolicy, StripperConfig, StructuralMetrics, TransformReport, TransformResult,
    TransformationAuditRecord, TransformationClass, WordTransform,
};
use crate::typing::TypingNoiseEngine;
use crate::wsd::SenseDisambiguator;
pub use ablation::{AblationMatrixReport, AblationRunEntry};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

pub struct LexiconStripper {
    tokenizer: Tokenizer,
    pos_tagger: PosTagger,
    lemmatizer: Lemmatizer,
    synset_db: SynsetDb,
    thesaurus_client: ThesaurusApiClient,
    domain_classifier: DomainClassifier,
    pub terminology_db: TerminologyDb,
    pub domain_detector: DomainDetector,
    pub terminology_client: TerminologyApiClient,
    disambiguator: SenseDisambiguator,
    entity_classifier: EntityClassifier,
    sampler: DistributionSampler,
    syntax_restructurer: SyntaxRestructurer,
    cadence_modulator: CadenceModulator,
    function_word_modulator: FunctionWordModulator,
    typing_engine: TypingNoiseEngine,
}

impl Default for LexiconStripper {
    fn default() -> Self {
        Self::new()
    }
}

impl LexiconStripper {
    pub fn new() -> Self {
        Self {
            tokenizer: Tokenizer::new(),
            pos_tagger: PosTagger::new(),
            lemmatizer: Lemmatizer::new(),
            synset_db: SynsetDb::new(),
            thesaurus_client: ThesaurusApiClient::new(true),
            domain_classifier: DomainClassifier::new(),
            terminology_db: TerminologyDb::new_embedded(),
            domain_detector: DomainDetector::new(),
            terminology_client: TerminologyApiClient::new(),
            disambiguator: SenseDisambiguator::new(),
            entity_classifier: EntityClassifier::new(),
            sampler: DistributionSampler::new(),
            syntax_restructurer: SyntaxRestructurer::new(),
            cadence_modulator: CadenceModulator::new(),
            function_word_modulator: FunctionWordModulator::new(),
            typing_engine: TypingNoiseEngine::new(),
        }
    }

    /// Single transformation pass
    pub fn transform(&self, text: &str, config: &StripperConfig) -> TransformResult {
        self.transform_multipass(text, config, 1)
    }

    /// Multi-pass transformation with comprehensive metrics, state tracking, and detector correlation
    pub fn transform_multipass(
        &self,
        text: &str,
        config: &StripperConfig,
        total_passes: usize,
    ) -> TransformResult {
        let input_sha256 = compute_sha256(text);

        // 1. Initial tokenization and tagging of original text
        let mut orig_tokens = self.tokenizer.tokenize(text);
        self.pos_tagger.tag_tokens(&mut orig_tokens);
        for token in &mut orig_tokens {
            if token.is_word() {
                token.lemma = Some(self.lemmatizer.lemmatize(&token.text, token.pos));
            }
        }
        let original_structural_metrics = compute_structural_metrics(&orig_tokens);

        let mut tokens = orig_tokens.clone();
        let original_word_tokens: Vec<String> = orig_tokens
            .iter()
            .filter(|t| t.is_word())
            .map(|t| t.text.to_lowercase())
            .collect();

        let mut unique_original_content: HashSet<String> = HashSet::new();
        let mut content_word_lemmas: Vec<String> = Vec::new();
        for t in &orig_tokens {
            if t.is_word() && t.pos.is_content_word() {
                unique_original_content.insert(t.text.to_lowercase());
                content_word_lemmas.push(t.lemma.clone().unwrap_or_else(|| t.text.to_lowercase()));
            }
        }

        let all_word_lemmas: Vec<String> = orig_tokens
            .iter()
            .filter(|t| t.is_word())
            .map(|t| t.lemma.clone().unwrap_or_else(|| t.text.to_lowercase()))
            .collect();

        let (terminology_enabled, term_domain_filter, protect_domain_terms) =
            if let Some(ref tc) = config.terminology_config {
                (tc.enabled, tc.domain_filter.as_deref(), tc.protect_domain_terms)
            } else {
                (false, None, false)
            };

        let detected_domains = self
            .domain_detector
            .score_document(&orig_tokens, &self.terminology_db);

        let active_domain = if config.domain == Domain::Auto {
            let primary = self.domain_detector.primary_domain(&detected_domains);
            let classified = self.domain_classifier.detect_domain(&all_word_lemmas);

            if classified != Domain::General {
                classified
            } else {
                match primary.as_str() {
                    "finance" | "economics" => Domain::Finance,
                    "biomedical" => Domain::Biomedical,
                    "computer_science" => Domain::ComputerScience,
                    "legal" => Domain::Legal,
                    "culinary" | "gastronomy" => Domain::Culinary,
                    "creative_arts" | "art" => Domain::CreativeArts,
                    "science" => Domain::Science,
                    "politics" => Domain::Politics,
                    "sports" => Domain::Sports,
                    "education" => Domain::Education,
                    "philosophy_psychology" | "philosophy" | "psychology" => Domain::PhilosophyPsychology,
                    "military" => Domain::Military,
                    "journalism_media" | "journalism" | "media" => Domain::JournalismMedia,
                    "travel_tourism" | "tourism" => Domain::TravelTourism,
                    "aviation_aerospace" | "aviation" | "aerospace" => Domain::AviationAerospace,
                    "architecture_construction" | "architecture" | "construction" => Domain::ArchitectureConstruction,
                    "gaming_esports" | "gaming" | "esports" => Domain::GamingEsports,
                    "agriculture_botany" | "agriculture" | "botany" => Domain::AgricultureBotany,
                    "fashion_textiles" | "fashion" | "textiles" => Domain::FashionTextiles,
                    "theology_religion" | "theology" | "religion" => Domain::TheologyReligion,
                    "audio_engineering" | "audio" => Domain::AudioEngineering,
                    "maritime_nautical" | "maritime" | "nautical" => Domain::MaritimeNautical,
                    "automotive_motorsport" | "automotive" | "motorsport" => Domain::AutomotiveMotorsport,
                    "film_television" | "film" | "television" => Domain::FilmTelevision,
                    "linguistics_philology" | "linguistics" | "philology" => Domain::LinguisticsPhilology,
                    "real_estate_property" | "real_estate" | "property" => Domain::RealEstateProperty,
                    "logistics_supply_chain" | "logistics" | "supply_chain" => Domain::LogisticsSupplyChain,
                    "fitness_kinesiology" | "fitness" | "kinesiology" => Domain::FitnessKinesiology,
                    "occult_astrology" | "occult" | "astrology" => Domain::OccultAstrology,
                    _ => Domain::General,
                }
            }
        } else {
            config.domain
        };

        // Longest-Match-First multiword terminology span detection
        let matched_spans = if terminology_enabled {
            self.terminology_db
                .scan_spans(&orig_tokens, term_domain_filter)
        } else {
            Vec::new()
        };

        let mut span_by_start: HashMap<usize, MatchedTermSpan> = HashMap::new();
        let mut covered_by_multiword: HashSet<usize> = HashSet::new();
        let mut domain_word_tokens: HashSet<usize> = HashSet::new();

        for span in &matched_spans {
            span_by_start.insert(span.start_token_idx, span.clone());
            for idx in span.start_token_idx..span.end_token_idx {
                covered_by_multiword.insert(idx);
                domain_word_tokens.insert(idx);
            }
        }

        let mut domain_locked_tokens_count = 0usize;
        let mut protected_terms_count = 0usize;
        let mut terminology_candidates_considered = 0usize;
        let mut terminology_candidates_rejected = 0usize;
        let mut terminology_replacements = 0usize;
        let mut term_diagnostics = Vec::new();
        let mut candidates_per_content_word: Vec<usize> = Vec::new();

        let mut pass_turnovers: Vec<f64> = Vec::new();
        let mut pass_jsd: Vec<f64> = Vec::new();
        let mut pass_replacements: Vec<usize> = Vec::new();

        let mut all_transform_records: Vec<WordTransform> = Vec::new();
        let mut all_rejection_records: Vec<RejectionRecord> = Vec::new();
        let mut all_audit_records: Vec<TransformationAuditRecord> = Vec::new();
        let mut categories_affected: HashMap<String, usize> = HashMap::new();
        let mut replacements_by_class: HashMap<String, usize> = HashMap::new();

        let mut convergence_pass = None;
        let mut fixed_point_reached = false;
        let mut prev_text = text.to_string();

        let loaded_heatmap: Option<SensitivityHeatmapModel> =
            if let Some(ref sc) = config.sensitivity_config {
                if let Some(ref path) = sc.heatmap_path {
                    SensitivityAnalyzer::load_heatmap_json(path).ok()
                } else {
                    None
                }
            } else {
                None
            };

        let loaded_composite_model: Option<CompositeModelWeights> =
            if let Some(ref sc) = config.sensitivity_config {
                if let Some(ref path) = sc.composite_model_path {
                    std::fs::read_to_string(path).ok().and_then(|c| serde_json::from_str(&c).ok())
                } else {
                    None
                }
            } else {
                None
            };

        let passes_to_run = total_passes.max(1);

        for pass_idx in 1..=passes_to_run {
            if fixed_point_reached {
                pass_turnovers.push(*pass_turnovers.last().unwrap_or(&0.0));
                pass_jsd.push(*pass_jsd.last().unwrap_or(&0.0));
                pass_replacements.push(0);
                continue;
            }

            let mut rng = match config.seed {
                Some(seed) => StdRng::seed_from_u64(seed + (pass_idx as u64 * 47)),
                None => StdRng::from_entropy(),
            };

            let mut pass_replacements_count = 0;

            let selected_budget_indices: Option<HashSet<usize>> =
                if let Some(ref sc) = config.sensitivity_config {
                    let mut eligible = Vec::new();
                    for (idx, tok) in tokens.iter().enumerate() {
                        let token_lemma = tok.lemma.clone().unwrap_or_else(|| tok.text.to_lowercase());
                        let coarse_pos = tok.pos.coarse_pos();
                        let has_synsets = !self.synset_db.find_synsets(&token_lemma, coarse_pos).is_empty();

                        if tok.is_word()
                            && tok.pos.is_content_word()
                            && has_synsets
                            && !self.entity_classifier.is_protected(tok)
                            && !covered_by_multiword.contains(&idx)
                            && (config.allow_re_replacement || !tok.is_transformed)
                        {
                            eligible.push(idx);
                        }
                    }

                    let total_eligible = eligible.len();
                    if total_eligible == 0 {
                        Some(HashSet::new())
                    } else {
                        let budget = sc
                            .edit_budget
                            .unwrap_or_else(|| {
                                ((total_eligible as f64 * config.replacement_probability).round()
                                    as usize)
                                    .max(1)
                            })
                            .min(total_eligible);

                        match sc.policy {
                            SensitivitySamplingPolicy::Uniform => {
                                let mut pool = eligible;
                                let mut chosen = HashSet::new();
                                for _ in 0..budget {
                                    if pool.is_empty() {
                                        break;
                                    }
                                    let pick = rng.gen_range(0..pool.len());
                                    chosen.insert(pool.swap_remove(pick));
                                }
                                Some(chosen)
                            }
                            SensitivitySamplingPolicy::CompositeModel
                            | SensitivitySamplingPolicy::ShuffledComposite
                            | SensitivitySamplingPolicy::LowComposite => {
                                let total_toks = tokens.len().max(1);
                                let mut pool = eligible;
                                let mut weights: Vec<f64> = pool
                                    .iter()
                                    .map(|&idx| {
                                        let tok = &tokens[idx];
                                        let norm_pos = idx as f64 / total_toks as f64;
                                        let score = if let Some(ref cm) = loaded_composite_model {
                                            let coarse = tok.pos.coarse_pos();
                                            let is_noun = if coarse == CoarsePos::Noun { 1.0 } else { 0.0 };
                                            let is_verb = if coarse == CoarsePos::Verb { 1.0 } else { 0.0 };
                                            let is_adj = if coarse == CoarsePos::Adjective { 1.0 } else { 0.0 };
                                            let tok_len = tok.text.len() as f64;
                                            let mut pred = cm.intercept;

                                            for (f_idx, fname) in cm.feature_names.iter().enumerate() {
                                                let w = cm.weights.get(f_idx).copied().unwrap_or(0.0);
                                                let m = cm.feature_means.get(f_idx).copied().unwrap_or(0.0);
                                                let s = cm.feature_stds.get(f_idx).copied().unwrap_or(1.0).max(1e-5);

                                                let raw_val = match fname.as_str() {
                                                    "Norm_Doc_Pos" | "pos_norm" => norm_pos,
                                                    "Norm_Doc_Pos_Sq" | "pos_norm_sq" => norm_pos * norm_pos,
                                                    "POS_Noun" | "is_noun" => is_noun,
                                                    "POS_Verb" | "is_verb" => is_verb,
                                                    "POS_Adj" | "is_adj" => is_adj,
                                                    "Token_Length" | "token_len" => tok_len,
                                                    "Context_Window_Count" | "window_count" => 2.0,
                                                    _ => 0.0,
                                                };
                                                let std_val = (raw_val - m) / s;
                                                pred += w * std_val;
                                            }
                                            pred.max(0.01)
                                        } else {
                                            (0.10 + (norm_pos - 0.5).abs() * 0.05).max(0.01)
                                        };

                                        if sc.policy == SensitivitySamplingPolicy::LowComposite {
                                            (1.0f64 / (score + 0.001f64)).max(0.001f64)
                                        } else {
                                            score.max(0.001f64)
                                        }
                                    })
                                    .collect();

                                if sc.policy == SensitivitySamplingPolicy::ShuffledComposite {
                                    use rand::seq::SliceRandom;
                                    weights.shuffle(&mut rng);
                                }

                                let mut chosen = HashSet::new();
                                for _ in 0..budget {
                                    if pool.is_empty() {
                                        break;
                                    }
                                    let total_w: f64 = weights.iter().sum();
                                    if total_w <= 0.0 {
                                        let pick = rng.gen_range(0..pool.len());
                                        chosen.insert(pool.swap_remove(pick));
                                        weights.swap_remove(pick);
                                        continue;
                                    }
                                    let p: f64 = rng.gen_range(0.0..total_w);
                                    let mut cum = 0.0;
                                    let mut pick = 0;
                                    for (wi, &w) in weights.iter().enumerate() {
                                        cum += w;
                                        if p <= cum {
                                            pick = wi;
                                            break;
                                        }
                                    }
                                    chosen.insert(pool.swap_remove(pick));
                                    weights.swap_remove(pick);
                                }
                                Some(chosen)
                            }
                            SensitivitySamplingPolicy::HighSensitivity
                            | SensitivitySamplingPolicy::LowSensitivity
                            | SensitivitySamplingPolicy::ShuffledSpatial => {
                                let total_toks = tokens.len().max(1);
                                let mut pool = eligible;
                                let mut weights: Vec<f64> = pool
                                    .iter()
                                    .map(|&idx| {
                                        let norm_pos = idx as f64 / total_toks as f64;
                                        let sensitivity_val = if let Some(ref hm) = loaded_heatmap {
                                            let bin_idx = ((norm_pos
                                                * hm.position_bins_20.len() as f64)
                                                .floor()
                                                as usize)
                                                .min(hm.position_bins_20.len().saturating_sub(1));
                                            hm.position_bins_20
                                                .get(bin_idx)
                                                .map(|b| b.mean_abs_delta_z.max(0.01))
                                                .unwrap_or(1.0)
                                        } else {
                                            (1.0 + (norm_pos - 0.5).abs()).max(0.01)
                                        };

                                        if sc.policy == SensitivitySamplingPolicy::LowSensitivity {
                                            (1.0f64 / (sensitivity_val + 0.001f64)).max(0.001f64)
                                        } else {
                                            sensitivity_val.max(0.001f64)
                                        }
                                    })
                                    .collect();

                                if sc.policy == SensitivitySamplingPolicy::ShuffledSpatial {
                                    use rand::seq::SliceRandom;
                                    weights.shuffle(&mut rng);
                                }

                                let mut chosen = HashSet::new();
                                for _ in 0..budget {
                                    if pool.is_empty() {
                                        break;
                                    }
                                    let total_w: f64 = weights.iter().sum();
                                    if total_w <= 0.0 {
                                        let pick = rng.gen_range(0..pool.len());
                                        chosen.insert(pool.swap_remove(pick));
                                        weights.swap_remove(pick);
                                        continue;
                                    }
                                    let p: f64 = rng.gen_range(0.0..total_w);
                                    let mut cum = 0.0;
                                    let mut pick = 0;
                                    for (wi, &w) in weights.iter().enumerate() {
                                        cum += w;
                                        if p <= cum {
                                            pick = wi;
                                            break;
                                        }
                                    }
                                    chosen.insert(pool.swap_remove(pick));
                                    weights.swap_remove(pick);
                                }
                                Some(chosen)
                            }
                        }
                    }
                } else {
                    None
                };

            // --- LAYER 1: Lexical & Terminology Substitution ---
            if config.enable_lexical {
                let mut i = 0;
                while i < tokens.len() {
                    if !tokens[i].is_word() {
                        i += 1;
                        continue;
                    }

                    // --- 1.A: MULTIWORD TERMINOLOGY PHRASE PROCESSING ---
                    if terminology_enabled {
                        if let Some(span) = span_by_start.get(&i).cloned() {
                            let span_len = span.end_token_idx - span.start_token_idx;
                            if span.is_locked || protect_domain_terms || span.acceptable_variants.is_empty() {
                                domain_locked_tokens_count += span_len;
                                if span.relation == TermRelation::ProtectedTerm {
                                    protected_terms_count += span_len;
                                }
                                all_rejection_records.push(RejectionRecord {
                                    word: span.matched_text.clone(),
                                    candidate: None,
                                    pos: tokens[i].pos,
                                    reason: RejectionReason::DomainLockedTerm {
                                        domain: span.domain.clone(),
                                    },
                                });
                                term_diagnostics.push(TerminologyRejectionDiagnostic {
                                    original_term: span.matched_text.clone(),
                                    candidate: "generic substitution".to_string(),
                                    domain: span.domain.clone(),
                                    reason: "DOMAIN_LOCKED / PROTECTED".to_string(),
                                });
                                i = span.end_token_idx;
                                continue;
                            } else {
                                // Multiword variant substitution (e.g. "market capitalization" -> "market cap")
                                terminology_candidates_considered += span.acceptable_variants.len();
                                let chosen_variant = &span.acceptable_variants[0];
                                let orig_text = span.matched_text.clone();

                                tokens[i].text = chosen_variant.clone();
                                tokens[i].is_transformed = true;
                                for k in (i + 1)..span.end_token_idx {
                                    tokens[k].text = String::new();
                                    tokens[k].is_transformed = true;
                                }

                                terminology_replacements += 1;
                                pass_replacements_count += 1;
                                *replacements_by_class.entry("lexical".into()).or_insert(0) += 1;

                                all_transform_records.push(WordTransform {
                                    token_index: i,
                                    original: orig_text,
                                    replacement: chosen_variant.clone(),
                                    original_lemma: span.canonical_term.clone(),
                                    replacement_lemma: chosen_variant.clone(),
                                    pos: tokens[i].pos,
                                    transformation_class: TransformationClass::Lexical,
                                    sense_gloss: format!("IATE/EuroVoc {} terminology equivalent", span.domain),
                                    confidence: 0.95,
                                    candidates_considered: vec![CandidateScore {
                                        lemma: chosen_variant.clone(),
                                        inflected: chosen_variant.clone(),
                                        semantic_score: 1.0,
                                        frequency_weight: 1.0,
                                        final_probability: 1.0,
                                    }],
                                    pass_number: pass_idx,
                                });

                                i = span.end_token_idx;
                                continue;
                            }
                        } else if covered_by_multiword.contains(&i) {
                            // Subsumed by multiword phrase
                            i += 1;
                            continue;
                        }
                    }

                    let token_pos = tokens[i].pos;
                    let token_casing = tokens[i].casing;
                    let token_text = tokens[i].text.clone();
                    let token_is_transformed = tokens[i].is_transformed;
                    let token_lemma = tokens[i]
                        .lemma
                        .clone()
                        .unwrap_or_else(|| token_text.to_lowercase());

                    // Fixed-point protection
                    if !config.allow_re_replacement && token_is_transformed {
                        all_rejection_records.push(RejectionRecord {
                            word: token_text,
                            candidate: None,
                            pos: token_pos,
                            reason: RejectionReason::AlreadyTransformed,
                        });
                        i += 1;
                        continue;
                    }

                    // Check protected entity or stopword
                    if self.entity_classifier.is_protected(&tokens[i]) {
                        let reason = if token_pos.is_content_word() {
                            RejectionReason::NamedEntityOrProperNoun
                        } else {
                            RejectionReason::ClosedClassOrStopword
                        };
                        all_rejection_records.push(RejectionRecord {
                            word: token_text,
                            candidate: None,
                            pos: token_pos,
                            reason,
                        });
                        i += 1;
                        continue;
                    }

                    // Domain terminology single-token check
                    if terminology_enabled {
                        let term_matches = self.terminology_db.lookup_phrase(
                            &token_text.to_lowercase(),
                            &token_lemma,
                            term_domain_filter,
                        );
                        if let Some(term_entry) = term_matches.first() {
                            domain_word_tokens.insert(i);
                            if term_entry.status == TermStatus::DomainLocked
                                || term_entry.relation == TermRelation::ProtectedTerm
                                || protect_domain_terms
                            {
                                domain_locked_tokens_count += 1;
                                if term_entry.relation == TermRelation::ProtectedTerm {
                                    protected_terms_count += 1;
                                }
                                all_rejection_records.push(RejectionRecord {
                                    word: token_text.clone(),
                                    candidate: None,
                                    pos: token_pos,
                                    reason: RejectionReason::DomainLockedTerm {
                                        domain: term_entry.domain_name.clone(),
                                    },
                                });
                                term_diagnostics.push(TerminologyRejectionDiagnostic {
                                    original_term: token_text.clone(),
                                    candidate: "none".into(),
                                    domain: term_entry.domain_name.clone(),
                                    reason: format!("{}: {}", term_entry.status, term_entry.relation),
                                });
                                i += 1;
                                continue;
                            }
                        }
                    }

                    // Fallback domain classifier check
                    let lex_class = self
                        .domain_classifier
                        .classify_lexical_class(&token_lemma, active_domain);
                    if lex_class == LexicalClass::DomainLocked {
                        domain_locked_tokens_count += 1;
                        all_rejection_records.push(RejectionRecord {
                            word: token_text,
                            candidate: None,
                            pos: token_pos,
                            reason: RejectionReason::DomainLockedTerm {
                                domain: active_domain.to_string(),
                            },
                        });
                        i += 1;
                        continue;
                    }

                    // Probability or sensitivity budget gate
                    if let Some(ref sel) = selected_budget_indices {
                        if !sel.contains(&i) {
                            all_rejection_records.push(RejectionRecord {
                                word: token_text,
                                candidate: None,
                                pos: token_pos,
                                reason: RejectionReason::ProbabilitySkipped,
                            });
                            i += 1;
                            continue;
                        }
                    } else if config.replacement_probability < 1.0 {
                        let r: f64 = rng.gen_range(0.0..1.0);
                        if r > config.replacement_probability {
                            all_rejection_records.push(RejectionRecord {
                                word: token_text,
                                candidate: None,
                                pos: token_pos,
                                reason: RejectionReason::ProbabilitySkipped,
                            });
                            i += 1;
                            continue;
                        }
                    }

                    let coarse_pos = token_pos.coarse_pos();
                    let synsets = self.synset_db.find_synsets(&token_lemma, coarse_pos);

                    let (chosen_gloss, raw_candidate_lemmas, confidence) = if !synsets.is_empty() {
                        let wsd_result = self.disambiguator.disambiguate(&tokens, i, &synsets, 10);
                        if wsd_result.confidence < config.min_confidence {
                            all_rejection_records.push(RejectionRecord {
                                word: token_text,
                                candidate: None,
                                pos: token_pos,
                                reason: RejectionReason::LowContextConfidence {
                                    confidence: wsd_result.confidence,
                                    threshold: config.min_confidence,
                                },
                            });
                            i += 1;
                            continue;
                        }

                        let chosen_synset = match wsd_result.best_synset {
                            Some(s) => s,
                            None => {
                                all_rejection_records.push(RejectionRecord {
                                    word: token_text,
                                    candidate: None,
                                    pos: token_pos,
                                    reason: RejectionReason::NoSynsetFound,
                                });
                                i += 1;
                                continue;
                            }
                        };

                        let cands: Vec<String> = chosen_synset
                            .lemmas
                            .iter()
                            .filter(|&&cand| {
                                if cand.eq_ignore_ascii_case(&token_lemma) {
                                    return false;
                                }
                                if chosen_synset
                                    .antonyms
                                    .iter()
                                    .any(|&ant| ant.eq_ignore_ascii_case(cand))
                                {
                                    return false;
                                }
                                true
                            })
                            .map(|&s| s.to_string())
                            .collect();

                        (
                            chosen_synset.gloss.to_string(),
                            cands,
                            wsd_result.confidence,
                        )
                    } else if config.use_thesaurus_api {
                        let api_cands =
                            self.thesaurus_client.fetch_synonyms(&token_text, token_pos);
                        if api_cands.is_empty() {
                            all_rejection_records.push(RejectionRecord {
                                word: token_text,
                                candidate: None,
                                pos: token_pos,
                                reason: RejectionReason::NoSynsetFound,
                            });
                            i += 1;
                            continue;
                        }
                        (
                            "thesaurus API semantic neighborhood".to_string(),
                            api_cands,
                            0.5,
                        )
                    } else {
                        all_rejection_records.push(RejectionRecord {
                            word: token_text,
                            candidate: None,
                            pos: token_pos,
                            reason: RejectionReason::NoSynsetFound,
                        });
                        i += 1;
                        continue;
                    };

                    // Filter candidate lemmas through domain-register validity gate
                    let mut candidate_lemmas = self.domain_classifier.filter_candidates(
                        &token_lemma,
                        coarse_pos,
                        active_domain,
                        raw_candidate_lemmas,
                    );

                    // Terminology layer relationship validation
                    if terminology_enabled {
                        let term_matches = self.terminology_db.lookup_phrase(
                            &token_text.to_lowercase(),
                            &token_lemma,
                            term_domain_filter,
                        );
                        if let Some(t_entry) = term_matches.first() {
                            terminology_candidates_considered += candidate_lemmas.len();
                            let acceptable_variants = self.terminology_db.get_acceptable_variants(
                                &token_lemma,
                                &t_entry.domain_name,
                            );

                            candidate_lemmas.retain(|cand| {
                                let is_valid = acceptable_variants.iter().any(|v| v.eq_ignore_ascii_case(cand));
                                if !is_valid {
                                    terminology_candidates_rejected += 1;
                                    term_diagnostics.push(TerminologyRejectionDiagnostic {
                                        original_term: token_text.clone(),
                                        candidate: cand.clone(),
                                        domain: t_entry.domain_name.clone(),
                                        reason: "GENERAL_ENGLISH / DOMAIN_MISMATCH".to_string(),
                                    });
                                }
                                is_valid
                            });
                        }
                    }

                    if token_pos.is_content_word() {
                        candidates_per_content_word.push(candidate_lemmas.len());
                    }

                    if candidate_lemmas.is_empty() {
                        all_rejection_records.push(RejectionRecord {
                            word: token_text,
                            candidate: None,
                            pos: token_pos,
                            reason: RejectionReason::NoSynsetFound,
                        });
                        i += 1;
                        continue;
                    }

                    if candidate_lemmas.len() > config.max_candidates_per_word {
                        candidate_lemmas.truncate(config.max_candidates_per_word);
                    }

                    let cand_refs: Vec<&str> =
                        candidate_lemmas.iter().map(|s| s.as_str()).collect();

                    let words_before: Vec<&str> = tokens[..i]
                        .iter()
                        .filter(|t| t.is_word())
                        .map(|t| t.text.as_str())
                        .collect();
                    let words_after: Vec<&str> = tokens[i + 1..]
                        .iter()
                        .filter(|t| t.is_word())
                        .map(|t| t.text.as_str())
                        .collect();

                    let sample_result = self.sampler.sample_candidate_with_audit(
                        &cand_refs,
                        &token_text,
                        token_pos,
                        token_casing,
                        confidence,
                        &words_before,
                        &words_after,
                        &active_domain.to_string(),
                        domain_word_tokens.contains(&i),
                        config,
                        &mut rng,
                    );

                    if let Some((rep_lemma, inflected_text, conf, candidate_scores, mut audit_rec)) =
                        sample_result
                    {
                        if inflected_text.eq_ignore_ascii_case(&token_text) {
                            audit_rec.is_accepted = false;
                            all_audit_records.push(audit_rec);
                            all_rejection_records.push(RejectionRecord {
                                word: token_text,
                                candidate: Some(inflected_text),
                                pos: token_pos,
                                reason: RejectionReason::IdenticalToOriginal,
                            });
                            i += 1;
                            continue;
                        }

                        all_audit_records.push(audit_rec);

                        let pos_cat_str = format!("{:?}", token_pos.coarse_pos());
                        *categories_affected.entry(pos_cat_str).or_insert(0) += 1;
                        *replacements_by_class
                            .entry("lexical".to_string())
                            .or_insert(0) += 1;

                        all_transform_records.push(WordTransform {
                            token_index: i,
                            original: token_text,
                            replacement: inflected_text.clone(),
                            original_lemma: token_lemma,
                            replacement_lemma: rep_lemma,
                            pos: token_pos,
                            transformation_class: TransformationClass::Lexical,
                            sense_gloss: chosen_gloss,
                            confidence: conf,
                            candidates_considered: candidate_scores,
                            pass_number: pass_idx,
                        });

                        tokens[i].text = inflected_text;
                        tokens[i].is_transformed = true;
                        pass_replacements_count += 1;
                    }

                    i += 1;
                }
            }

            let current_text = tokens.iter().map(|t| t.text.as_str()).collect::<String>();
            let current_words: Vec<String> = tokens
                .iter()
                .filter(|t| t.is_word())
                .map(|t| t.text.to_lowercase())
                .collect();

            let jsd = compute_jsd(&original_word_tokens, &current_words);
            let mut unique_changed: HashSet<String> = HashSet::new();
            for tr in &all_transform_records {
                unique_changed.insert(tr.original.to_lowercase());
            }
            let turnover = if !unique_original_content.is_empty() {
                unique_changed.len() as f64 / unique_original_content.len() as f64
            } else {
                0.0
            };

            pass_turnovers.push(turnover);
            pass_jsd.push(jsd);
            pass_replacements.push(pass_replacements_count);

            if (pass_replacements_count == 0 || current_text == prev_text)
                && convergence_pass.is_none() {
                    convergence_pass = Some(pass_idx);
                    fixed_point_reached = true;
                }
            prev_text = current_text;
        }

        // --- LAYER 2: (Optional) Function-Word Variation ---
        let mut final_text = tokens.iter().map(|t| t.text.as_str()).collect::<String>();

        if config.enable_function_words {
            let before = all_transform_records.len();
            final_text = self.function_word_modulator.modulate_function_words(
                &final_text,
                &mut all_transform_records,
                1,
            );
            let delta = all_transform_records.len() - before;
            if delta > 0 {
                *replacements_by_class
                    .entry("function_word".into())
                    .or_insert(0) += delta;
            }
        }

        // --- LAYER 3: (Experimental Secondary) Syntax Restructuring ---
        if config.enable_syntax {
            let before = all_transform_records.len();
            final_text = self.syntax_restructurer.restructure_syntax(
                &final_text,
                &mut all_transform_records,
                1,
            );
            let delta = all_transform_records.len() - before;
            if delta > 0 {
                *replacements_by_class.entry("syntax".into()).or_insert(0) += delta;
            }
        }

        // --- LAYER 4: (Experimental Secondary) Cadence Modulation ---
        if config.enable_cadence {
            let before = all_transform_records.len();
            final_text =
                self.cadence_modulator
                    .modulate_cadence(&final_text, &mut all_transform_records, 1);
            let delta = all_transform_records.len() - before;
            if delta > 0 {
                *replacements_by_class.entry("cadence".into()).or_insert(0) += delta;
            }
        }

        // --- LAYER 5: Human Typing Noise Simulator ---
        let typing_metrics = if let Some(ref typing_cfg) = config.typing_noise_config {
            if typing_cfg.rate > 0.0 {
                let mut current_tokens = self.tokenizer.tokenize(&final_text);
                let metrics = self.typing_engine.apply_typing_noise(
                    &mut current_tokens,
                    typing_cfg,
                    config.seed,
                );
                final_text = current_tokens
                    .iter()
                    .map(|t| t.text.as_str())
                    .collect::<String>();
                Some(metrics)
            } else {
                None
            }
        } else {
            None
        };

        // Calculate general vs domain-specific turnover percentages
        let mut general_unique_total: HashSet<String> = HashSet::new();
        let mut general_unique_changed: HashSet<String> = HashSet::new();
        let mut domain_unique_total: HashSet<String> = HashSet::new();
        let mut domain_unique_changed: HashSet<String> = HashSet::new();

        for (idx, t) in orig_tokens.iter().enumerate() {
            if t.is_word() && t.pos.is_content_word() {
                let lower = t.text.to_lowercase();
                if domain_word_tokens.contains(&idx) {
                    domain_unique_total.insert(lower);
                } else {
                    general_unique_total.insert(lower);
                }
            }
        }

        for tr in &all_transform_records {
            let lower = tr.original.to_lowercase();
            if domain_unique_total.contains(&lower) {
                domain_unique_changed.insert(lower);
            } else {
                general_unique_changed.insert(lower);
            }
        }

        let general_lexical_turnover_pct = if !general_unique_total.is_empty() {
            (general_unique_changed.len() as f64 / general_unique_total.len() as f64) * 100.0
        } else {
            0.0
        };

        let domain_lexical_turnover_pct = if !domain_unique_total.is_empty() {
            (domain_unique_changed.len() as f64 / domain_unique_total.len() as f64) * 100.0
        } else {
            0.0
        };

        let domain_terms_with_variants = matched_spans
            .iter()
            .filter(|s| !s.acceptable_variants.is_empty())
            .count();

        let terminology_metrics = if terminology_enabled {
            Some(TerminologyMetrics {
                terminology_enabled: true,
                terminology_source: config
                    .terminology_config
                    .as_ref()
                    .map(|c| c.source.clone())
                    .unwrap_or_else(|| "local".into()),
                terminology_version: self.terminology_db.version.clone(),
                terminology_hash: self.terminology_db.get_hash().to_string(),
                detected_domains: detected_domains.clone(),
                domain_terms_detected: matched_spans.len(),
                domain_terms_eligible: domain_word_tokens.len(),
                domain_terms_protected: domain_locked_tokens_count,
                domain_terms_with_variants,
                domain_terms_replaced: terminology_replacements,
                matched_terms_count: matched_spans.len(),
                matched_spans,
                protected_terms_count,
                domain_locked_terms_count: domain_locked_tokens_count,
                terminology_candidates_considered,
                terminology_candidates_rejected,
                terminology_replacements,
                general_lexical_turnover_pct,
                domain_lexical_turnover_pct,
                diagnostics: term_diagnostics,
            })
        } else {
            None
        };

        let output_sha256 = compute_sha256(&final_text);

        // Final tokenization and metrics calculation
        let mut final_tokens = self.tokenizer.tokenize(&final_text);
        self.pos_tagger.tag_tokens(&mut final_tokens);
        let transformed_structural_metrics = compute_structural_metrics(&final_tokens);

        let total_words = orig_tokens.iter().filter(|t| t.is_word()).count();
        let eligible_words = orig_tokens
            .iter()
            .filter(|t| t.is_word() && t.pos.is_content_word() && !self.entity_classifier.is_protected(t))
            .count();

        let replaced_words = all_transform_records.len()
            + typing_metrics
                .as_ref()
                .map(|m| m.corrupted_tokens)
                .unwrap_or(0);
        let replacement_rate = if eligible_words > 0 {
            (replaced_words as f64 / eligible_words as f64) * 100.0
        } else {
            0.0
        };

        let mut unique_changed: HashSet<String> = HashSet::new();
        for tr in &all_transform_records {
            unique_changed.insert(tr.original.to_lowercase());
        }
        if let Some(ref tm) = typing_metrics {
            for m in &tm.mutations {
                unique_changed.insert(m.original_word.to_lowercase());
            }
        }
        let lexical_turnover = if !unique_original_content.is_empty() {
            unique_changed.len() as f64 / unique_original_content.len() as f64
        } else {
            0.0
        };

        // Multi-category JSD
        let jsd_metrics = compute_category_jsd(&orig_tokens, &final_tokens);

        let mean_confidence = if !all_transform_records.is_empty() {
            all_transform_records
                .iter()
                .map(|t| t.confidence)
                .sum::<f64>()
                / all_transform_records.len() as f64
        } else {
            0.0
        };

        let stability_report = if passes_to_run > 1 {
            Some(PassStabilityReport {
                total_passes: passes_to_run,
                pass_turnovers,
                pass_jsd,
                pass_replacements,
                convergence_pass,
                fixed_point_reached,
            })
        } else {
            None
        };

        let mut enabled_classes = Vec::new();
        if config.enable_lexical {
            enabled_classes.push(TransformationClass::Lexical);
        }
        if config.enable_syntax {
            enabled_classes.push(TransformationClass::Syntax);
        }
        if config.enable_cadence {
            enabled_classes.push(TransformationClass::Cadence);
        }
        if config.enable_function_words {
            enabled_classes.push(TransformationClass::FunctionWord);
        }
        if config
            .typing_noise_config
            .as_ref()
            .map(|c| c.rate > 0.0)
            .unwrap_or(false)
        {
            enabled_classes.push(TransformationClass::TypingNoise);
        }

        // Detector Observation & Multi-Metric Association Analysis
        let detector_impact = config
            .detector_import
            .as_ref()
            .map(|rec| DetectorImpactAnalysis {
                detector_name: rec.detector.clone(),
                detector_version: rec.version.clone().unwrap_or_else(|| "unknown".into()),
                classification: rec.classification.clone(),
                ai_probability: rec.ai_probability,
                human_probability: rec.human_probability,
                mixed_probability: rec.mixed_probability,
                total_jsd: jsd_metrics.total_jsd,
                content_words_jsd: jsd_metrics.content_words_jsd,
                nouns_jsd: jsd_metrics.nouns_jsd,
                verbs_jsd: jsd_metrics.verbs_jsd,
                adjectives_jsd: jsd_metrics.adjectives_jsd,
                adverbs_jsd: jsd_metrics.adverbs_jsd,
                function_words_jsd: jsd_metrics.function_words_jsd,
                lexical_turnover,
                semantic_confidence: mean_confidence,
            });

        let effective_replacement_entropy = self.domain_classifier.compute_effective_entropy(
            &content_word_lemmas,
            active_domain,
            &candidates_per_content_word,
        );

        let report = TransformReport {
            input_sha256,
            output_sha256,
            seed_used: config.seed,
            mode_used: config.mode,
            enabled_classes,
            confidence_threshold: config.min_confidence,
            detected_domain: active_domain.to_string(),
            effective_replacement_entropy,
            domain_locked_tokens_count,
            total_tokens: tokens.len(),
            total_words,
            eligible_words,
            replaced_words,
            replacement_rate,
            replacements_by_class,
            unique_original_tokens: unique_original_content.len(),
            unique_original_tokens_changed: unique_changed.len(),
            lexical_turnover,
            jsd_metrics,
            original_structural_metrics,
            transformed_structural_metrics,
            stability_report,
            categories_affected,
            mean_confidence,
            replacements: all_transform_records,
            rejected_candidates: all_rejection_records,
            typing_metrics,
            terminology_metrics,
            detector_impact,
            audit_records: all_audit_records,
        };

        TransformResult {
            transformed_text: final_text,
            report,
        }
    }
}

/// Computes SHA-256 hash string for cryptographic reproducibility
fn compute_sha256(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Computes multi-category Jensen-Shannon Divergence across grammatical subsets
fn compute_category_jsd(
    orig_tokens: &[crate::tokenization::Token],
    final_tokens: &[crate::tokenization::Token],
) -> CategoryJsdMetrics {
    let orig_all: Vec<String> = orig_tokens
        .iter()
        .filter(|t| t.is_word())
        .map(|t| t.text.to_lowercase())
        .collect();
    let final_all: Vec<String> = final_tokens
        .iter()
        .filter(|t| t.is_word())
        .map(|t| t.text.to_lowercase())
        .collect();

    let orig_content: Vec<String> = orig_tokens
        .iter()
        .filter(|t| t.is_word() && t.pos.is_content_word())
        .map(|t| t.text.to_lowercase())
        .collect();
    let final_content: Vec<String> = final_tokens
        .iter()
        .filter(|t| t.is_word() && t.pos.is_content_word())
        .map(|t| t.text.to_lowercase())
        .collect();

    let orig_nouns: Vec<String> = orig_tokens
        .iter()
        .filter(|t| {
            t.is_word() && matches!(t.pos, crate::types::PosTag::NN | crate::types::PosTag::NNS)
        })
        .map(|t| t.text.to_lowercase())
        .collect();
    let final_nouns: Vec<String> = final_tokens
        .iter()
        .filter(|t| {
            t.is_word() && matches!(t.pos, crate::types::PosTag::NN | crate::types::PosTag::NNS)
        })
        .map(|t| t.text.to_lowercase())
        .collect();

    let orig_verbs: Vec<String> = orig_tokens
        .iter()
        .filter(|t| {
            t.is_word()
                && matches!(
                    t.pos,
                    crate::types::PosTag::VB
                        | crate::types::PosTag::VBD
                        | crate::types::PosTag::VBG
                        | crate::types::PosTag::VBN
                        | crate::types::PosTag::VBP
                        | crate::types::PosTag::VBZ
                )
        })
        .map(|t| t.text.to_lowercase())
        .collect();
    let final_verbs: Vec<String> = final_tokens
        .iter()
        .filter(|t| {
            t.is_word()
                && matches!(
                    t.pos,
                    crate::types::PosTag::VB
                        | crate::types::PosTag::VBD
                        | crate::types::PosTag::VBG
                        | crate::types::PosTag::VBN
                        | crate::types::PosTag::VBP
                        | crate::types::PosTag::VBZ
                )
        })
        .map(|t| t.text.to_lowercase())
        .collect();

    let orig_adjs: Vec<String> = orig_tokens
        .iter()
        .filter(|t| {
            t.is_word()
                && matches!(
                    t.pos,
                    crate::types::PosTag::JJ
                        | crate::types::PosTag::JJR
                        | crate::types::PosTag::JJS
                )
        })
        .map(|t| t.text.to_lowercase())
        .collect();
    let final_adjs: Vec<String> = final_tokens
        .iter()
        .filter(|t| {
            t.is_word()
                && matches!(
                    t.pos,
                    crate::types::PosTag::JJ
                        | crate::types::PosTag::JJR
                        | crate::types::PosTag::JJS
                )
        })
        .map(|t| t.text.to_lowercase())
        .collect();

    let orig_advs: Vec<String> = orig_tokens
        .iter()
        .filter(|t| {
            t.is_word()
                && matches!(
                    t.pos,
                    crate::types::PosTag::RB
                        | crate::types::PosTag::RBR
                        | crate::types::PosTag::RBS
                )
        })
        .map(|t| t.text.to_lowercase())
        .collect();
    let final_advs: Vec<String> = final_tokens
        .iter()
        .filter(|t| {
            t.is_word()
                && matches!(
                    t.pos,
                    crate::types::PosTag::RB
                        | crate::types::PosTag::RBR
                        | crate::types::PosTag::RBS
                )
        })
        .map(|t| t.text.to_lowercase())
        .collect();

    let orig_func: Vec<String> = orig_tokens
        .iter()
        .filter(|t| t.is_word() && !t.pos.is_content_word())
        .map(|t| t.text.to_lowercase())
        .collect();
    let final_func: Vec<String> = final_tokens
        .iter()
        .filter(|t| t.is_word() && !t.pos.is_content_word())
        .map(|t| t.text.to_lowercase())
        .collect();

    CategoryJsdMetrics {
        total_jsd: compute_jsd(&orig_all, &final_all),
        content_words_jsd: compute_jsd(&orig_content, &final_content),
        nouns_jsd: compute_jsd(&orig_nouns, &final_nouns),
        verbs_jsd: compute_jsd(&orig_verbs, &final_verbs),
        adjectives_jsd: compute_jsd(&orig_adjs, &final_adjs),
        adverbs_jsd: compute_jsd(&orig_advs, &final_advs),
        function_words_jsd: compute_jsd(&orig_func, &final_func),
    }
}

/// Calculates structural and syntactic metrics for a token sequence
fn compute_structural_metrics(tokens: &[crate::tokenization::Token]) -> StructuralMetrics {
    let mut sentences: Vec<Vec<&crate::tokenization::Token>> = Vec::new();
    let mut current_sentence: Vec<&crate::tokenization::Token> = Vec::new();

    let mut pos_distribution: HashMap<String, usize> = HashMap::new();
    let mut conjunction_distribution: HashMap<String, usize> = HashMap::new();
    let mut adjective_distribution: HashMap<String, usize> = HashMap::new();

    let mut content_word_count = 0;
    let mut function_word_count = 0;

    for token in tokens {
        if token.is_word() {
            *pos_distribution.entry(token.pos.to_string()).or_insert(0) += 1;
            if token.pos.is_content_word() {
                content_word_count += 1;
            } else {
                function_word_count += 1;
            }

            if matches!(
                token.pos,
                crate::types::PosTag::CC | crate::types::PosTag::IN
            ) {
                *conjunction_distribution
                    .entry(token.text.to_lowercase())
                    .or_insert(0) += 1;
            }
            if matches!(
                token.pos,
                crate::types::PosTag::JJ | crate::types::PosTag::JJR | crate::types::PosTag::JJS
            ) {
                *adjective_distribution
                    .entry(token.text.to_lowercase())
                    .or_insert(0) += 1;
            }

            current_sentence.push(token);
        }

        if (token.text == "." || token.text == "!" || token.text == "?" || token.text == "\n\n")
            && !current_sentence.is_empty() {
                sentences.push(current_sentence);
                current_sentence = Vec::new();
            }
    }
    if !current_sentence.is_empty() {
        sentences.push(current_sentence);
    }

    let sentence_count = sentences.len().max(1);
    let lengths: Vec<f64> = sentences.iter().map(|s| s.len() as f64).collect();
    let total_len: f64 = lengths.iter().sum();
    let mean_sentence_length = total_len / sentence_count as f64;

    let variance = if lengths.len() > 1 {
        let sum_sq_diff: f64 = lengths
            .iter()
            .map(|l| (l - mean_sentence_length).powi(2))
            .sum();
        sum_sq_diff / lengths.len() as f64
    } else {
        0.0
    };

    // Repeated sentence openings (first 2 words matching)
    let mut opening_counts: HashMap<String, usize> = HashMap::new();
    for s in &sentences {
        if s.len() >= 2 {
            let key = format!("{} {}", s[0].text.to_lowercase(), s[1].text.to_lowercase());
            *opening_counts.entry(key).or_insert(0) += 1;
        }
    }
    let repeated_sentence_openings = opening_counts
        .values()
        .filter(|&&c| c > 1)
        .map(|&c| c - 1)
        .sum();

    // Repeated syntactic templates (e.g. initial adverbial or subordinate clauses)
    let mut template_counts: HashMap<String, usize> = HashMap::new();
    for s in &sentences {
        if s.len() >= 4 {
            let pos_pattern = format!("{}-{}-{}-{}", s[0].pos, s[1].pos, s[2].pos, s[3].pos);
            *template_counts.entry(pos_pattern).or_insert(0) += 1;
        }
    }
    let repeated_syntactic_patterns = template_counts
        .values()
        .filter(|&&c| c > 1)
        .map(|&c| c - 1)
        .sum();

    let content_function_ratio = if function_word_count > 0 {
        content_word_count as f64 / function_word_count as f64
    } else {
        content_word_count as f64
    };

    StructuralMetrics {
        sentence_count,
        mean_sentence_length,
        sentence_length_variance: variance,
        pos_distribution,
        repeated_sentence_openings,
        repeated_syntactic_patterns,
        conjunction_distribution,
        adjective_distribution,
        content_word_count,
        function_word_count,
        content_function_ratio,
    }
}

/// Calculates Jensen-Shannon Divergence (JSD) in bits between two token collections
fn compute_jsd(p_tokens: &[String], q_tokens: &[String]) -> f64 {
    if p_tokens.is_empty() || q_tokens.is_empty() {
        return 0.0;
    }

    let mut p_counts: HashMap<&str, f64> = HashMap::new();
    for tok in p_tokens {
        *p_counts.entry(tok.as_str()).or_insert(0.0) += 1.0;
    }
    let p_total = p_tokens.len() as f64;

    let mut q_counts: HashMap<&str, f64> = HashMap::new();
    for tok in q_tokens {
        *q_counts.entry(tok.as_str()).or_insert(0.0) += 1.0;
    }
    let q_total = q_tokens.len() as f64;

    let mut vocab: HashSet<&str> = HashSet::new();
    for &k in p_counts.keys() {
        vocab.insert(k);
    }
    for &k in q_counts.keys() {
        vocab.insert(k);
    }

    let mut kl_pm = 0.0;
    let mut kl_qm = 0.0;

    for &w in &vocab {
        let p = p_counts.get(w).copied().unwrap_or(0.0) / p_total;
        let q = q_counts.get(w).copied().unwrap_or(0.0) / q_total;
        let m = 0.5 * (p + q);

        if p > 0.0 && m > 0.0 {
            kl_pm += p * (p / m).log2();
        }
        if q > 0.0 && m > 0.0 {
            kl_qm += q * (q / m).log2();
        }
    }

    0.5 * (kl_pm + kl_qm)
}
