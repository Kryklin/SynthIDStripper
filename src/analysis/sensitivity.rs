use crate::detector::{SynthIdConfig, SynthIdDetector};
use crate::lexicon::{DomainClassifier, SynsetDb};
use crate::morphology::{Inflector, Lemmatizer};
use crate::ner::EntityClassifier;
use crate::pos::PosTagger;
use crate::terminology::TerminologyDb;
use crate::tokenization::Tokenizer;
use crate::types::{
    Domain, PosTag, PositionalBinStats, SensitivityHeatmapModel, SensitivityObservationStatus,
    TokenSensitivityRecord,
};
use chrono::Utc;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io;

/// Metadata for an individual token position within a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPositionMeta {
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
}

pub struct SensitivityAnalyzer {
    tokenizer: Tokenizer,
    pos_tagger: PosTagger,
    lemmatizer: Lemmatizer,
    synset_db: SynsetDb,
    inflector: Inflector,
    entity_classifier: EntityClassifier,
    domain_classifier: DomainClassifier,
    terminology_db: TerminologyDb,
    detector: SynthIdDetector,
}

impl SensitivityAnalyzer {
    pub fn new(detector_config: SynthIdConfig) -> Self {
        Self {
            tokenizer: Tokenizer::new(),
            pos_tagger: PosTagger::new(),
            lemmatizer: Lemmatizer::new(),
            synset_db: SynsetDb::new(),
            inflector: Inflector::new(),
            entity_classifier: EntityClassifier::new(),
            domain_classifier: DomainClassifier::new(),
            terminology_db: TerminologyDb::new_embedded(),
            detector: SynthIdDetector::new(detector_config),
        }
    }

    /// Extracts structural token metadata with sentence, paragraph, and normalized position indices.
    pub fn extract_token_positions(
        &self,
        doc_id: &str,
        text: &str,
        domain_hint: Option<Domain>,
    ) -> (Vec<TokenPositionMeta>, f64) {
        let mut tokens = self.tokenizer.tokenize(text);
        self.pos_tagger.tag_tokens(self.pos_tagger_tokens(&mut tokens));

        let active_domain = match domain_hint {
            Some(Domain::Auto) | None => {
                let lemmas: Vec<String> = tokens
                    .iter()
                    .filter(|t| t.is_word())
                    .map(|t| self.lemmatizer.lemmatize(&t.text, t.pos).to_lowercase())
                    .collect();
                self.domain_classifier.detect_domain(&lemmas)
            }
            Some(d) => d,
        };

        let baseline_scan = self.detector.detect(text);
        let base_z = baseline_scan.z_score;

        let total_tokens = tokens.len().max(1);
        let mut metas = Vec::new();

        let mut sentence_idx = 0usize;
        let mut paragraph_idx = 0usize;
        let mut char_offset = 0usize;

        for (i, t) in tokens.iter().enumerate() {
            if t.text.contains("\n\n") {
                paragraph_idx += 1;
            }
            if t.is_sentence_start && i > 0 {
                sentence_idx += 1;
            }

            let lemma = self.lemmatizer.lemmatize(&t.text, t.pos);
            let norm_pos = i as f64 / total_tokens as f64;

            metas.push(TokenPositionMeta {
                document_id: doc_id.to_string(),
                token_index: i,
                sentence_index: sentence_idx,
                paragraph_index: paragraph_idx,
                token_text: t.text.clone(),
                lemma,
                pos: t.pos,
                domain_class: active_domain.to_string(),
                character_offset: char_offset,
                normalized_position: norm_pos,
            });

            char_offset += t.text.len();
        }

        (metas, base_z)
    }

