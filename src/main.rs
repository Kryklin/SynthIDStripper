use chrono::Utc;
use clap::{CommandFactory, Parser};
use colored::*;
use lexicon_stripper::{
    AblationMatrixReport, DetectorImportRecord, DistributionMode, Domain, EmpiricalLog,
    LexiconStripper, SelectionStrategy, SensitivityAnalyzer, SensitivityConfig,
    SensitivitySamplingPolicy, StripperConfig, SynthIdConfig, SynthIdDetector, TransformReport,
    TransformResult, TypingErrorClass, TypingNoiseConfig,
};
use std::fs;
use std::io::{self, Read, Write};

#[derive(Parser, Debug)]
#[command(
    name = "lexicon_stripper",
    about = "Production-grade linguistic perturbation & watermark robustness engine without AI in the loop",
    version = "1.0.0",
    after_help = "SPECIALIZED HELP TOPICS:\n  help typos           Detailed guide to the human typing noise & biomechanical error simulator\n  help domains         Directory of all 29 supported specialized linguistic domains & protected jargon\n  help watermark       Guide to SynthID & Kirchenbauer statistical watermark analysis\n\nRun 'lexicon_stripper help typos' or 'lexicon_stripper --help-typos' for full typing noise documentation."
)]
struct Cli {
    /// Input text string (if not provided, reads from --file or stdin)
    #[arg(value_name = "TEXT")]
    text: Option<String>,

    /// Read input from a file
    #[arg(
        short = 'f',
        long = "file",
        value_name = "PATH",
        help_heading = "Input / Output Options"
    )]
    file: Option<String>,

    /// Write transformed text to output file (default: stdout)
    #[arg(
        short = 'o',
        long = "output",
        value_name = "PATH",
        help_heading = "Input / Output Options"
    )]
    output: Option<String>,

    /// Write comprehensive empirical metrics & transformation log to JSON file
    #[arg(
        short = 'l',
        long = "log",
        value_name = "PATH",
        help_heading = "Input / Output Options"
    )]
    log: Option<String>,

    /// Write human-readable text report to a file
    #[arg(
        long = "log-text",
        value_name = "PATH",
        help_heading = "Input / Output Options"
    )]
    log_text: Option<String>,

    /// Unique experiment ID string for tracking & dose-response curves (e.g. lexical_015_seed_12345)
    #[arg(
        long = "experiment-id",
        value_name = "ID",
        help_heading = "Input / Output Options"
    )]
    experiment_id: Option<String>,

    /// Replicate sequence number for repeated experiment batches (default: 1)
    #[arg(
        long = "replicate",
        default_value_t = 1,
        help_heading = "Input / Output Options"
    )]
    replicate: usize,

    /// Print formatted empirical transformation report to stderr
    #[arg(short = 'r', long = "report", short_alias = 'v', aliases = ["verbose"], help_heading = "Input / Output Options")]
    report: bool,

    /// Show colorized terminal diff of modifications (printed to stderr)
    #[arg(short = 'd', long = "diff", help_heading = "Input / Output Options")]
    diff: bool,

    /// Output full transformation result and metrics in JSON format to stdout
    #[arg(short = 'j', long = "json", help_heading = "Input / Output Options")]
    json: bool,

    /// Launch interactive REPL mode
    #[arg(
        short = 'i',
        long = "interactive",
        help_heading = "Input / Output Options"
    )]
    interactive: bool,

    // --- PRIMARY LAYER: Lexical Resampling (Always Enabled by Default) ---
    /// Lexical frequency distribution mode
    #[arg(short = 'm', long = "mode", value_enum, default_value_t = DistributionMode::Human, help_heading = "Primary Core: Lexical Resampling")]
    mode: DistributionMode,

    /// Linguistic domain register mode (auto, general, finance, biomedical, computer_science, legal)
    #[arg(long = "domain", value_enum, default_value_t = Domain::Auto, help_heading = "Primary Core: Lexical Resampling")]
    domain: Domain,

    /// Display directory of all 29 supported specialized linguistic domains & protected jargon
    #[arg(
        long = "help-domains",
        aliases = ["domain-help", "help-domain"],
        help_heading = "Primary Core: Lexical Resampling"
    )]
    help_domains: bool,

    /// Advanced candidate selection strategy (baseline, uniform, context-neutral, efficiency-guided, conservative-efficiency)
    #[arg(long = "strategy", value_enum, default_value_t = SelectionStrategy::ConservativeEfficiency, help_heading = "Primary Core: Lexical Resampling")]
    strategy: SelectionStrategy,

    /// Replacement probability for eligible content words (0.0 to 1.0)
    #[arg(
        short = 'p',
        long = "prob",
        default_value_t = 0.30,
        help_heading = "Primary Core: Lexical Resampling"
    )]
    probability: f64,

    /// Minimum confidence threshold for semantic compatibility (0.0 to 1.0)
    #[arg(
        short = 'c',
        long = "min-confidence",
        default_value_t = 0.55,
        help_heading = "Primary Core: Lexical Resampling"
    )]
    min_confidence: f64,

    /// Sampling temperature (higher = flatter distribution, lower = sharper)
    #[arg(
        short = 't',
        long = "temperature",
        default_value_t = 1.0,
        help_heading = "Primary Core: Lexical Resampling"
    )]
    temperature: f64,

    /// Deterministic RNG master seed for reproducible transformations
    #[arg(
        short = 's',
        long = "seed",
        help_heading = "Primary Core: Lexical Resampling"
    )]
    seed: Option<u64>,

    /// Number of iterative transformation passes to run
    #[arg(
        short = 'n',
        long = "passes",
        default_value_t = 1,
        help_heading = "Primary Core: Lexical Resampling"
    )]
    passes: usize,

    /// Maximum candidate synonyms considered per target word
    #[arg(
        long = "max-candidates",
        default_value_t = 10,
        help_heading = "Primary Core: Lexical Resampling"
    )]
    max_candidates: usize,

    /// Disable the primary lexical resampling layer
    #[arg(long = "no-lexical", help_heading = "Primary Core: Lexical Resampling")]
    no_lexical: bool,

    /// Allow re-replacing already transformed tokens in subsequent passes
    #[arg(
        long = "allow-re-replacement",
        help_heading = "Primary Core: Lexical Resampling"
    )]
    allow_re_replacement: bool,

    /// Query computational linguistics Thesaurus API for extended vocabulary
    #[arg(
        short = 'a',
        long = "api",
        help_heading = "Primary Core: Lexical Resampling"
    )]
    api: bool,

    // --- OPTIONAL ENHANCEMENT LAYERS ---
    /// Display comprehensive guide, biomechanical error classes, and examples for the human typing noise simulator
    #[arg(
        long = "help-typos",
        aliases = ["typo-help", "help-typo"],
        help_heading = "Optional Enhancement: Typing Noise"
    )]
    help_typos: bool,

    /// Probability that an eligible token receives human typing noise (0.0 to 1.0, e.g. 0.03 = 3%)
    #[arg(
        long = "typing-noise",
        default_value_t = 0.0,
        help_heading = "Optional Enhancement: Typing Noise"
    )]
    typing_noise: f64,

    /// Deterministic RNG seed for human typing noise reproducibility
    #[arg(
        long = "typing-seed",
        help_heading = "Optional Enhancement: Typing Noise"
    )]
    typing_seed: Option<u64>,

    /// Comma-separated list of allowed typing error classes (substitution,transposition,omission,insertion,duplication,temporal)
    #[arg(
        long = "typing-errors",
        value_delimiter = ',',
        help_heading = "Optional Enhancement: Typing Noise"
    )]
    typing_errors: Option<Vec<String>>,

    /// Enable controlled function-word and discourse connector alternatives
    #[arg(
        long = "function-words",
        help_heading = "Optional Enhancement: Function Words"
    )]
    function_words: bool,

    // --- EXPERIMENTAL SECONDARY MODIFIERS ---
    /// Enable experimental conservative syntactic restructuring
    #[arg(
        long = "syntax",
        alias = "experimental-syntax",
        help_heading = "Experimental Secondary Modifiers"
    )]
    syntax: bool,

    /// Enable experimental cadence & sentence pattern variation
    #[arg(
        long = "cadence",
        alias = "experimental-cadence",
        help_heading = "Experimental Secondary Modifiers"
    )]
    cadence: bool,

    /// Enable all transformation layers (lexical, function words, syntax, cadence)
    #[arg(long = "combined", help_heading = "Experimental Secondary Modifiers")]
    combined: bool,

    // --- VERIFICATION, WATERMARKING & LAB ANALYSIS ---
    /// Secret key seed for SynthID watermarking & detection (default: 428917492)
    #[arg(
        long = "synthid-key",
        default_value_t = 428917492,
        help_heading = "Verification & Watermark Analysis"
    )]
    synthid_key: u64,

    /// Context length k for SynthID watermarking & detection (default: 2)
    #[arg(
        long = "synthid-k",
        default_value_t = 2,
        help_heading = "Verification & Watermark Analysis"
    )]
    synthid_k: usize,

    /// Perform standalone SynthID statistical watermark detection scan on input text
    #[arg(
        long = "synthid-detect",
        help_heading = "Verification & Watermark Analysis"
    )]
    synthid_detect: bool,

    /// Watermark input text using SynthID tournament sampling with secret key
    #[arg(
        long = "synthid-watermark",
        help_heading = "Verification & Watermark Analysis"
    )]
    synthid_watermark: bool,

    // --- KIRCHENBAUER (MARYLAND) WATERMARKING & DETECTION ---
    /// Secret key seed for Kirchenbauer watermarking & detection (default: 133742069)
    #[arg(
        long = "kirchenbauer-key",
        default_value_t = 133742069,
        help_heading = "Kirchenbauer (Maryland) Watermark Analysis"
    )]
    kirchenbauer_key: u64,

    /// Context length k for Kirchenbauer watermark (default: 1)
    #[arg(
        long = "kirchenbauer-k",
        default_value_t = 1,
        help_heading = "Kirchenbauer (Maryland) Watermark Analysis"
    )]
    kirchenbauer_k: usize,

    /// Green-list fraction gamma for Kirchenbauer watermark (default: 0.50)
    #[arg(
        long = "kirchenbauer-gamma",
        default_value_t = 0.50,
        help_heading = "Kirchenbauer (Maryland) Watermark Analysis"
    )]
    kirchenbauer_gamma: f64,

    /// Perform standalone Kirchenbauer statistical watermark detection scan on input text
    #[arg(
        long = "kirchenbauer-detect",
        help_heading = "Kirchenbauer (Maryland) Watermark Analysis"
    )]
    kirchenbauer_detect: bool,

    /// Watermark input text using Kirchenbauer green-list preference
    #[arg(
        long = "kirchenbauer-watermark",
        help_heading = "Kirchenbauer (Maryland) Watermark Analysis"
    )]
    kirchenbauer_watermark: bool,

    /// Run full 8-way controlled ablation experiment matrix (Runs A through H)
    #[arg(long = "ablation", help_heading = "Verification & Watermark Analysis")]
    ablation: bool,

    /// Evaluate each ablation run using the official GPTZero API
    #[arg(
        long = "evaluate-gptzero",
        help_heading = "Verification & Watermark Analysis"
    )]
    evaluate_gptzero: bool,

    /// GPTZero API key (can also be supplied via GPTZERO_API_KEY environment variable)
    #[arg(
        long = "gptzero-api-key",
        env = "GPTZERO_API_KEY",
        value_name = "KEY",
        help_heading = "Verification & Watermark Analysis"
    )]
    gptzero_api_key: Option<String>,

    /// Path to external detector JSON output (e.g. GPTZero result) for correlation
    /// Path to external detector JSON output (e.g. GPTZero result) for correlation
    #[arg(
        long = "detector-import",
        value_name = "PATH",
        help_heading = "Verification & Watermark Analysis"
    )]
    detector_import: Option<String>,

    // --- DOMAIN TERMINOLOGY LAYER (IATE / EuroVoc) ---
    /// Enable authoritative IATE / EuroVoc domain terminology validation layer
    #[arg(
        long = "terminology",
        help_heading = "Domain Terminology Layer (IATE / EuroVoc)"
    )]
    terminology: bool,

    /// Terminology data source mode (local or api)
    #[arg(
        long = "terminology-source",
        default_value = "local",
        help_heading = "Domain Terminology Layer (IATE / EuroVoc)"
    )]
    terminology_source: String,

    /// Path to custom terminology JSON database file
    #[arg(
        long = "terminology-db",
        value_name = "PATH",
        help_heading = "Domain Terminology Layer (IATE / EuroVoc)"
    )]
    terminology_db: Option<String>,

    /// Target terminology domain filter (e.g. finance, biomedical, computer_science, legal)
    #[arg(
        long = "terminology-domain",
        value_name = "DOMAIN",
        help_heading = "Domain Terminology Layer (IATE / EuroVoc)"
    )]
    terminology_domain: Option<String>,

    /// Refresh / synchronize the local terminology cache from remote authoritative endpoint
    #[arg(
        long = "terminology-refresh",
        help_heading = "Domain Terminology Layer (IATE / EuroVoc)"
    )]
    terminology_refresh: bool,

    /// Print comprehensive terminology diagnostics (matched terms, relations, rejected candidates)
    #[arg(
        long = "terminology-debug",
        help_heading = "Domain Terminology Layer (IATE / EuroVoc)"
    )]
    terminology_debug: bool,

    /// Strictly protect all identified domain terminology terms from modification
    #[arg(
        long = "protect-domain-terms",
        help_heading = "Domain Terminology Layer (IATE / EuroVoc)"
    )]
    protect_domain_terms: bool,

    // --- TOKEN-POSITION SENSITIVITY ANALYSIS & HEATMAP SAMPLING ---
    /// Run token-level empirical sensitivity profiling on the input document
    #[arg(
        long = "analyze-sensitivity",
        help_heading = "Token-Position Sensitivity Analysis"
    )]
    analyze_sensitivity: bool,

    /// Number of candidate perturbation trials per token position (default: 5)
    #[arg(
        long = "sensitivity-samples",
        default_value_t = 5,
        help_heading = "Token-Position Sensitivity Analysis"
    )]
    sensitivity_samples: usize,

    /// Minimum trials to classify a token sensitivity observation as VALID (default: 3)
    #[arg(
        long = "min-observations",
        default_value_t = 3,
        help_heading = "Token-Position Sensitivity Analysis"
    )]
    min_observations: usize,

    /// Controlled sampling policy for equal-budget sensitivity experiments (uniform, high_sensitivity, low_sensitivity)
    #[arg(
        long = "sensitivity-policy",
        value_enum,
        default_value_t = SensitivitySamplingPolicy::Uniform,
        help_heading = "Token-Position Sensitivity Analysis"
    )]
    sensitivity_policy: SensitivitySamplingPolicy,

    /// Path to frozen sensitivity heatmap JSON file for guided sampling
    #[arg(
        long = "sensitivity-heatmap",
        value_name = "PATH",
        help_heading = "Token-Position Sensitivity Analysis"
    )]
    sensitivity_heatmap: Option<String>,

    /// Path to save generated sensitivity heatmap JSON model
    #[arg(
        long = "save-heatmap",
        value_name = "PATH",
        help_heading = "Token-Position Sensitivity Analysis"
    )]
    save_heatmap: Option<String>,

    /// Path to export token-level sensitivity records as CSV
    #[arg(
        long = "export-sensitivity-csv",
        value_name = "PATH",
        help_heading = "Token-Position Sensitivity Analysis"
    )]
    export_sensitivity_csv: Option<String>,

    /// Exact target edit budget (number of token replacements)
    #[arg(
        long = "edit-budget",
        help_heading = "Token-Position Sensitivity Analysis"
    )]
    edit_budget: Option<usize>,

    /// Path to frozen composite sensitivity model JSON file
    #[arg(
        long = "composite-model",
        value_name = "PATH",
        help_heading = "Token-Position Sensitivity Analysis"
    )]
    composite_model: Option<String>,
}

