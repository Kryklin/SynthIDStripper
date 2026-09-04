use crate::detector::{
    GptZeroClient, GptZeroDocumentResult, SynthIdConfig, SynthIdDetectionResult, SynthIdDetector,
};
use crate::engine::LexiconStripper;
use crate::types::{
    CategoryJsdMetrics, DistributionMode, StripperConfig, TransformResult, TypingNoiseConfig,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AblationRunEntry {
    pub label: String,
    pub description: String,
    pub lexical_enabled: bool,
    pub terminology_enabled: bool,
    pub syntax_enabled: bool,
    pub cadence_enabled: bool,
    pub function_words_enabled: bool,
    pub typing_enabled: bool,
    pub transform_result: TransformResult,
    pub synthid_result: SynthIdDetectionResult,
    pub gptzero_result: Option<GptZeroDocumentResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AblationMatrixReport {
    pub input_text_sha256: String,
    pub runs: Vec<AblationRunEntry>,
}

impl LexiconStripper {
    /// Executes the full controlled ablation experiment across baseline and modular layers
    pub fn run_ablation_matrix(
        &self,
        input_text: &str,
        mode: DistributionMode,
        seed: Option<u64>,
        synthid_config: Option<SynthIdConfig>,
        gptzero_client: Option<&GptZeroClient>,
    ) -> AblationMatrixReport {
        let configs = [
            (
                "A",
                "Baseline (No Transformations)",
                false,
                false,
                false,
                false,
                false,
                false,
            ),
            ("B", "WordNet Lexical Only (No Terminology)", true, false, false, false, false, false),
            ("C", "WordNet + IATE/EuroVoc Terminology", true, true, false, false, false, false),
            ("D", "Typing Noise Only", false, false, false, false, false, true),
            (
                "E",
                "Lexical + Terminology + Typing Noise",
                true,
                true,
                false,
                false,
                false,
                true,
            ),
            (
                "F",
                "Syntax Restructuring Only",
                false,
                false,
                true,
                false,
                false,
                false,
            ),
            (
                "G",
                "Cadence Variation Only",
                false,
                false,
                false,
                true,
                false,
                false,
            ),
            ("H", "Function Words Only", false, false, false, false, true, false),
            ("I", "Full Pipeline + Terminology", true, true, true, true, true, true),
        ];

        let synthid_detector = SynthIdDetector::new(synthid_config.unwrap_or_default());
        let mut runs = Vec::new();

        for (label, desc, lex, term, syn, cad, func, typing) in configs {
            let typing_cfg = if typing {
                Some(TypingNoiseConfig {
                    rate: 0.03, // Standard 3% baseline typing noise for ablation
                    seed,
                    ..Default::default()
                })
            } else {
                None
            };

            let term_cfg = if term {
                Some(crate::terminology::TerminologyConfig {
                    enabled: true,
                    ..Default::default()
                })
            } else {
                None
            };

            let config = StripperConfig {
                mode,
                seed,
                enable_lexical: lex,
                enable_syntax: syn,
                enable_cadence: cad,
                enable_function_words: func,
                typing_noise_config: typing_cfg,
                terminology_config: term_cfg,
                ..Default::default()
            };

            let transform_res = if !lex && !syn && !cad && !func && !typing {
                // Baseline: identical text with zero transforms
                let mut base_res = self.transform(input_text, &config);
                base_res.transformed_text = input_text.to_string();
                base_res.report.replaced_words = 0;
                base_res.report.replacement_rate = 0.0;
                base_res.report.lexical_turnover = 0.0;
                base_res.report.jsd_metrics = CategoryJsdMetrics {
                    total_jsd: 0.0,
                    content_words_jsd: 0.0,
                    nouns_jsd: 0.0,
                    verbs_jsd: 0.0,
                    adjectives_jsd: 0.0,
                    adverbs_jsd: 0.0,
                    function_words_jsd: 0.0,
                };
                base_res
            } else {
                self.transform(input_text, &config)
            };

            let synthid_res = synthid_detector.detect(&transform_res.transformed_text);

            // Call GPTZero if client is configured
            let gptzero_res = if let Some(client) = gptzero_client {
                match client.predict_text(&transform_res.transformed_text) {
                    Ok(resp) => resp.documents.into_iter().next(),
                    Err(e) => {
                        eprintln!("[GPTZero API Warning] Run {}: {}", label, e);
                        None
                    }
                }
            } else {
                None
            };

            runs.push(AblationRunEntry {
                label: label.to_string(),
                description: desc.to_string(),
                lexical_enabled: lex,
                terminology_enabled: term,
                syntax_enabled: syn,
                cadence_enabled: cad,
                function_words_enabled: func,
                typing_enabled: typing,
                transform_result: transform_res,
                synthid_result: synthid_res,
                gptzero_result: gptzero_res,
            });
        }

        let input_sha = runs
            .first()
            .map(|r| r.transform_result.report.input_sha256.clone())
            .unwrap_or_default();

        AblationMatrixReport {
            input_text_sha256: input_sha,
            runs,
        }
    }
}