    fn pos_tagger_tokens<'a>(
        &self,
        tokens: &'a mut Vec<crate::tokenization::Token>,
    ) -> &'a mut Vec<crate::tokenization::Token> {
        tokens
    }

    /// Performs empirical paired perturbation experiments for all eligible token positions in a document.
    pub fn profile_document(
        &self,
        doc_id: &str,
        text: &str,
        domain_hint: Option<Domain>,
        samples_per_token: usize,
        min_observations: usize,
        seed: u64,
    ) -> Vec<TokenSensitivityRecord> {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut tokens = self.tokenizer.tokenize(text);
        self.pos_tagger.tag_tokens(&mut tokens);

        for t in &mut tokens {
            t.lemma = Some(self.lemmatizer.lemmatize(&t.text, t.pos));
        }

        let active_domain = match domain_hint {
            Some(Domain::Auto) | None => {
                let lemmas: Vec<String> = tokens
                    .iter()
                    .filter(|t| t.is_word())
                    .map(|t| t.lemma.clone().unwrap_or_default().to_lowercase())
                    .collect();
                self.domain_classifier.detect_domain(&lemmas)
            }
            Some(d) => d,
        };

        let baseline_scan = self.detector.detect(text);
        let base_z = baseline_scan.z_score;
        let total_tokens = tokens.len().max(1);

        let mut sentence_idx = 0usize;
        let mut paragraph_idx = 0usize;
        let mut char_offset = 0usize;
        let mut records = Vec::new();

        // Identify multiword protected spans
        let matched_spans = self.terminology_db.scan_spans(&tokens, None);
        let mut locked_indices = std::collections::HashSet::new();
        for s in matched_spans {
            for idx in s.start_token_idx..s.end_token_idx {
                locked_indices.insert(idx);
            }
        }

        for i in 0..tokens.len() {
            let t = &tokens[i];
            if t.text.contains("\n\n") {
                paragraph_idx += 1;
            }
            if t.is_sentence_start && i > 0 {
                sentence_idx += 1;
            }

            let norm_pos = i as f64 / total_tokens as f64;
            let current_char_offset = char_offset;
            char_offset += t.text.len();

            let is_eligible = t.is_word()
                && t.pos.is_content_word()
                && !self.entity_classifier.is_protected(t)
                && !locked_indices.contains(&i);

            if !is_eligible {
                continue;
            }

            let token_lemma = t.lemma.clone().unwrap_or_else(|| t.text.to_lowercase());
            let raw_synsets = self.synset_db.get_candidate_synonyms(&token_lemma, t.pos.coarse_pos());
            let raw_cands: Vec<String> = raw_synsets.into_iter().map(|(w, _)| w.to_string()).collect();
            let vetted_cands = self.domain_classifier.filter_candidates(
                &token_lemma,
                t.pos.coarse_pos(),
                active_domain,
                raw_cands,
            );

            if vetted_cands.is_empty() {
                continue;
            }

            let mut delta_zs = Vec::new();
            let trials_to_run = samples_per_token.min(vetted_cands.len()).max(1);

            for trial in 0..trials_to_run {
                let cand_idx = if trials_to_run < vetted_cands.len() {
                    rng.gen_range(0..vetted_cands.len())
                } else {
                    trial
                };
                let cand_lemma = &vetted_cands[cand_idx];
                let inflected = self.inflector.inflect(cand_lemma, t.pos, t.casing);

                if inflected.eq_ignore_ascii_case(&t.text) {
                    continue;
                }

                // Create single-token perturbed text
                let mut perturbed_tokens = tokens.clone();
                perturbed_tokens[i].text = inflected;
                let perturbed_text = perturbed_tokens
                    .iter()
                    .map(|tok| tok.text.as_str())
                    .collect::<String>();

                let after_scan = self.detector.detect(&perturbed_text);
                let delta_z = after_scan.z_score - base_z;
                delta_zs.push((delta_z, after_scan.z_score));
            }

            if delta_zs.is_empty() {
                continue;
            }

            let n_trials = delta_zs.len();
            let dz_values: Vec<f64> = delta_zs.iter().map(|(dz, _)| *dz).collect();
            let mean_dz = dz_values.iter().sum::<f64>() / n_trials as f64;

            let mut sorted_dz = dz_values.clone();
            sorted_dz.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let median_dz = if n_trials % 2 == 1 {
                sorted_dz[n_trials / 2]
            } else {
                (sorted_dz[n_trials / 2 - 1] + sorted_dz[n_trials / 2]) / 2.0
            };

            let variance = if n_trials > 1 {
                dz_values
                    .iter()
                    .map(|dz| (dz - mean_dz).powi(2))
                    .sum::<f64>()
                    / (n_trials - 1) as f64
            } else {
                0.0
            };
            let std_dz = variance.sqrt();
            let se = if n_trials > 0 {
                std_dz / (n_trials as f64).sqrt()
            } else {
                0.0
            };
            let mean_abs_dz =
                dz_values.iter().map(|dz| dz.abs()).sum::<f64>() / n_trials as f64;

            let clean_transitions = delta_zs
                .iter()
                .filter(|(_, z_out)| *z_out < 1.645)
                .count();
            let clean_rate = if base_z >= 1.645 {
                clean_transitions as f64 / n_trials as f64
            } else {
                1.0
            };

            let status = if n_trials >= min_observations {
                SensitivityObservationStatus::Valid
            } else {
                SensitivityObservationStatus::UnderSampled
            };

            records.push(TokenSensitivityRecord {
                document_id: doc_id.to_string(),
                token_index: i,
                sentence_index: sentence_idx,
                paragraph_index: paragraph_idx,
                token_text: t.text.clone(),
                lemma: token_lemma,
                pos: t.pos,
                domain_class: active_domain.to_string(),
                character_offset: current_char_offset,
                normalized_position: norm_pos,
                mean_delta_z: mean_dz,
                median_delta_z: median_dz,
                std_delta_z: std_dz,
                mean_abs_delta_z: mean_abs_dz,
                clean_transition_rate: clean_rate,
                standard_error: se,
                ci95_low: mean_dz - 1.96 * se,
                ci95_high: mean_dz + 1.96 * se,
                n_trials,
                status,
            });
        }

        records
    }

    /// Aggregates token-level sensitivity records across multiple spatial and structural resolutions.
    pub fn build_heatmap_model(
        &self,
        records: Vec<TokenSensitivityRecord>,
        doc_ids: Vec<String>,
        corpus_hash: &str,
        seed: u64,
        min_observations: usize,
    ) -> SensitivityHeatmapModel {
        let total_tokens_profiled = records.len();
        let total_trials_executed = records.iter().map(|r| r.n_trials).sum();

        let bins_10 = self.aggregate_position_bins(&records, 10, min_observations);
        let bins_20 = self.aggregate_position_bins(&records, 20, min_observations);
        let bins_50 = self.aggregate_position_bins(&records, 50, min_observations);
        let sent_bins = self.aggregate_sentence_bins(&records, min_observations);
        let para_bins = self.aggregate_paragraph_bins(&records, min_observations);
        let pos_stats = self.aggregate_pos_stats(&records, min_observations);
        let domain_stats = self.aggregate_domain_stats(&records, min_observations);

        let mut model = SensitivityHeatmapModel {
            source_corpus_hash: corpus_hash.to_string(),
            source_document_ids: doc_ids,
            random_seed: seed,
            configuration: "default_synthid_k2_key428917492".to_string(),
            detector_configuration: "synthid_k=2_threshold=3.0_alpha=0.05".to_string(),
            creation_timestamp: Utc::now().to_rfc3339(),
            heatmap_hash: String::new(),
            min_observations,
            total_tokens_profiled,
            total_trials_executed,
            position_bins_10: bins_10,
            position_bins_20: bins_20,
            position_bins_50: bins_50,
            sentence_position_bins: sent_bins,
            paragraph_position_bins: para_bins,
            pos_stats,
            domain_stats,
            token_records: records,
        };

        // Compute deterministic SHA-256 hash
        let json_bytes = serde_json::to_vec(&model).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(&json_bytes);
        model.heatmap_hash = format!("{:x}", hasher.finalize());

        model
    }

    fn aggregate_position_bins(
        &self,
        records: &[TokenSensitivityRecord],
        num_bins: usize,
        min_obs: usize,
    ) -> Vec<PositionalBinStats> {
        let mut bins = Vec::new();
        let bin_width = 1.0 / num_bins as f64;

        for b in 0..num_bins {
            let start = b as f64 * bin_width;
            let end = (b + 1) as f64 * bin_width;
            let bin_records: Vec<&TokenSensitivityRecord> = records
                .iter()
                .filter(|r| {
                    if b == num_bins - 1 {
                        r.normalized_position >= start && r.normalized_position <= end
                    } else {
                        r.normalized_position >= start && r.normalized_position < end
                    }
                })
                .collect();

            let bin_id = format!("pos_bin_{:02}_of_{}", b + 1, num_bins);
            bins.push(self.compute_bin_stats(bin_id, b, start, end, &bin_records, min_obs));
        }

        bins
    }

    fn aggregate_sentence_bins(
        &self,
        records: &[TokenSensitivityRecord],
        min_obs: usize,
    ) -> Vec<PositionalBinStats> {
        let max_sent = records.iter().map(|r| r.sentence_index).max().unwrap_or(0);
        let num_bins = (max_sent + 1).min(15);
        let mut bins = Vec::new();

        for s in 0..num_bins {
            let bin_records: Vec<&TokenSensitivityRecord> = records
                .iter()
                .filter(|r| {
                    if s == num_bins - 1 {
                        r.sentence_index >= s
                    } else {
                        r.sentence_index == s
                    }
                })
                .collect();

            let bin_id = format!("sentence_{:02}", s);
            bins.push(self.compute_bin_stats(
                bin_id,
                s,
                s as f64,
                (s + 1) as f64,
                &bin_records,
                min_obs,
            ));
        }

        bins
    }

    fn aggregate_paragraph_bins(
        &self,
        records: &[TokenSensitivityRecord],
        min_obs: usize,
    ) -> Vec<PositionalBinStats> {
        let max_para = records.iter().map(|r| r.paragraph_index).max().unwrap_or(0);
        let num_bins = (max_para + 1).min(10);
        let mut bins = Vec::new();

        for p in 0..num_bins {
            let bin_records: Vec<&TokenSensitivityRecord> = records
                .iter()
                .filter(|r| {
                    if p == num_bins - 1 {
                        r.paragraph_index >= p
                    } else {
                        r.paragraph_index == p
                    }
                })
                .collect();

            let bin_id = format!("paragraph_{:02}", p);
            bins.push(self.compute_bin_stats(
                bin_id,
                p,
                p as f64,
                (p + 1) as f64,
                &bin_records,
                min_obs,
            ));
        }

        bins
    }

    fn aggregate_pos_stats(
        &self,
        records: &[TokenSensitivityRecord],
        min_obs: usize,
    ) -> HashMap<String, PositionalBinStats> {
        let mut by_pos: HashMap<String, Vec<&TokenSensitivityRecord>> = HashMap::new();
        for r in records {
            let coarse = format!("{:?}", r.pos.coarse_pos());
            by_pos.entry(coarse).or_default().push(r);
        }

        let mut res = HashMap::new();
        for (idx, (pos_name, pos_recs)) in by_pos.into_iter().enumerate() {
            res.insert(
                pos_name.clone(),
                self.compute_bin_stats(pos_name, idx, 0.0, 1.0, &pos_recs, min_obs),
            );
        }

        res
    }

    fn aggregate_domain_stats(
        &self,
        records: &[TokenSensitivityRecord],
        min_obs: usize,
    ) -> HashMap<String, PositionalBinStats> {
        let mut by_dom: HashMap<String, Vec<&TokenSensitivityRecord>> = HashMap::new();
        for r in records {
            by_dom.entry(r.domain_class.clone()).or_default().push(r);
        }

        let mut res = HashMap::new();
        for (idx, (dom_name, dom_recs)) in by_dom.into_iter().enumerate() {
            res.insert(
                dom_name.clone(),
                self.compute_bin_stats(dom_name, idx, 0.0, 1.0, &dom_recs, min_obs),
            );
        }

        res
    }

    fn compute_bin_stats(
        &self,
        bin_id: String,
        bin_idx: usize,
        start_norm: f64,
        end_norm: f64,
        records: &[&TokenSensitivityRecord],
        min_obs: usize,
    ) -> PositionalBinStats {
        let token_count = records.len();
        let total_trials: usize = records.iter().map(|r| r.n_trials).sum();

        if records.is_empty() {
            return PositionalBinStats {
                bin_id,
                bin_index: bin_idx,
                start_normalized: start_norm,
                end_normalized: end_norm,
                token_count: 0,
                mean_delta_z: 0.0,
                median_delta_z: 0.0,
                std_delta_z: 0.0,
                mean_abs_delta_z: 0.0,
                clean_transition_rate: 0.0,
                standard_error: 0.0,
                ci95_low: 0.0,
                ci95_high: 0.0,
                n_trials: 0,
                status: SensitivityObservationStatus::UnderSampled,
            };
        }

        let dz_vals: Vec<f64> = records.iter().map(|r| r.mean_delta_z).collect();
        let mean_dz = dz_vals.iter().sum::<f64>() / token_count as f64;

        let mut sorted_dz = dz_vals.clone();
        sorted_dz.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let median_dz = if token_count % 2 == 1 {
            sorted_dz[token_count / 2]
        } else {
            (sorted_dz[token_count / 2 - 1] + sorted_dz[token_count / 2]) / 2.0
        };

        let variance = if token_count > 1 {
            dz_vals
                .iter()
                .map(|dz| (dz - mean_dz).powi(2))
                .sum::<f64>()
                / (token_count - 1) as f64
        } else {
            0.0
        };
        let std_dz = variance.sqrt();
        let se = if token_count > 0 {
            std_dz / (token_count as f64).sqrt()
        } else {
            0.0
        };
        let mean_abs_dz =
            records.iter().map(|r| r.mean_abs_delta_z).sum::<f64>() / token_count as f64;
        let clean_rate =
            records.iter().map(|r| r.clean_transition_rate).sum::<f64>() / token_count as f64;

        let status = if token_count >= min_obs {
            SensitivityObservationStatus::Valid
        } else {
            SensitivityObservationStatus::UnderSampled
        };

        PositionalBinStats {
            bin_id,
            bin_index: bin_idx,
            start_normalized: start_norm,
            end_normalized: end_norm,
            token_count,
            mean_delta_z: mean_dz,
            median_delta_z: median_dz,
            std_delta_z: std_dz,
            mean_abs_delta_z: mean_abs_dz,
            clean_transition_rate: clean_rate,
            standard_error: se,
            ci95_low: mean_dz - 1.96 * se,
            ci95_high: mean_dz + 1.96 * se,
            n_trials: total_trials,
            status,
        }
    }

    /// Saves the frozen heatmap model as JSON.
    pub fn save_heatmap_json(&self, model: &SensitivityHeatmapModel, path: &str) -> io::Result<()> {
        let json_str = serde_json::to_string_pretty(model)
            .map_err(io::Error::other)?;
        fs::write(path, json_str)
    }

    /// Loads a frozen sensitivity heatmap model from JSON.
    pub fn load_heatmap_json(path: &str) -> io::Result<SensitivityHeatmapModel> {
        let content = fs::read_to_string(path)?;
        serde_json::from_str(&content).map_err(io::Error::other)
    }

    /// Exports token sensitivity records as CSV.
    pub fn export_heatmap_csv(records: &[TokenSensitivityRecord], path: &str) -> io::Result<()> {
        let mut lines = Vec::new();
        lines.push("document_id,token_index,sentence_index,paragraph_index,token_text,lemma,pos,domain_class,character_offset,normalized_position,mean_delta_z,median_delta_z,std_delta_z,mean_abs_delta_z,clean_transition_rate,standard_error,ci95_low,ci95_high,n_trials,status".to_string());
        for r in records {
            lines.push(format!(
                "{},{},{},{},\"{}\",\"{}\",{:?},{},{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{},{:?}",
                r.document_id, r.token_index, r.sentence_index, r.paragraph_index,
                r.token_text.replace('"', "\"\""), r.lemma.replace('"', "\"\""),
                r.pos, r.domain_class, r.character_offset, r.normalized_position,
                r.mean_delta_z, r.median_delta_z, r.std_delta_z, r.mean_abs_delta_z,
                r.clean_transition_rate, r.standard_error, r.ci95_low, r.ci95_high,
                r.n_trials, r.status
            ));
        }
        fs::write(path, lines.join("\n"))
    }
}