fn main() -> io::Result<()> {
    // 0. Intercept custom help topic commands before Clap parsing
    let raw_args: Vec<String> = std::env::args().collect();
    if raw_args.len() > 1 {
        let first = raw_args[1].to_ascii_lowercase();
        if first == "help" || first == "--help" || first == "-h" {
            if let Some(topic) = raw_args.get(2) {
                match topic.to_ascii_lowercase().as_str() {
                    "typo" | "typos" | "typing" | "noise" => {
                        print_typo_help();
                        return Ok(());
                    }
                    "domain" | "domains" | "lexicon" => {
                        print_domain_help();
                        return Ok(());
                    }
                    "watermark" | "watermarks" | "synthid" | "kirchenbauer" => {
                        print_watermark_help();
                        return Ok(());
                    }
                    _ => {
                        eprintln!(
                            "{}",
                            format!("Unknown help topic '{}'. Available topics: typos, domains, watermark\n", topic)
                                .yellow()
                        );
                        print_general_help_with_topics();
                        return Ok(());
                    }
                }
            } else if first == "help" {
                print_general_help_with_topics();
                return Ok(());
            }
        } else if matches!(first.as_str(), "typos" | "typo" | "typing" | "--help-typos" | "--typo-help" | "help-typos" | "help-typo") {
            print_typo_help();
            return Ok(());
        } else if matches!(first.as_str(), "domains" | "domain" | "--help-domains" | "--domain-help") {
            print_domain_help();
            return Ok(());
        } else if matches!(first.as_str(), "watermark" | "watermarks") {
            print_watermark_help();
            return Ok(());
        }
    }

    let cli = Cli::parse();

    if cli.help_typos {
        print_typo_help();
        return Ok(());
    }

    if cli.help_domains {
        print_domain_help();
        return Ok(());
    }

    let mut stripper = LexiconStripper::new();

    // Terminology Refresh Mode
    if cli.terminology_refresh {
        let path = cli.terminology_db.as_deref().map(std::path::Path::new);
        match stripper.terminology_client.refresh_cache(path, &stripper.terminology_db) {
            Ok(db) => {
                eprintln!(
                    "{}",
                    format!(
                        "[Terminology Refresh] Success: Synchronized {} terminology entries (Version: {}).",
                        db.terms.len(),
                        db.version
                    )
                    .green()
                    .bold()
                );
            }
            Err(e) => {
                eprintln!("{}", format!("[Terminology Refresh] Error: {}", e).red().bold());
            }
        }
        return Ok(());
    }

    // Load custom terminology database if specified
    if let Some(ref db_path) = cli.terminology_db {
        match lexicon_stripper::TerminologyDb::from_file(db_path) {
            Ok(custom_db) => {
                eprintln!(
                    "{}",
                    format!(
                        "[Terminology] Loaded custom database from {} ({} terms, Hash: {}).",
                        db_path,
                        custom_db.terms.len(),
                        &custom_db.get_hash()[..12]
                    )
                    .cyan()
                );
                stripper.terminology_db = custom_db;
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    format!("[Terminology Warning] Could not load {}: {} (Falling back to embedded IATE database).", db_path, e)
                        .yellow()
                );
            }
        }
    }

    if cli.interactive {
        return run_interactive(&stripper);
    }

    // 1. Resolve input text
    let input_text = if let Some(text) = cli.text {
        text
    } else if let Some(ref path) = cli.file {
        fs::read_to_string(path)?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    if input_text.trim().is_empty() {
        eprintln!("{}", "Error: No input text provided. Provide text as argument, via -f <path>, or through stdin.".red().bold());
        return Ok(());
    }

    // Token-Level Empirical Sensitivity Profiling Mode
    if cli.analyze_sensitivity {
        let det_cfg = SynthIdConfig {
            key: cli.synthid_key,
            context_length: cli.synthid_k,
            detection_threshold: 3.0,
        };
        let analyzer = SensitivityAnalyzer::new(det_cfg);
        let doc_id = cli
            .file.as_deref()
            .unwrap_or("cli_input");
        let seed = cli.seed.unwrap_or(42);

        let records = analyzer.profile_document(
            doc_id,
            &input_text,
            if cli.domain != Domain::Auto {
                Some(cli.domain)
            } else {
                None
            },
            cli.sensitivity_samples,
            cli.min_observations,
            seed,
        );

        if let Some(ref csv_path) = cli.export_sensitivity_csv {
            let _ = SensitivityAnalyzer::export_heatmap_csv(&records, csv_path);
            eprintln!(
                "{}",
                format!("[Sensitivity] Exported {} records to {}", records.len(), csv_path).cyan()
            );
        }

        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(input_text.as_bytes());
        let corpus_hash = format!("{:x}", hasher.finalize());

        let model = analyzer.build_heatmap_model(
            records.clone(),
            vec![doc_id.to_string()],
            &corpus_hash,
            seed,
            cli.min_observations,
        );

        if let Some(ref save_path) = cli.save_heatmap {
            let _ = analyzer.save_heatmap_json(&model, save_path);
            eprintln!(
                "{}",
                format!(
                    "[Sensitivity] Saved heatmap model (Hash: {}) to {}",
                    if model.heatmap_hash.len() >= 12 { &model.heatmap_hash[..12] } else { &model.heatmap_hash },
                    save_path
                )
                .green()
            );
        }

        // Print Summary Report to stdout/stderr
        println!("\n{}", "=========================================================================================".cyan().bold());
        println!("{}", "            EMPIRICAL TOKEN-POSITION SENSITIVITY HEATMAP REPORT".cyan().bold());
        println!("{}\n", "=========================================================================================".cyan().bold());
        println!("Document: {} | Total Tokens Profiled: {} | Total Trials: {}", doc_id, model.total_tokens_profiled, model.total_trials_executed);
        println!("Heatmap Hash: {}", model.heatmap_hash);

        println!("\n{}", "--- NORMALIZED DOCUMENT POSITION BINS (10 Bins) ---".yellow().bold());
        println!("{:<18} | {:<14} | {:<12} | {:<12} | {:<10} | {:<8}", "Position Bin", "Range", "Mean ΔZ", "Mean |ΔZ|", "Clean Rate", "Status");
        println!("{:-<18}-+-{:-<14}-+-{:-<12}-+-{:-<12}-+-{:-<10}-+-{:-<8}", "", "", "", "", "", "");
        for b in &model.position_bins_10 {
            println!(
                "{:<18} | [{:.2}, {:.2})   | {:+10.4}  | {:10.4}   | {:8.1}%  | {:?}",
                b.bin_id, b.start_normalized, b.end_normalized, b.mean_delta_z, b.mean_abs_delta_z, b.clean_transition_rate * 100.0, b.status
            );
        }

        println!("\n{}", "--- POS CATEGORY SENSITIVITY ---".yellow().bold());
        println!("{:<15} | {:<8} | {:<12} | {:<12} | {:<10}", "POS Category", "Tokens", "Mean ΔZ", "Mean |ΔZ|", "Clean Rate");
        println!("{:-<15}-+-{:-<8}-+-{:-<12}-+-{:-<12}-+-{:-<10}", "", "", "", "", "");
        for (pos_name, b) in &model.pos_stats {
            println!(
                "{:<15} | {:<8} | {:+10.4}  | {:10.4}   | {:8.1}%",
                pos_name, b.token_count, b.mean_delta_z, b.mean_abs_delta_z, b.clean_transition_rate * 100.0
            );
        }
        println!();

        return Ok(());
    }

    // Standalone SynthID Detection Mode
    if cli.synthid_detect {
        let detector = SynthIdDetector::new(SynthIdConfig {
            key: cli.synthid_key,
            context_length: cli.synthid_k,
            detection_threshold: 3.0,
        });
        let result = detector.detect(&input_text);
        print_synthid_report(&result);
        return Ok(());
    }

    // Standalone SynthID Watermark Mode
    if cli.synthid_watermark {
        let tokenizer = lexicon_stripper::Tokenizer::new();
        let pos_tagger = lexicon_stripper::PosTagger::new();
        let lemmatizer = lexicon_stripper::Lemmatizer::new();
        let inflector = lexicon_stripper::Inflector::new();
        let synset_db = lexicon_stripper::lexicon::SynsetDb::new();
        let disambiguator = lexicon_stripper::SenseDisambiguator::new();

        let watermarker = lexicon_stripper::SynthIdWatermarker::new(SynthIdConfig {
            key: cli.synthid_key,
            context_length: cli.synthid_k,
            detection_threshold: 3.0,
        });

        let mut tokens = tokenizer.tokenize(&input_text);
        pos_tagger.tag_tokens(&mut tokens);
        for token in &mut tokens {
            if token.is_word() {
                token.lemma = Some(lemmatizer.lemmatize(&token.text, token.pos));
            }
        }

        let k = cli.synthid_k;
        let mut history: Vec<String> = Vec::new();

        for i in 0..tokens.len() {
            if !tokens[i].is_word() {
                continue;
            }
            let token_text = tokens[i].text.clone();
            let token_lemma = tokens[i]
                .lemma
                .clone()
                .unwrap_or_else(|| token_text.to_lowercase());
            let token_pos = tokens[i].pos;
            let token_casing = tokens[i].casing;
            let coarse_pos = token_pos.coarse_pos();

            if history.len() >= k && token_pos.is_content_word() {
                let synsets = synset_db.find_synsets(&token_lemma, coarse_pos);
                if !synsets.is_empty() {
                    let wsd_res = disambiguator.disambiguate(&tokens, i, &synsets, 10);
                    if let Some(synset) = wsd_res.best_synset {
                        let mut candidates = Vec::new();
                        for &lem in synset.lemmas {
                            let inflected = inflector.inflect(lem, token_pos, token_casing);
                            candidates.push(inflected);
                        }
                        if !candidates.contains(&token_text) {
                            candidates.push(token_text.clone());
                        }

                        let prev_tokens: Vec<&str> = history[history.len() - k..]
                            .iter()
                            .map(|s| s.as_str())
                            .collect();
                        let mut best_cand = token_text.clone();
                        let mut max_g = watermarker.compute_g_value(&prev_tokens, &token_text);
                        for cand in &candidates {
                            let g = watermarker.compute_g_value(&prev_tokens, cand);
                            if g > max_g {
                                max_g = g;
                                best_cand = cand.clone();
                            }
                        }
                        tokens[i].text = best_cand;
                    }
                }
            }
            history.push(tokens[i].text.to_lowercase());
        }

        let watermarked_text = tokens.iter().map(|t| t.text.as_str()).collect::<String>();
        if let Some(ref out_path) = cli.output {
            fs::write(out_path, &watermarked_text)?;
            eprintln!(
                "{}",
                format!("[SynthID] Watermarked text saved to {}", out_path).green()
            );
        } else {
            print!("{}", watermarked_text);
            if !watermarked_text.ends_with('\n') {
                println!();
            }
        }
        return Ok(());
    }

    // Standalone Kirchenbauer Detection Mode
    if cli.kirchenbauer_detect {
        let detector = lexicon_stripper::KirchenbauerDetector::new(lexicon_stripper::KirchenbauerConfig {
            key: cli.kirchenbauer_key,
            context_length: cli.kirchenbauer_k,
            gamma: cli.kirchenbauer_gamma,
            delta: 2.0,
            detection_threshold: 1.645,
        });
        let result = detector.detect(&input_text);
        print_kirchenbauer_report(&result);
        return Ok(());
    }

    // Standalone Kirchenbauer Watermark Mode
    if cli.kirchenbauer_watermark {
        let tokenizer = lexicon_stripper::Tokenizer::new();
        let pos_tagger = lexicon_stripper::PosTagger::new();
        let lemmatizer = lexicon_stripper::Lemmatizer::new();
        let inflector = lexicon_stripper::Inflector::new();
        let synset_db = lexicon_stripper::lexicon::SynsetDb::new();
        let disambiguator = lexicon_stripper::SenseDisambiguator::new();

        let _watermarker = lexicon_stripper::KirchenbauerWatermarker::new(lexicon_stripper::KirchenbauerConfig {
            key: cli.kirchenbauer_key,
            context_length: cli.kirchenbauer_k,
            gamma: cli.kirchenbauer_gamma,
            delta: 2.0,
            detection_threshold: 1.645,
        });

        let mut tokens = tokenizer.tokenize(&input_text);
        pos_tagger.tag_tokens(&mut tokens);
        for token in &mut tokens {
            if token.is_word() {
                token.lemma = Some(lemmatizer.lemmatize(&token.text, token.pos));
            }
        }

        let k = cli.kirchenbauer_k;
        let mut history: Vec<String> = Vec::new();

        for i in 0..tokens.len() {
            if !tokens[i].is_word() {
                continue;
            }
            let token_text = tokens[i].text.clone();
            let token_lemma = tokens[i]
                .lemma
                .clone()
                .unwrap_or_else(|| token_text.to_lowercase());
            let token_pos = tokens[i].pos;
            let token_casing = tokens[i].casing;
            let coarse_pos = token_pos.coarse_pos();

            if history.len() >= k && token_pos.is_content_word() {
                let synsets = synset_db.find_synsets(&token_lemma, coarse_pos);
                if !synsets.is_empty() {
                    let wsd_res = disambiguator.disambiguate(&tokens, i, &synsets, 10);
                    if let Some(synset) = wsd_res.best_synset {
                        let mut candidates = Vec::new();
                        for &lem in synset.lemmas {
                            let inflected = inflector.inflect(lem, token_pos, token_casing);
                            candidates.push(inflected);
                        }
                        if !candidates.contains(&token_text) {
                            candidates.push(token_text.clone());
                        }

                        let prev_tokens: Vec<&str> = history[history.len() - k..]
                            .iter()
                            .map(|s| s.as_str())
                            .collect();
                        
                        let detector = lexicon_stripper::KirchenbauerDetector::new(lexicon_stripper::KirchenbauerConfig {
                            key: cli.kirchenbauer_key,
                            context_length: cli.kirchenbauer_k,
                            gamma: cli.kirchenbauer_gamma,
                            delta: 2.0,
                            detection_threshold: 1.645,
                        });

                        let green_cands: Vec<String> = candidates
                            .iter()
                            .filter(|c| detector.is_green_token(&prev_tokens, c))
                            .cloned()
                            .collect();

                        if let Some(best) = green_cands.first() {
                            tokens[i].text = best.clone();
                        }
                    }
                }
            }
            history.push(tokens[i].text.to_lowercase());
        }

        let watermarked_text = tokens.iter().map(|t| t.text.as_str()).collect::<String>();
        if let Some(ref out_path) = cli.output {
            fs::write(out_path, &watermarked_text)?;
            eprintln!(
                "{}",
                format!("[Kirchenbauer] Watermarked text saved to {}", out_path).green()
            );
        } else {
            print!("{}", watermarked_text);
            if !watermarked_text.ends_with('\n') {
                println!();
            }
        }
        return Ok(());
    }

    // Handle 8-Way Ablation Matrix
    if cli.ablation || cli.evaluate_gptzero {
        let gptzero_client = if cli.evaluate_gptzero {
            if let Some(ref key) = cli.gptzero_api_key {
                Some(lexicon_stripper::GptZeroClient::new(key.clone()))
            } else {
                eprintln!("{}", "[Warning] --evaluate-gptzero specified but no GPTZero API key provided via --gptzero-api-key or GPTZERO_API_KEY env var.".yellow());
                None
            }
        } else {
            None
        };

        let synthid_cfg = SynthIdConfig {
            key: cli.synthid_key,
            context_length: cli.synthid_k,
            detection_threshold: 3.0,
        };

        let ablation_report = stripper.run_ablation_matrix(
            &input_text,
            cli.mode,
            cli.seed,
            Some(synthid_cfg),
            gptzero_client.as_ref(),
        );

        if cli.json {
            let json_output =
                serde_json::to_string_pretty(&ablation_report).expect("Serialize ablation to JSON");
            println!("{}", json_output);
            return Ok(());
        }

        print_ablation_summary(&ablation_report);
        return Ok(());
    }

    // --- Pipeline Layer Configuration ---
    // Primary Core: Lexical Resampling (default: ON unless --no-lexical)
    let enable_lexical = if cli.combined {
        true
    } else { !cli.no_lexical };

    // Optional: Function Words
    let enable_function_words = cli.combined || cli.function_words;

    // Experimental Secondary: Syntax & Cadence
    let enable_syntax = cli.combined || cli.syntax;
    let enable_cadence = cli.combined || cli.cadence;

    // Optional: Typing Noise Simulator
    let typing_noise_config = if cli.typing_noise > 0.0 {
        let mut allowed = Vec::new();
        if let Some(ref errs) = cli.typing_errors {
            for err in errs {
                match err.to_lowercase().trim() {
                    "substitution" | "sub" => allowed.push(TypingErrorClass::Substitution),
                    "transposition" | "trans" => allowed.push(TypingErrorClass::Transposition),
                    "omission" | "omit" => allowed.push(TypingErrorClass::Omission),
                    "insertion" | "ins" => allowed.push(TypingErrorClass::Insertion),
                    "duplication" | "dup" => allowed.push(TypingErrorClass::Duplication),
                    "temporal" | "temp" => allowed.push(TypingErrorClass::Temporal),
                    _ => eprintln!("Warning: unknown typing error class '{}'", err),
                }
            }
        }
        if allowed.is_empty() {
            allowed = vec![
                TypingErrorClass::Substitution,
                TypingErrorClass::Transposition,
                TypingErrorClass::Omission,
                TypingErrorClass::Insertion,
                TypingErrorClass::Duplication,
                TypingErrorClass::Temporal,
            ];
        }

        Some(TypingNoiseConfig {
            rate: cli.typing_noise,
            seed: cli.typing_seed,
            allowed_errors: allowed,
            ..Default::default()
        })
    } else {
        None
    };

    // Domain Terminology Layer Configuration
    let terminology_config = if cli.terminology
        || cli.terminology_domain.is_some()
        || cli.terminology_debug
        || cli.protect_domain_terms
    {
        Some(lexicon_stripper::TerminologyConfig {
            enabled: true,
            source: cli.terminology_source.clone(),
            db_path: cli.terminology_db.clone(),
            domain_filter: cli.terminology_domain.clone(),
            protect_domain_terms: cli.protect_domain_terms,
            debug: cli.terminology_debug,
        })
    } else {
        None
    };

    // External detector import (optional)
    let detector_import: Option<DetectorImportRecord> = if let Some(ref path) = cli.detector_import
    {
        let content = fs::read_to_string(path)?;
        serde_json::from_str(&content).ok()
    } else {
        None
    };

    // Sensitivity Sampling Configuration (optional)
    let sensitivity_config = if cli.sensitivity_policy != SensitivitySamplingPolicy::Uniform
        || cli.sensitivity_heatmap.is_some()
        || cli.composite_model.is_some()
        || cli.edit_budget.is_some()
    {
        Some(SensitivityConfig {
            policy: cli.sensitivity_policy,
            heatmap_path: cli.sensitivity_heatmap.clone(),
            composite_model_path: cli.composite_model.clone(),
            edit_budget: cli.edit_budget,
            min_observations: cli.min_observations,
        })
    } else {
        None
    };

    let config = StripperConfig {
        mode: cli.mode,
        replacement_probability: cli.probability,
        min_confidence: cli.min_confidence,
        preserve_named_entities: true,
        preserve_technical_terms: true,
        seed: cli.seed,
        temperature: cli.temperature,
        max_candidates_per_word: cli.max_candidates,
        use_thesaurus_api: cli.api,
        allow_re_replacement: cli.allow_re_replacement,
        domain: cli.domain,
        selection_strategy: cli.strategy,
        enable_lexical,
        enable_syntax,
        enable_cadence,
        enable_function_words,
        typing_noise_config,
        terminology_config,
        sensitivity_config,
        detector_import,
    };

    // 2. Execute Transformation Pipeline
    let result = stripper.transform_multipass(&input_text, &config, cli.passes);

    // 3. Automated SynthID Verification on Output Text
    let synthid_detector = SynthIdDetector::new(SynthIdConfig {
        key: cli.synthid_key,
        context_length: cli.synthid_k,
        detection_threshold: 3.0,
    });
    let synthid_scan = synthid_detector.detect(&result.transformed_text);

    // 4. Output Writing
    if cli.json {
        let json_output = serde_json::to_string_pretty(&result).expect("Serialize to JSON");
        println!("{}", json_output);
    } else if let Some(ref out_path) = cli.output {
        fs::write(out_path, &result.transformed_text)?;
        eprintln!(
            "{}",
            format!("[Output] Transformed text written to {}", out_path).green()
        );
    } else if !cli.diff {
        // Piped stdout: pure transformed text
        print!("{}", result.transformed_text);
        if !result.transformed_text.ends_with('\n') {
            println!();
        }
    }

    // 5. Generate Empirical Log (if -l or --log specified)
    if let Some(ref log_path) = cli.log {
        let active_layers: Vec<String> = result
            .report
            .enabled_classes
            .iter()
            .map(|c| c.to_string())
            .collect();

        let exp_id = cli.experiment_id.clone().unwrap_or_else(|| {
            let seed_str = cli
                .seed
                .map(|s| s.to_string())
                .unwrap_or_else(|| "none".into());
            let layers_str = if active_layers.is_empty() {
                "baseline".to_string()
            } else {
                active_layers.join("_")
            };
            format!("{}_prob{:.2}_seed_{}", layers_str, cli.probability, seed_str)
        });

        let exp_term_enabled = result.report.terminology_metrics.is_some();
        let (exp_term_source, exp_term_version, exp_term_hash) =
            if let Some(ref tm) = result.report.terminology_metrics {
                (
                    tm.terminology_source.clone(),
                    tm.terminology_version.clone(),
                    tm.terminology_hash.clone(),
                )
            } else {
                ("none".into(), "none".into(), "none".into())
            };

        let experiment = lexicon_stripper::ExperimentMetadata {
            id: exp_id,
            parent_input_sha256: result.report.input_sha256.clone(),
            random_seed: result.report.seed_used,
            replicate: cli.replicate,
            terminology_enabled: exp_term_enabled,
            terminology_source: exp_term_source,
            terminology_database_version: exp_term_version,
            terminology_hash: exp_term_hash,
        };

        let empirical_log = EmpiricalLog {
            experiment,
            timestamp_utc: Utc::now().to_rfc3339(),
            input_sha256: result.report.input_sha256.clone(),
            output_sha256: result.report.output_sha256.clone(),
            seed: result.report.seed_used,
            distribution_mode: result.report.mode_used.to_string(),
            detected_domain: result.report.detected_domain.clone(),
            effective_replacement_entropy: result.report.effective_replacement_entropy,
            domain_locked_tokens_count: result.report.domain_locked_tokens_count,
            active_layers,
            total_tokens: result.report.total_tokens,
            total_words: result.report.total_words,
            eligible_words: result.report.eligible_words,
            replaced_words: result.report.replaced_words,
            replacement_rate_pct: result.report.replacement_rate,
            lexical_turnover_pct: result.report.lexical_turnover * 100.0,
            unique_content_tokens_changed: result.report.unique_original_tokens_changed,
            unique_content_tokens_total: result.report.unique_original_tokens,
            mean_semantic_confidence: result.report.mean_confidence,
            jsd_metrics: result.report.jsd_metrics.clone(),
            original_structural_metrics: result.report.original_structural_metrics.clone(),
            transformed_structural_metrics: result.report.transformed_structural_metrics.clone(),
            replacements_by_layer: result.report.replacements_by_class.clone(),
            typing_noise_metrics: result.report.typing_metrics.clone(),
            terminology_metrics: result.report.terminology_metrics.clone(),
            synthid_verification: Some(synthid_scan.clone()),
            transformations: result.report.replacements.clone(),
            audit_records: result.report.audit_records.clone(),
        };

        let log_json =
            serde_json::to_string_pretty(&empirical_log).expect("Serialize empirical log");
        fs::write(log_path, &log_json)?;
        eprintln!(
            "{}",
            format!("[Log] Empirical metrics saved to {}", log_path)
                .cyan()
                .bold()
        );
    }

    if let Some(ref log_txt_path) = cli.log_text {
        let text_report = format_human_report_string(&result.report, Some(&synthid_scan));
        fs::write(log_txt_path, &text_report)?;
        eprintln!(
            "{}",
            format!("[Log] Human report written to {}", log_txt_path).cyan()
        );
    }

    if cli.diff {
        display_diff(&input_text, &result);
    }

    if cli.terminology_debug {
        print_terminology_debug(&result.report);
    }

    if cli.report {
        print_human_report(&result.report, Some(&synthid_scan));
    }

    Ok(())
}

fn display_diff(original: &str, result: &TransformResult) {
    eprintln!("\n{}", "=== TRANSFORMATION DIFF ===".bold().cyan());
    eprintln!(
        "Mode: {}",
        result.report.mode_used.to_string().yellow().bold()
    );
    eprintln!(
        "Enabled Layers: {}",
        result
            .report
            .enabled_classes
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
            .green()
    );
    eprintln!("Original:    {}", original.dimmed());
    eprintln!("Transformed: {}\n", result.transformed_text.green().bold());
}

fn format_human_report_string(
    report: &TransformReport,
    synthid_scan: Option<&lexicon_stripper::SynthIdDetectionResult>,
) -> String {
    let mut s = String::new();
    s.push_str("==================================================\n");
    s.push_str("            EMPIRICAL TRANSFORMATION REPORT       \n");
    s.push_str("==================================================\n");
    s.push_str(&format!(
        "Input SHA-256:                 {}\n",
        report.input_sha256
    ));
    s.push_str(&format!(
        "Output SHA-256:                {}\n",
        report.output_sha256
    ));
    if let Some(seed) = report.seed_used {
        s.push_str(&format!("Deterministic Seed:            {}\n", seed));
    }
    s.push_str(&format!(
        "Distribution Mode:             {}\n",
        report.mode_used
    ));
    s.push_str(&format!(
        "Detected Linguistic Domain:    {}\n",
        report.detected_domain
    ));
    s.push_str(&format!(
        "Effective Replacement Entropy: {:.3} bits/word\n",
        report.effective_replacement_entropy
    ));
    s.push_str(&format!(
        "Domain-Locked Terms Count:     {}\n",
        report.domain_locked_tokens_count
    ));
    s.push_str(&format!(
        "Enabled Layers:                {}\n",
        report
            .enabled_classes
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    ));
    s.push_str(&format!(
        "Total Tokens:                 {}\n",
        report.total_tokens
    ));
    s.push_str(&format!(
        "Total Words:                  {}\n",
        report.total_words
    ));
    s.push_str(&format!(
        "Eligible Words:               {}\n",
        report.eligible_words
    ));
    s.push_str(&format!(
        "Replaced Words:               {}\n",
        report.replaced_words
    ));
    s.push_str(&format!(
        "Replacement Rate:             {:.1}%\n",
        report.replacement_rate
    ));
    s.push_str(&format!(
        "Lexical Turnover:             {:.1}% ({}/{} unique content words)\n",
        report.lexical_turnover * 100.0,
        report.unique_original_tokens_changed,
        report.unique_original_tokens
    ));
    s.push_str(&format!(
        "Mean Confidence:              {:.3}\n",
        report.mean_confidence
    ));

    s.push_str("\n--- Multi-Category Jensen-Shannon Divergence (JSD) ---\n");
    s.push_str(&format!(
        "  Total Token Vocabulary:     {:.4} bits\n",
        report.jsd_metrics.total_jsd
    ));
    s.push_str(&format!(
        "  Content Words (NN/VB/JJ/RB): {:.4} bits\n",
        report.jsd_metrics.content_words_jsd
    ));
    s.push_str(&format!(
        "  Nouns:                      {:.4} bits\n",
        report.jsd_metrics.nouns_jsd
    ));
    s.push_str(&format!(
        "  Verbs:                      {:.4} bits\n",
        report.jsd_metrics.verbs_jsd
    ));
    s.push_str(&format!(
        "  Adjectives:                 {:.4} bits\n",
        report.jsd_metrics.adjectives_jsd
    ));
    s.push_str(&format!(
        "  Adverbs:                    {:.4} bits\n",
        report.jsd_metrics.adverbs_jsd
    ));
    s.push_str(&format!(
        "  Function / Closed-Class:    {:.4} bits\n",
        report.jsd_metrics.function_words_jsd
    ));

    s.push_str("\n--- Structural Metrics Comparison ---\n");
    s.push_str(&format!(
        "  Sentence Count:             {} -> {}\n",
        report.original_structural_metrics.sentence_count,
        report.transformed_structural_metrics.sentence_count
    ));
    s.push_str(&format!(
        "  Mean Sentence Length:       {:.1} -> {:.1} words\n",
        report.original_structural_metrics.mean_sentence_length,
        report.transformed_structural_metrics.mean_sentence_length
    ));
    s.push_str(&format!(
        "  Sentence Length Variance:   {:.2} -> {:.2}\n",
        report.original_structural_metrics.sentence_length_variance,
        report
            .transformed_structural_metrics
            .sentence_length_variance
    ));
    s.push_str(&format!(
        "  Content/Function Ratio:     {:.2} -> {:.2}\n",
        report.original_structural_metrics.content_function_ratio,
        report.transformed_structural_metrics.content_function_ratio
    ));

    if let Some(scan) = synthid_scan {
        s.push_str("\n--- SynthID Watermark Verification ---\n");
        s.push_str(&format!(
            "  Evaluated Windows:          {}\n",
            scan.evaluated_windows
        ));
        s.push_str(&format!(
            "  Observed Mean G-Value:      {:.4} (null baseline = 0.5000)\n",
            scan.mean_g_value
        ));
        s.push_str(&format!(
            "  Standardized Z-Score:       {:.3} (threshold Z >= {:.2})\n",
            scan.z_score, scan.threshold_z
        ));
        s.push_str(&format!(
            "  Statistical p-value:        {:.6} (alpha = {:.2})\n",
            scan.p_value, scan.alpha
        ));
        s.push_str(&format!(
            "  Signal Detected:            {}\n",
            scan.watermark_signal_detected
        ));
        s.push_str(&format!(
            "  Statistical Classification: {}\n",
            scan.classification
        ));
    }

    if let Some(ref tm) = report.terminology_metrics {
        s.push_str("\n--- Authoritative Domain Terminology (IATE / EuroVoc) ---\n");
        s.push_str(&format!(
            "  Terminology Source / Version: {} ({})\n",
            tm.terminology_source, tm.terminology_version
        ));
        s.push_str(&format!(
            "  Database SHA-256 Hash:        {}\n",
            tm.terminology_hash
        ));
        s.push_str("  Detected Domain Scores:\n");
        for (dom, score) in &tm.detected_domains {
            s.push_str(&format!("    - {:16}: {:.2}\n", dom, score));
        }
        s.push_str(&format!(
            "  Domain Terms Detected:        {}\n",
            tm.domain_terms_detected
        ));
        s.push_str(&format!(
            "  Domain Terms Eligible:        {}\n",
            tm.domain_terms_eligible
        ));
        s.push_str(&format!(
            "  Domain Terms Protected:       {}\n",
            tm.domain_terms_protected
        ));
        s.push_str(&format!(
            "  Domain Terms With Variants:   {}\n",
            tm.domain_terms_with_variants
        ));
        s.push_str(&format!(
            "  Domain Terms Actually Replaced: {}\n",
            tm.domain_terms_replaced
        ));
        for span in &tm.matched_spans {
            s.push_str(&format!(
                "    - \"{}\" -> canonical: \"{}\" [{} | {} | {}]\n",
                span.matched_text, span.canonical_term, span.domain, span.relation, span.status
            ));
        }
        s.push_str(&format!(
            "  General Lexical Turnover:     {:.1}%\n",
            tm.general_lexical_turnover_pct
        ));
        s.push_str(&format!(
            "  Domain Lexical Turnover:      {:.1}%\n",
            tm.domain_lexical_turnover_pct
        ));
    }

    if let Some(ref tm) = report.typing_metrics {
        s.push_str("\n--- Human Typing Noise Metrics ---\n");
        s.push_str(&format!(
            "  Configured Rate:            {:.1}%\n",
            tm.rate_configured * 100.0
        ));
        s.push_str(&format!(
            "  Corrupted Tokens:           {}\n",
            tm.corrupted_tokens
        ));
        s.push_str(&format!(
            "  Actual Error Rate:          {:.1}%\n",
            tm.actual_error_rate * 100.0
        ));
        s.push_str(&format!(
            "  Mean Keyboard Distance:     {:.2}\n",
            tm.mean_keyboard_distance
        ));
        s.push_str(&format!(
            "  Character Edit Distance:    {}\n",
            tm.character_edit_distance
        ));
        s.push_str(&format!(
            "  Word Edit Distance:         {}\n",
            tm.word_edit_distance
        ));
    }

    s.push_str("==================================================\n");
    s
}

fn print_terminology_debug(report: &TransformReport) {
    eprintln!(
        "\n{}",
        "==================================================".magenta()
    );
    eprintln!(
        "{}",
        "       DOMAIN TERMINOLOGY DIAGNOSTIC TRACE        "
            .bold()
            .magenta()
    );
    eprintln!(
        "{}",
        "==================================================".magenta()
    );

    if let Some(ref tm) = report.terminology_metrics {
        eprintln!("Database Version:  {}", tm.terminology_version.cyan());
        eprintln!("Database Hash:     {}", tm.terminology_hash.dimmed());
        eprintln!("Source Mode:       {}", tm.terminology_source.yellow());
        eprintln!("\nDomain Density Scores:");
        for (dom, score) in &tm.detected_domains {
            eprintln!("  {:18} : {:.3}", dom.cyan(), score);
        }

        eprintln!("\nGranular Denominators:");
        eprintln!("  Domain Terms Detected         : {}", tm.domain_terms_detected);
        eprintln!("  Domain Terms Eligible         : {}", tm.domain_terms_eligible);
        eprintln!("  Domain Terms Protected        : {}", tm.domain_terms_protected);
        eprintln!("  Domain Terms With Variants    : {}", tm.domain_terms_with_variants);
        eprintln!("  Domain Terms Actually Replaced: {}", tm.domain_terms_replaced);

        eprintln!("\nMatched Terminology Expressions (Longest-Match-First):");
        if tm.matched_spans.is_empty() {
            eprintln!("  (none detected)");
        } else {
            for (i, span) in tm.matched_spans.iter().enumerate() {
                eprintln!(
                    "  {:2}. \"{}\" [Tokens {}-{}] (Domain: {}, Relation: {}, Status: {})",
                    i + 1,
                    span.matched_text.yellow().bold(),
                    span.start_token_idx,
                    span.end_token_idx,
                    span.domain.cyan(),
                    span.relation.to_string().magenta(),
                    span.status.to_string().red()
                );
                if !span.acceptable_variants.is_empty() {
                    eprintln!(
                        "      Acceptable Dialect Equivalents: {}",
                        span.acceptable_variants.join(", ").green()
                    );
                } else {
                    eprintln!("      Acceptable Dialect Equivalents: (none - protected immutable region)");
                }
            }
        }

        eprintln!("\nTerminology Candidate Validation Diagnostics:");
        if tm.diagnostics.is_empty() {
            eprintln!("  (all candidates conformed to domain register)");
        } else {
            for (i, diag) in tm.diagnostics.iter().enumerate() {
                eprintln!(
                    "  {:2}. Term: \"{}\" | Cand: \"{}\" | Domain: {} | Action: {}",
                    i + 1,
                    diag.original_term.yellow(),
                    diag.candidate.dimmed(),
                    diag.domain.cyan(),
                    diag.reason.red().bold()
                );
            }
        }

        eprintln!("\nTurnover Stratification:");
        eprintln!(
            "  General English Turnover : {:.2}%",
            tm.general_lexical_turnover_pct
        );
        eprintln!(
            "  Domain Lexical Turnover  : {:.2}%",
            tm.domain_lexical_turnover_pct
        );
    } else {
        eprintln!("  (Terminology layer was not enabled for this run)");
    }
    eprintln!(
        "{}",
        "==================================================".magenta()
    );
}

fn print_human_report(
    report: &TransformReport,
    synthid_scan: Option<&lexicon_stripper::SynthIdDetectionResult>,
) {
    eprintln!(
        "{}",
        "==================================================".cyan()
    );
    eprintln!(
        "{}",
        "            TRANSFORMATION REPORT                 "
            .bold()
            .cyan()
    );
    eprintln!(
        "{}",
        "==================================================".cyan()
    );
    eprintln!(
        "Input SHA-256:                 {}",
        report.input_sha256.dimmed()
    );
    eprintln!(
        "Output SHA-256:                {}",
        report.output_sha256.dimmed()
    );
    if let Some(s) = report.seed_used {
        eprintln!("Deterministic Seed:            {}", s);
    }
    eprintln!(
        "Distribution Mode:             {}",
        report.mode_used.to_string().yellow().bold()
    );
    eprintln!(
        "Detected Linguistic Domain:    {}",
        report.detected_domain.cyan().bold()
    );
    eprintln!(
        "Effective Replacement Entropy: {:.3} bits/word",
        report.effective_replacement_entropy
    );
    eprintln!(
        "Domain-Locked Terms Count:     {}",
        report.domain_locked_tokens_count
    );
    eprintln!(
        "Enabled Layers:                {}",
        report
            .enabled_classes
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
            .green()
    );
    eprintln!("Total Tokens:                 {}", report.total_tokens);
    eprintln!("Total Words:                  {}", report.total_words);
    eprintln!("Eligible Words:               {}", report.eligible_words);
    eprintln!(
        "Replaced Words:               {}",
        report.replaced_words.to_string().green().bold()
    );
    eprintln!(
        "Replacement Rate:             {:.1}%",
        report.replacement_rate
    );
    eprintln!(
        "Lexical Turnover:             {:.1}% ({}/{} unique content words)",
        report.lexical_turnover * 100.0,
        report.unique_original_tokens_changed,
        report.unique_original_tokens
    );
    eprintln!(
        "Mean Confidence:              {:.3}",
        report.mean_confidence
    );

    if let Some(ref tm) = report.terminology_metrics {
        eprintln!(
            "\n{}",
            "--- Authoritative Domain Terminology (IATE / EuroVoc) ---"
                .bold()
                .cyan()
        );
        eprintln!("  Terminology Source:             {}", tm.terminology_source.green());
        eprintln!("  Terminology Version:            {}", tm.terminology_version.cyan());
        eprintln!("  Database SHA-256:               {}", tm.terminology_hash.dimmed());
        eprintln!("  Domain Terms Detected:          {}", tm.domain_terms_detected);
        eprintln!("  Domain Terms Eligible:          {}", tm.domain_terms_eligible);
        eprintln!("  Domain Terms Protected:         {}", tm.domain_terms_protected);
        eprintln!("  Domain Terms with Variants:     {}", tm.domain_terms_with_variants);
        eprintln!("  Domain Terms Actually Replaced: {}", tm.domain_terms_replaced);
        eprintln!(
            "  General Lexical Turnover:       {:.1}%",
            tm.general_lexical_turnover_pct
        );
        eprintln!(
            "  Domain Lexical Turnover:        {:.1}%",
            tm.domain_lexical_turnover_pct
        );
    }

    eprintln!(
        "\n{}",
        "--- Multi-Category Jensen-Shannon Divergence (JSD) ---"
            .bold()
            .magenta()
    );
    eprintln!(
        "  Total Token Vocabulary:     {:.4} bits",
        report.jsd_metrics.total_jsd
    );
    eprintln!(
        "  Content Words (NN/VB/JJ/RB): {:.4} bits",
        report.jsd_metrics.content_words_jsd
    );
    eprintln!(
        "  Nouns:                      {:.4} bits",
        report.jsd_metrics.nouns_jsd
    );
    eprintln!(
        "  Verbs:                      {:.4} bits",
        report.jsd_metrics.verbs_jsd
    );
    eprintln!(
        "  Adjectives:                 {:.4} bits",
        report.jsd_metrics.adjectives_jsd
    );
    eprintln!(
        "  Adverbs:                    {:.4} bits",
        report.jsd_metrics.adverbs_jsd
    );
    eprintln!(
        "  Function / Closed-Class:    {:.4} bits",
        report.jsd_metrics.function_words_jsd
    );

    eprintln!(
        "\n{}",
        "--- Structural Metrics Comparison ---".bold().blue()
    );
    eprintln!(
        "  Sentence Count:             {} -> {}",
        report.original_structural_metrics.sentence_count,
        report.transformed_structural_metrics.sentence_count
    );
    eprintln!(
        "  Mean Sentence Length:       {:.1} -> {:.1} words",
        report.original_structural_metrics.mean_sentence_length,
        report.transformed_structural_metrics.mean_sentence_length
    );
    eprintln!(
        "  Sentence Length Variance:   {:.2} -> {:.2}",
        report.original_structural_metrics.sentence_length_variance,
        report
            .transformed_structural_metrics
            .sentence_length_variance
    );
    eprintln!(
        "  Content/Function Ratio:     {:.2} -> {:.2}",
        report.original_structural_metrics.content_function_ratio,
        report.transformed_structural_metrics.content_function_ratio
    );

    if !report.replacements_by_class.is_empty() {
        eprintln!("\n{}", "--- Transformations By Layer ---".bold());
        for (class, count) in &report.replacements_by_class {
            eprintln!("  {:14} : {} transformations", class.cyan(), count);
        }
    }

    if let Some(scan) = synthid_scan {
        eprintln!(
            "\n{}",
            "--- SynthID Watermark Verification ---".bold().cyan()
        );
        eprintln!("  Evaluated Windows:          {}", scan.evaluated_windows);
        eprintln!(
            "  Observed Mean G-Value:      {:.4} (null baseline = 0.5000)",
            scan.mean_g_value
        );
        eprintln!(
            "  Standardized Z-Score:       {:.3} (threshold Z >= {:.2})",
            scan.z_score, scan.threshold_z
        );
        eprintln!(
            "  Statistical p-value:        {:.6} (alpha = {:.2})",
            scan.p_value, scan.alpha
        );
        eprintln!(
            "  Watermark Signal Detected:  {}",
            scan.watermark_signal_detected
        );
        if scan.watermark_signal_detected {
            eprintln!(
                "  Statistical Classification: {}",
                scan.classification.red().bold()
            );
        } else if scan.z_score >= 1.645 {
            eprintln!(
                "  Statistical Classification: {}",
                scan.classification.yellow().bold()
            );
        } else {
            eprintln!(
                "  Statistical Classification: {}",
                scan.classification.green().bold()
            );
        }
    }

    if let Some(ref tm) = report.typing_metrics {
        eprintln!("\n{}", "--- Human Typing Noise ---".bold().red());
        eprintln!(
            "  Typing Noise Rate:          {:.1}%",
            tm.rate_configured * 100.0
        );
        eprintln!("  Eligible Tokens:            {}", tm.eligible_tokens);
        eprintln!(
            "  Tokens Corrupted:           {}",
            tm.corrupted_tokens.to_string().red().bold()
        );
        eprintln!(
            "  Actual Error Rate:          {:.1}%",
            tm.actual_error_rate * 100.0
        );
        eprintln!(
            "  Mean Keyboard Distance:     {:.2}",
            tm.mean_keyboard_distance
        );
        eprintln!(
            "  Character Edit Distance:    {}",
            tm.character_edit_distance
        );
        eprintln!("  Word Edit Distance:         {}", tm.word_edit_distance);
        if !tm.error_class_counts.is_empty() {
            eprintln!("  Error Classes:");
            for (err_cls, count) in &tm.error_class_counts {
                eprintln!("    {:14} : {}", err_cls.cyan(), count);
            }
        }
        if !tm.mutations.is_empty() {
            eprintln!("  Applied Typing Mutations:");
            for (i, m) in tm.mutations.iter().enumerate() {
                eprintln!(
                    "    {:2}. {} -> {} [{}] (pos: {}, '{}' -> '{}', dist: {:.2})",
                    i + 1,
                    m.original_word.yellow(),
                    m.mutated_word.red().bold(),
                    m.error_class.as_str().cyan(),
                    m.char_position,
                    m.original_char,
                    m.mutated_char,
                    m.keyboard_distance
                );
            }
        }
    }

    eprintln!("\n{}", "--- Applied Transformations ---".bold());
    if report.replacements.is_empty() {
        eprintln!("  (none)");
    } else {
        for (i, t) in report.replacements.iter().enumerate() {
            eprintln!(
                "  {:2}. [{}] {} {} {} [{}] (confidence: {:.2})",
                i + 1,
                t.transformation_class.to_string().cyan(),
                t.original.red().strikethrough(),
                "->".dimmed(),
                t.replacement.green().bold(),
                t.pos.to_string().yellow(),
                t.confidence
            );
            eprintln!("      Sense/Role: \"{}\"", t.sense_gloss.italic().dimmed());
        }
    }

    eprintln!(
        "{}",
        "==================================================".cyan()
    );
}

fn print_synthid_report(scan: &lexicon_stripper::SynthIdDetectionResult) {
    eprintln!(
        "\n{}",
        "==================================================".cyan()
    );
    eprintln!(
        "{}",
        "        SYNTHID TEXT WATERMARK DETECTION          "
            .bold()
            .cyan()
    );
    eprintln!(
        "{}",
        "==================================================".cyan()
    );
    eprintln!("Evaluated Context Windows:    {}", scan.evaluated_windows);
    eprintln!("Context History Depth (k):    {}", scan.context_length);
    eprintln!(
        "Mean Observed G-Value:        {:.4} (null baseline = 0.5000)",
        scan.mean_g_value
    );
    eprintln!(
        "Standardized Z-Score:         {:.3} (threshold Z >= {:.2})",
        scan.z_score, scan.threshold_z
    );
    eprintln!(
        "Statistical p-value:          {:.6} (alpha = {:.2})",
        scan.p_value, scan.alpha
    );
    eprintln!(
        "Watermark Signal Detected:    {}",
        scan.watermark_signal_detected
    );
    if scan.watermark_signal_detected {
        eprintln!(
            "Statistical Classification:   {}",
            scan.classification.red().bold()
        );
    } else if scan.z_score >= 1.645 {
        eprintln!(
            "Statistical Classification:   {}",
            scan.classification.yellow().bold()
        );
    } else {
        eprintln!(
            "Statistical Classification:   {}",
            scan.classification.green().bold()
        );
    }
    eprintln!(
        "{}",
        "==================================================".cyan()
    );
}

fn print_kirchenbauer_report(scan: &lexicon_stripper::KirchenbauerDetectionResult) {
    eprintln!(
        "\n{}",
        "==================================================".green()
    );
    eprintln!(
        "{}",
        "       KIRCHENBAUER (MARYLAND) WATERMARK SCAN     "
            .bold()
            .green()
    );
    eprintln!(
        "{}",
        "==================================================".green()
    );
    eprintln!("Context Length k:             {}", scan.context_length);
    eprintln!("Total Evaluated Tokens:       {}", scan.total_tokens);
    eprintln!("Green List Tokens:            {}", scan.green_tokens);
    eprintln!(
        "Observed Green Fraction:      {:.4} (null expectation gamma = {:.4})",
        scan.green_fraction, scan.expected_fraction
    );
    eprintln!(
        "Standardized Z-Score:         {:.3} (threshold Z >= {:.2})",
        scan.z_score, scan.threshold_z
    );
    eprintln!(
        "Statistical p-value:          {:.6} (alpha = {:.2})",
        scan.p_value, scan.alpha
    );
    eprintln!(
        "Watermark Signal Detected:    {}",
        scan.watermark_signal_detected
    );
    if scan.watermark_signal_detected {
        eprintln!(
            "Statistical Classification:   {}",
            scan.classification.red().bold()
        );
    } else if scan.z_score >= 1.645 {
        eprintln!(
            "Statistical Classification:   {}",
            scan.classification.yellow().bold()
        );
    } else {
        eprintln!(
            "Statistical Classification:   {}",
            scan.classification.green().bold()
        );
    }
    eprintln!(
        "{}",
        "==================================================".green()
    );
}

fn print_ablation_summary(report: &AblationMatrixReport) {
    eprintln!("\n{}", "=====================================================================================================================================".cyan());
    eprintln!("{}", "                                      CONTROLLED ABLATION EXPERIMENT MATRIX REPORT                                                   ".bold().cyan());
    eprintln!("{}", "=====================================================================================================================================".cyan());
    eprintln!("Input SHA-256: {}", report.input_text_sha256.dimmed());
    eprintln!();
    eprintln!(
        "{:4} | {:30} | {:4} {:4} {:4} {:4} {:4} | {:8} | {:8} | {:8} | {:8} | {:14} | {:18}",
        "Run",
        "Description",
        "Lex",
        "Syn",
        "Cad",
        "Func",
        "Type",
        "Turnover",
        "Tot JSD",
        "Cont JSD",
        "Adj JSD",
        "SynthID Z-Score",
        "GPTZero Score"
    );
    eprintln!("{:-<140}", "");
    for run in &report.runs {
        let lex_sym = if run.lexical_enabled { "✅" } else { "❌" };
        let syn_sym = if run.syntax_enabled { "✅" } else { "❌" };
        let cad_sym = if run.cadence_enabled { "✅" } else { "❌" };
        let func_sym = if run.function_words_enabled {
            "✅"
        } else {
            "❌"
        };
        let type_sym = if run.typing_enabled { "✅" } else { "❌" };

        let jsd = &run.transform_result.report.jsd_metrics;
        let turnover = run.transform_result.report.lexical_turnover * 100.0;

        let synthid_str = if run.synthid_result.watermark_signal_detected {
            format!("Z={:.2} (SIG)", run.synthid_result.z_score)
                .red()
                .bold()
                .to_string()
        } else if run.synthid_result.z_score >= 1.645 {
            format!("Z={:.2} (BORDERLINE)", run.synthid_result.z_score)
                .yellow()
                .to_string()
        } else {
            format!("Z={:.2} (NOT_SIG)", run.synthid_result.z_score)
                .green()
                .bold()
                .to_string()
        };

        let gptzero_str = if let Some(ref gz) = run.gptzero_result {
            let class_str = gz.document_classification.as_deref().unwrap_or("UNK");
            let ai_prob = gz.completely_generated_prob.unwrap_or(0.0) * 100.0;
            format!("{} ({:.1}%)", class_str, ai_prob)
        } else {
            "N/A (no key)".to_string()
        };

        eprintln!(
            "{:4} | {:30} |  {}   {}   {}   {}   {}  | {:7.1}% | {:6.4}b | {:6.4}b | {:6.4}b | {:14} | {:18}",
            run.label.bold().yellow(),
            run.description,
            lex_sym, syn_sym, cad_sym, func_sym, type_sym,
            turnover,
            jsd.total_jsd,
            jsd.content_words_jsd,
            jsd.adjectives_jsd,
            synthid_str,
            gptzero_str.bold()
        );
    }
    eprintln!("{}", "=====================================================================================================================================".cyan());
}

fn print_general_help_with_topics() {
    let mut cmd = Cli::command();
    let _ = cmd.print_help();
    println!();
}

fn print_typo_help() {
    println!("{}", "================================================================================".cyan().bold());
    println!("{}", "            HUMAN TYPING NOISE & BIOMECHANICAL ERROR SIMULATOR                 ".cyan().bold());
    println!("{}", "================================================================================".cyan().bold());
    println!();
    println!("{}", "1. OVERVIEW & MOTIVATION".yellow().bold());
    println!("  The typing noise engine simulates realistic human typing imperfections based on");
    println!("  physical keyboard ergonomics and neuromuscular timing variations. Unlike random");
    println!("  synthetic corruption, all mutations conform to physical 104-key ANSI QWERTY");
    println!("  geometry, finger reach trajectories, and human motor habits.");
    println!("  This breaks periodic n-gram statistical biases of AI watermarks while preserving");
    println!("  human readability and natural colloquial cadence.");
    println!();
    println!("{}", "2. PHYSICAL KEYBOARD MODELING".yellow().bold());
    println!("  • Keyboard Layout: Standard ANSI QWERTY 104-key staggered matrix.");
    println!("  • Coordinate Space: Discrete Euclidean coordinates (x, y) assigned to every key.");
    println!("  • Proximity Weighting: Neighbor substitution probability decays exponentially with");
    println!("    physical distance: P(key' | key) ∝ exp(-α · distance).");
    println!("  • Case & Shift Awareness: Preserves capitalization and case consistency.");
    println!();
    println!("{}", "3. THE 6 BIOMECHANICAL ERROR CLASSES".yellow().bold());
    println!("  {} (Weight: 35%)", "• substitution".green().bold());
    println!("    Finger strikes a physical neighbor key adjacent to the intended character on");
    println!("    the keyboard matrix (e.g. 'd' -> 's', 'e', 'r', 'f', 'x', 'c').");
    println!("    Example: 'location' -> 'locstion' (strike 's' instead of 'a', distance: 1.0)");
    println!();
    println!("  {} (Weight: 20%)", "• transposition".green().bold());
    println!("    Inversion of two adjacent characters due to finger timing slips or rapid alternate-");
    println!("    hand keystrokes. Weighted higher for keys struck in rapid succession.");
    println!("    Example: 'the' -> 'teh', 'receive' -> 'recieve', 'from' -> 'form'");
    println!();
    println!("  {} (Weight: 15%)", "• omission".green().bold());
    println!("    A keystroke is skipped or insufficiently pressed. Guarded against short words");
    println!("    (length <= 3) to protect reader comprehension and token legibility.");
    println!("    Example: 'industry' -> 'idustry', 'particular' -> 'particlar'");
    println!();
    println!("  {} (Weight: 15%)", "• insertion".green().bold());
    println!("    An unintended adjacent key is struck alongside the intended character during");
    println!("    key press or key release (fat-finger / double contact).");
    println!("    Example: 'recipe' -> 'recxipe' (stray 'x' beside 'c')");
    println!();
    println!("  {} (Weight: 10%)", "• duplication".green().bold());
    println!("    Key bounce or mechanical double-strike repeating a valid character.");
    println!("    Example: 'coffee' -> 'coffeey', 'speed' -> 'speeed'");
    println!();
    println!("  {} (Weight: 5%)", "• temporal".green().bold());
    println!("    Sequential finger-reach trajectory error. Models biomechanical reach where the");
    println!("    hand moving from key t-1 to key t+1 accidentally brushes an intermediate key.");
    println!("    Example: Trajectory reach errors on interior characters of long words.");
    println!();
    println!("{}", "4. SAFETY GUARDS & IMMUNITY FILTERS".yellow().bold());
    println!("  The typing engine guarantees preservation of critical semantic anchors:");
    println!("  ✓ Named Entities & Proper Nouns (NNP, NNPS) are 100% immune.");
    println!("  ✓ Numeric quantities, prices, currency signs, and percentages are immune.");
    println!("  ✓ URLs, email addresses, file paths, and code tokens are immune.");
    println!("  ✓ Command line flags and options (--flag, -f) are immune.");
    println!("  ✓ Short words (length < 2) and function-critical words are protected.");
    println!("  ✓ Previously transformed tokens are protected against compounded corruption.");
    println!();
    println!("{}", "5. COMMAND LINE OPTIONS".yellow().bold());
    println!("  {} <FLOAT>", "--typing-noise".magenta().bold());
    println!("      Rate/probability of corrupting an eligible content word (0.0 to 1.0).");
    println!("      Default: 0.0 (disabled). Recommended human baseline: 0.02 to 0.05 (2% - 5%).");
    println!();
    println!("  {} <INT>", "--typing-seed".magenta().bold());
    println!("      64-bit deterministic RNG master seed for 100% reproducible error placement.");
    println!();
    println!("  {} <CLASSES>", "--typing-errors".magenta().bold());
    println!("      Comma-separated list of active error classes. Available classes:");
    println!("      substitution (sub), transposition (trans), omission (omit),");
    println!("      insertion (ins), duplication (dup), temporal (temp).");
    println!("      Default: All 6 error classes enabled with biomechanical weights.");
    println!();
    println!("{}", "6. REPORTING & METRICS".yellow().bold());
    println!("  When combined with '--report', the engine logs comprehensive typo diagnostics:");
    println!("  • Total eligible vs corrupted token count and exact percentage.");
    println!("  • Breakdown of mutations by error class.");
    println!("  • Mean physical keyboard Euclidean distance (in key-pitch units).");
    println!("  • Levenshtein character edit distance & Word Error Rate (WER).");
    println!("  • Detailed mutation audit log showing before/after, positions, and distances.");
    println!();
    println!("{}", "7. CLI USAGE EXAMPLES".yellow().bold());
    println!("  # 1. Subtle 3% human typing noise with deterministic reproducibility:");
    println!("  {}", "lexicon_stripper -f input.txt -o output.txt --typing-noise 0.03 --typing-seed 1337 --report".cyan());
    println!();
    println!("  # 2. Target only realistic transpositions and neighbor substitutions:");
    println!("  {}", "lexicon_stripper \"The quick brown fox\" --typing-noise 0.15 --typing-errors substitution,transposition".cyan());
    println!();
    println!("  # 3. Combined perturbation (Lexical Resampling + Syntax + Typing Noise):");
    println!("  {}", "lexicon_stripper -f essay.txt -o cleaned.txt --combined --typing-noise 0.02 --report".cyan());
    println!();
    println!("{}", "================================================================================".cyan().bold());
}

fn print_domain_help() {
    println!("{}", "================================================================================".cyan().bold());
    println!("{}", "               SUPPORTED SPECIALIZED LINGUISTIC DOMAINS                         ".cyan().bold());
    println!("{}", "================================================================================".cyan().bold());
    println!();
    println!("The engine features 29 specialized linguistic domains with locked jargon protection");
    println!("and curated bidirectional equivalences (use with '--domain <NAME>' or default 'auto'):");
    println!();
    let domains = [
        ("culinary", "Fine dining, cooking techniques, wine, gastronomy (sommelier, stagiaire, Michelin, reduction)"),
        ("finance", "Markets, banking, corporate finance (ebitda, liquidity, yield, amortization, insolvency)"),
        ("biomedical", "Medicine, pathology, pharmacology (apoptosis, cytokine, pathogen, oncogene, mrna)"),
        ("computer_science", "Algorithms, systems, software engineering (compiler, mutex, semaphore, deadlock, heap)"),
        ("legal", "Jurisprudence, litigation, contracts (subpoena, tort, plaintiff, affidavit, injunction)"),
        ("creative_arts", "Visual arts, sculpture, art history (chiaroscuro, triptych, fresco, contrapposto)"),
        ("science", "Physics, chemistry, general research (spectroscopy, entropy, neutrino, isotope, catalyst)"),
        ("politics", "Governance, elections, policy (filibuster, gerrymandering, incumbent, referendum)"),
        ("sports", "Athletics, competitive tournaments (quarterback, goalkeeper, decathlon, steeplechase)"),
        ("education", "Academia, pedagogy, university administration (pedagogy, syllabus, dissertation, matriculation)"),
        ("philosophy_psychology", "Cognition, epistemology, psychoanalysis (phenomenology, hermeneutics, behaviorism)"),
        ("military", "Defense, warfare tactics, armed forces (brigade, reconnaissance, artillery, battalion, flank)"),
        ("journalism_media", "News reporting, publishing, broadcasting (byline, op-ed, broadsheet, syndication, newsroom)"),
        ("travel_tourism", "Hospitality, itineraries, vacationing (itinerary, excursion, concierge, sightseeing)"),
        ("aviation_aerospace", "Aeronautics, avionics, space flight (avionics, telemetry, fuselage, altimeter, mach)"),
        ("architecture_construction", "Structural engineering, architecture (cantilever, load-bearing, hvac, brutalism, masonry)"),
        ("gaming_esports", "Game mechanics, esports, speedrunning (hitbox, aggro, frame data, rng, dps, nerfed)"),
        ("agriculture_botany", "Agronomy, plant biology, farming (hydroponics, grafting, cultivar, photosynthesis, npk)"),
        ("fashion_textiles", "Haute couture, garment fabrication (selvage, warp, weft, bias cut, silhouette, bespoke)"),
        ("theology_religion", "Religious studies, scripture, liturgy (exegesis, ecclesiastical, dogma, liturgy, canon)"),
        ("audio_engineering", "Sound design, music production, acoustics (reverb, equalization, lossless, polyrhythm, attenuation)"),
        ("maritime_nautical", "Naval operations, sailing, ocean shipping (starboard, ballast, keel, draft, bulkhead, knot)"),
        ("automotive_motorsport", "Automotive engineering, racing, powertrains (camshaft, chassis, torque, oversteer, slipstream)"),
        ("film_television", "Cinematography, production, post-production (mise-en-scène, foley, anamorphic, best boy, storyboard)"),
        ("linguistics_philology", "Phonetics, syntax, language evolution (phoneme, morphology, fricative, diphthong, syntax)"),
        ("real_estate_property", "Housing markets, deeds, land ownership (escrow, lien, appraisal, zoning, sublet, title)"),
        ("logistics_supply_chain", "Freight operations, warehousing, supply networks (intermodal, procurement, bill of lading, sku)"),
        ("fitness_kinesiology", "Sports science, gym culture, hypertrophy (hypertrophy, anabolic, macros, deadlift, isometric)"),
        ("occult_astrology", "Esoterica, tarot, zodiac, astrological charts (retrograde, sigil, astral, manifestation, divination)"),
    ];
    for (i, (name, desc)) in domains.iter().enumerate() {
        println!("  {:2}. {:<25} {}", (i + 1).to_string().cyan(), name.green().bold(), desc);
    }
    println!();
    println!("{}", "================================================================================".cyan().bold());
}

fn print_watermark_help() {
    println!("{}", "================================================================================".cyan().bold());
    println!("{}", "              WATERMARK DETECTION & PERTURBATION ARCHITECTURE                   ".cyan().bold());
    println!("{}", "================================================================================".cyan().bold());
    println!();
    println!("{}", "1. SUPPORTED WATERMARK SCHEMES".yellow().bold());
    println!("  • SynthID (Google DeepMind): Pseudo-random G-statistic tournament sampling.");
    println!("    Configured with '--synthid-key <KEY>' (default: 428917492) and '--synthid-k <K>' (default: 2).");
    println!("    Scan input text with '--synthid-detect' or watermark text with '--synthid-watermark'.");
    println!();
    println!("  • Kirchenbauer et al. (Maryland): Red-Green token partitioning scheme.");
    println!("    Configured with '--kirchenbauer-key <KEY>', '--kirchenbauer-k <K>', and '--kirchenbauer-gamma <GAMMA>'.");
    println!("    Scan input text with '--kirchenbauer-detect' or watermark text with '--kirchenbauer-watermark'.");
    println!();
    println!("{}", "2. WATERMARK STRIPPING MECHANICS".yellow().bold());
    println!("  The engine strips watermark signals without AI models by breaking periodic n-gram");
    println!("  statistical biases through human frequency resampling, domain-locked preservation,");
    println!("  and optional physical typing noise injection.");
    println!();
    println!("{}", "================================================================================".cyan().bold());
}

fn run_interactive(stripper: &LexiconStripper) -> io::Result<()> {
    println!(
        "{}",
        "=================================================="
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "       LEXICON STRIPPER - INTERACTIVE MODE        "
            .yellow()
            .bold()
    );
    println!(
        "{}",
        "=================================================="
            .cyan()
            .bold()
    );
    println!("Commands:");
    println!("  :help                           - Display command help");
    println!("  :mode <neutral|human|ai|random>  - Change distribution mode");
    println!("  :prob <0.0..1.0>                - Set replacement probability");
    println!("  :conf <0.0..1.0>                - Set min confidence threshold");
    println!("  :typo <0.0..1.0>                - Set typing noise rate (e.g. :typo 0.03)");
    println!("  :typos                          - View human typing noise guide");
    println!("  :syntax                         - Toggle syntax restructuring");
    println!("  :cadence                        - Toggle cadence modulation");
    println!("  :func                           - Toggle function word modulation");
    println!("  :quit / :exit                   - Exit REPL");
    println!(
        "{}",
        "--------------------------------------------------".dimmed()
    );

    let mut config = StripperConfig::default();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!(
            "{} [{}] > ",
            "lexicon".green().bold(),
            config.mode.to_string().yellow()
        );
        stdout.flush()?;

        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            break;
        }
        let input = line.trim();

        if input.is_empty() {
            continue;
        }

        if input == ":quit" || input == ":exit" {
            break;
        }

        if input == ":help" {
            println!("Commands:");
            println!("  :help                           - Display command help");
            println!("  :mode <neutral|human|ai|random>  - Change distribution mode");
            println!("  :prob <0.0..1.0>                - Set replacement probability");
            println!("  :conf <0.0..1.0>                - Set min confidence threshold");
            println!("  :typo <0.0..1.0>                - Set typing noise rate (e.g. :typo 0.03)");
            println!("  :typos                          - View human typing noise guide");
            println!("  :syntax                         - Toggle syntax restructuring");
            println!("  :cadence                        - Toggle cadence modulation");
            println!("  :func                           - Toggle function word modulation");
            println!("  :quit / :exit                   - Exit REPL");
            continue;
        }

        if input == ":typos" || input == ":typo-help" {
            print_typo_help();
            continue;
        }

        if let Some(rest) = input.strip_prefix(":typo ") {
            if let Ok(rate) = rest.parse::<f64>() {
                if rate <= 0.0 {
                    config.typing_noise_config = None;
                    println!("Typing noise disabled");
                } else {
                    config.typing_noise_config = Some(TypingNoiseConfig {
                        rate: rate.clamp(0.0, 1.0),
                        ..Default::default()
                    });
                    println!("Typing noise rate set to: {:.1}%", rate * 100.0);
                }
            }
            continue;
        }

        if let Some(rest) = input.strip_prefix(":prob ") {
            if let Ok(p) = rest.parse::<f64>() {
                config.replacement_probability = p.clamp(0.0, 1.0);
                println!(
                    "Replacement probability set to: {}",
                    config.replacement_probability
                );
            }
            continue;
        }

        if let Some(rest) = input.strip_prefix(":conf ") {
            if let Ok(c) = rest.parse::<f64>() {
                config.min_confidence = c.clamp(0.0, 1.0);
                println!("Min confidence set to: {}", config.min_confidence);
            }
            continue;
        }

        if let Some(rest) = input.strip_prefix(":mode ") {
            match rest.to_lowercase().as_str() {
                "neutral" => config.mode = DistributionMode::Neutral,
                "human" => config.mode = DistributionMode::Human,
                "ai" => config.mode = DistributionMode::Ai,
                "random" => config.mode = DistributionMode::Random,
                _ => println!("Invalid mode. Choose neutral, human, ai, or random."),
            }
            println!("Mode set to: {}", config.mode.to_string().yellow().bold());
            continue;
        }

        if input == ":syntax" {
            config.enable_syntax = !config.enable_syntax;
            println!("Syntax restructuring: {}", config.enable_syntax);
            continue;
        }

        if input == ":cadence" {
            config.enable_cadence = !config.enable_cadence;
            println!("Cadence modulation: {}", config.enable_cadence);
            continue;
        }

        if input == ":func" {
            config.enable_function_words = !config.enable_function_words;
            println!(
                "Function words modulation: {}",
                config.enable_function_words
            );
            continue;
        }

        // Run transformation
        let result = stripper.transform(input, &config);
        display_diff(input, &result);
        print_human_report(&result.report, None);
    }

    Ok(())
}
