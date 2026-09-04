use super::types::{DomainTerm, MatchedTermSpan, TermRelation, TermStatus};
use crate::tokenization::Token;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub const DEFAULT_TERMINOLOGY_VERSION: &str = "IATE-EuroVoc-2026.1";

/// In-memory indexed database of authoritative domain terminology
#[derive(Debug, Clone)]
pub struct TerminologyDb {
    pub version: String,
    pub terms: Vec<DomainTerm>,
    /// Index: normalized surface phrase -> indices in `terms`
    by_term: HashMap<String, Vec<usize>>,
    /// Index: normalized lemma phrase -> indices in `terms`
    by_lemma: HashMap<String, Vec<usize>>,
    /// Index: domain name -> indices in `terms`
    by_domain: HashMap<String, Vec<usize>>,
    /// Max word count in any multiword term in the DB (for window sizing)
    pub max_phrase_tokens: usize,
    /// Cached cryptographic SHA-256 hash of the database state
    db_hash: String,
}

impl Default for TerminologyDb {
    fn default() -> Self {
        Self::new_embedded()
    }
}

impl TerminologyDb {
    /// Creates a terminology database populated with curated IATE / EuroVoc terminology
    pub fn new_embedded() -> Self {
        let mut db = Self {
            version: DEFAULT_TERMINOLOGY_VERSION.to_string(),
            terms: Vec::new(),
            by_term: HashMap::new(),
            by_lemma: HashMap::new(),
            by_domain: HashMap::new(),
            max_phrase_tokens: 1,
            db_hash: String::new(),
        };

        db.populate_curated_dataset();
        db.rebuild_indexes();
        db
    }

    /// Loads terminology definitions from an external JSON file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| format!("Failed to read terminology file: {}", e))?;
        let terms: Vec<DomainTerm> = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse terminology JSON: {}", e))?;

        let mut db = Self {
            version: format!("custom_{}", path.as_ref().display()),
            terms,
            by_term: HashMap::new(),
            by_lemma: HashMap::new(),
            by_domain: HashMap::new(),
            max_phrase_tokens: 1,
            db_hash: String::new(),
        };

        db.rebuild_indexes();
        Ok(db)
    }

    /// Rebuilds all in-memory lookup indexes and computes the version hash
    pub fn rebuild_indexes(&mut self) {
        self.by_term.clear();
        self.by_lemma.clear();
        self.by_domain.clear();
        self.max_phrase_tokens = 1;

        for (idx, item) in self.terms.iter().enumerate() {
            let term_norm = item.term.to_lowercase().trim().to_string();
            let lemma_norm = item.lemma.to_lowercase().trim().to_string();
            let domain_norm = item.domain_name.to_lowercase().trim().to_string();

            let token_count = term_norm.split_whitespace().count();
            if token_count > self.max_phrase_tokens {
                self.max_phrase_tokens = token_count;
            }

            self.by_term.entry(term_norm).or_default().push(idx);
            self.by_lemma.entry(lemma_norm).or_default().push(idx);
            self.by_domain.entry(domain_norm).or_default().push(idx);
        }

        self.db_hash = self.calculate_hash();
    }

    /// Returns the SHA-256 hash of the terminology database
    pub fn get_hash(&self) -> &str {
        &self.db_hash
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.version.as_bytes());
        for term in &self.terms {
            hasher.update(term.term.as_bytes());
            hasher.update(term.domain_name.as_bytes());
            hasher.update(term.relation.as_str().as_bytes());
            hasher.update(term.status.to_string().as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    /// Performs Longest-Match-First multiword phrase scanning across token sequence
    pub fn scan_spans(&self, tokens: &[Token], domain_filter: Option<&str>) -> Vec<MatchedTermSpan> {
        let mut matched_spans = Vec::new();
        let word_indices: Vec<usize> = tokens
            .iter()
            .enumerate()
            .filter(|(_, t)| t.is_word())
            .map(|(i, _)| i)
            .collect();

        let n_words = word_indices.len();
        let mut word_cursor = 0;

        while word_cursor < n_words {
            let mut longest_match: Option<MatchedTermSpan> = None;

            // Try candidate lengths from max_phrase_tokens down to 1
            let max_window = self.max_phrase_tokens.min(n_words - word_cursor);
            for len in (1..=max_window).rev() {
                let start_token_idx = word_indices[word_cursor];
                let end_token_idx = word_indices[word_cursor + len - 1] + 1;

                // Reconstruct surface phrase and lemma phrase
                let mut surface_words = Vec::new();
                let mut lemma_words = Vec::new();

                for &idx in &word_indices[word_cursor..word_cursor + len] {
                    surface_words.push(tokens[idx].text.to_lowercase());
                    lemma_words.push(
                        tokens[idx]
                            .lemma
                            .clone()
                            .unwrap_or_else(|| tokens[idx].text.to_lowercase()),
                    );
                }

                let phrase_surface = surface_words.join(" ");
                let phrase_lemma = lemma_words.join(" ");

                // Look up in index
                let matches = self.lookup_phrase(&phrase_surface, &phrase_lemma, domain_filter);
                if let Some(best_term) = matches.first() {
                    let mut acceptable_variants = Vec::new();
                    if let Some(ref pref) = best_term.preferred_term {
                        if !pref.eq_ignore_ascii_case(&phrase_surface) {
                            acceptable_variants.push(pref.clone());
                        }
                    }
                    if let Some(ref abbrev) = best_term.abbreviation {
                        if !abbrev.eq_ignore_ascii_case(&phrase_surface) {
                            acceptable_variants.push(abbrev.clone());
                        }
                    }

                    // Look up other synonyms with exact/acceptable relations
                    let variants_from_domain = self.get_acceptable_variants(&phrase_surface, &best_term.domain_name);
                    for v in variants_from_domain {
                        if !v.eq_ignore_ascii_case(&phrase_surface) && !acceptable_variants.contains(&v) {
                            acceptable_variants.push(v);
                        }
                    }

                    let is_locked = best_term.status == TermStatus::DomainLocked
                        || best_term.relation == TermRelation::ProtectedTerm;

                    longest_match = Some(MatchedTermSpan {
                        start_token_idx,
                        end_token_idx,
                        matched_text: tokens[start_token_idx..end_token_idx]
                            .iter()
                            .map(|t| t.text.as_str())
                            .collect::<Vec<_>>()
                            .join(""),
                        canonical_term: best_term.term.clone(),
                        domain: best_term.domain_name.clone(),
                        relation: best_term.relation,
                        status: best_term.status,
                        acceptable_variants,
                        is_locked,
                    });
                    break; // Longest-match found
                }
            }

            if let Some(span) = longest_match {
                let consumed_words = tokens[span.start_token_idx..span.end_token_idx]
                    .iter()
                    .filter(|t| t.is_word())
                    .count();
                matched_spans.push(span);
                word_cursor += consumed_words.max(1);
            } else {
                word_cursor += 1;
            }
        }

        matched_spans
    }

    /// Looks up terms by surface string or lemma
    pub fn lookup_phrase(
        &self,
        surface: &str,
        lemma: &str,
        domain_filter: Option<&str>,
    ) -> Vec<&DomainTerm> {
        let mut results = Vec::new();
        let mut seen = HashSet::new();

        if let Some(indices) = self.by_term.get(surface) {
            for &idx in indices {
                let t = &self.terms[idx];
                if let Some(dom) = domain_filter {
                    if !t.domain_name.eq_ignore_ascii_case(dom) {
                        continue;
                    }
                }
                if seen.insert(idx) {
                    results.push(t);
                }
            }
        }

        if let Some(indices) = self.by_lemma.get(lemma) {
            for &idx in indices {
                let t = &self.terms[idx];
                if let Some(dom) = domain_filter {
                    if !t.domain_name.eq_ignore_ascii_case(dom) {
                        continue;
                    }
                }
                if seen.insert(idx) {
                    results.push(t);
                }
            }
        }

        results
    }

    /// Gets authorized domain-equivalent variants for a given term within a domain
    pub fn get_acceptable_variants(&self, term: &str, domain: &str) -> Vec<String> {
        let norm_term = term.to_lowercase();
        let mut variants = Vec::new();

        if let Some(indices) = self.by_domain.get(&domain.to_lowercase()) {
            for &idx in indices {
                let entry = &self.terms[idx];
                if entry.term.eq_ignore_ascii_case(&norm_term) {
                    if let Some(ref pref) = entry.preferred_term {
                        if !pref.eq_ignore_ascii_case(&norm_term) {
                            variants.push(pref.clone());
                        }
                    }
                    if let Some(ref abbrev) = entry.abbreviation {
                        if !abbrev.eq_ignore_ascii_case(&norm_term) {
                            variants.push(abbrev.clone());
                        }
                    }
                } else if entry.preferred_term.as_deref() == Some(&norm_term)
                    && entry.relation.is_substitution_permitted()
                {
                    variants.push(entry.term.clone());
                }
            }
        }

        variants
    }

    /// Returns whether a candidate replacement is an authorized domain equivalent
    pub fn is_authorized_replacement(&self, original_term: &str, candidate: &str, domain: &str) -> bool {
        let variants = self.get_acceptable_variants(original_term, domain);
        variants.iter().any(|v| v.eq_ignore_ascii_case(candidate))
    }

    fn populate_curated_dataset(&mut self) {
        let add = |terms: &mut Vec<DomainTerm>,
                   term: &str,
                   lemma: &str,
                   domain: &str,
                   relation: TermRelation,
                   pref: Option<&str>,
                   abbrev: Option<&str>,
                   status: TermStatus| {
            terms.push(DomainTerm {
                term: term.to_string(),
                lemma: lemma.to_string(),
                language: "en".to_string(),
                domain_id: format!("iate:{}", domain),
                domain_name: domain.to_string(),
                relation,
                preferred_term: pref.map(|s| s.to_string()),
                abbreviation: abbrev.map(|s| s.to_string()),
                status,
                source: "IATE/EuroVoc".to_string(),
                source_version: DEFAULT_TERMINOLOGY_VERSION.to_string(),
                pos: None,
            });
        };

        // --- 1. FINANCE & ECONOMICS ---
        add(&mut self.terms, "market capitalization", "market capitalization", "finance", TermRelation::PreferredTerm, None, Some("market cap"), TermStatus::DomainLocked);
        add(&mut self.terms, "market cap", "market cap", "finance", TermRelation::AcceptableVariant, Some("market capitalization"), Some("mcap"), TermStatus::DomainLocked);
        add(&mut self.terms, "mcap", "mcap", "finance", TermRelation::StandardAbbreviation, Some("market capitalization"), None, TermStatus::DomainLocked);
        add(&mut self.terms, "price-to-earnings ratio", "price-to-earnings ratio", "finance", TermRelation::PreferredTerm, None, Some("P/E ratio"), TermStatus::DomainLocked);
        add(&mut self.terms, "p/e ratio", "p/e ratio", "finance", TermRelation::AcceptableVariant, Some("price-to-earnings ratio"), Some("PE"), TermStatus::DomainLocked);
        add(&mut self.terms, "gross domestic product", "gross domestic product", "economics", TermRelation::PreferredTerm, None, Some("GDP"), TermStatus::DomainLocked);
        add(&mut self.terms, "gdp", "gdp", "economics", TermRelation::StandardAbbreviation, Some("gross domestic product"), None, TermStatus::DomainLocked);
        add(&mut self.terms, "discounted cash flow", "discounted cash flow", "finance", TermRelation::PreferredTerm, None, Some("DCF"), TermStatus::DomainLocked);
        add(&mut self.terms, "net present value", "net present value", "finance", TermRelation::PreferredTerm, None, Some("NPV"), TermStatus::DomainLocked);
        add(&mut self.terms, "initial public offering", "initial public offering", "finance", TermRelation::PreferredTerm, None, Some("IPO"), TermStatus::DomainLocked);
        add(&mut self.terms, "value at risk", "value at risk", "finance", TermRelation::PreferredTerm, None, Some("VaR"), TermStatus::DomainLocked);
        add(&mut self.terms, "debt-to-equity ratio", "debt-to-equity ratio", "finance", TermRelation::PreferredTerm, None, Some("D/E ratio"), TermStatus::DomainLocked);
        add(&mut self.terms, "balance sheet", "balance sheet", "finance", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "cash flow", "cash flow", "finance", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "interest rate", "interest rate", "finance", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "fiscal policy", "fiscal policy", "economics", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "monetary policy", "monetary policy", "economics", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        
        // Single-word finance terms
        add(&mut self.terms, "ebitda", "ebitda", "finance", TermRelation::ProtectedTerm, None, None, TermStatus::Protected);
        add(&mut self.terms, "liquidity", "liquidity", "finance", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "amortization", "amortization", "finance", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "yield", "yield", "finance", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "insolvency", "insolvency", "finance", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "leverage", "leverage", "finance", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "securities", "security", "finance", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "dividend", "dividend", "finance", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "collateral", "collateral", "finance", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "arbitrage", "arbitrage", "finance", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "underwriting", "underwrite", "finance", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "solvency", "solvency", "finance", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "fiduciary", "fiduciary", "finance", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "inflation", "inflation", "economics", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "deflation", "deflation", "economics", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "recession", "recession", "economics", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "equity", "equity", "finance", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "liability", "liability", "finance", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);

        // Substitutable finance relations
        add(&mut self.terms, "firm", "firm", "finance", TermRelation::PreferredTerm, Some("enterprise"), None, TermStatus::Active);
        add(&mut self.terms, "enterprise", "enterprise", "finance", TermRelation::AcceptableVariant, Some("firm"), None, TermStatus::Active);
        add(&mut self.terms, "company", "company", "finance", TermRelation::AcceptableVariant, Some("firm"), None, TermStatus::Active);
        add(&mut self.terms, "corporation", "corporation", "finance", TermRelation::AcceptableVariant, Some("firm"), None, TermStatus::Active);
        add(&mut self.terms, "investor", "investor", "finance", TermRelation::PreferredTerm, Some("shareholder"), None, TermStatus::Active);
        add(&mut self.terms, "shareholder", "shareholder", "finance", TermRelation::AcceptableVariant, Some("investor"), None, TermStatus::Active);
        add(&mut self.terms, "profit", "profit", "finance", TermRelation::PreferredTerm, Some("earnings"), None, TermStatus::Active);
        add(&mut self.terms, "earnings", "earning", "finance", TermRelation::AcceptableVariant, Some("profit"), None, TermStatus::Active);
        add(&mut self.terms, "revenue", "revenue", "finance", TermRelation::PreferredTerm, Some("sales"), None, TermStatus::Active);
        add(&mut self.terms, "sales", "sale", "finance", TermRelation::AcceptableVariant, Some("revenue"), None, TermStatus::Active);
        add(&mut self.terms, "capital", "capital", "finance", TermRelation::PreferredTerm, Some("funds"), None, TermStatus::Active);
        add(&mut self.terms, "funds", "fund", "finance", TermRelation::AcceptableVariant, Some("capital"), None, TermStatus::Active);
        add(&mut self.terms, "valuation", "valuation", "finance", TermRelation::PreferredTerm, Some("market value"), None, TermStatus::Active);
        add(&mut self.terms, "market value", "market value", "finance", TermRelation::AcceptableVariant, Some("valuation"), None, TermStatus::Active);

        // --- 2. BIOMEDICAL & LIFE SCIENCES ---
        add(&mut self.terms, "messenger ribonucleic acid", "messenger ribonucleic acid", "biomedical", TermRelation::PreferredTerm, None, Some("mRNA"), TermStatus::DomainLocked);
        add(&mut self.terms, "mrna", "mrna", "biomedical", TermRelation::StandardAbbreviation, Some("messenger ribonucleic acid"), None, TermStatus::DomainLocked);
        add(&mut self.terms, "deoxyribonucleic acid", "deoxyribonucleic acid", "biomedical", TermRelation::PreferredTerm, None, Some("DNA"), TermStatus::DomainLocked);
        add(&mut self.terms, "dna", "dna", "biomedical", TermRelation::StandardAbbreviation, Some("deoxyribonucleic acid"), None, TermStatus::DomainLocked);
        add(&mut self.terms, "natural killer cell", "natural killer cell", "biomedical", TermRelation::PreferredTerm, None, Some("NK cell"), TermStatus::DomainLocked);
        add(&mut self.terms, "polymerase chain reaction", "polymerase chain reaction", "biomedical", TermRelation::PreferredTerm, None, Some("PCR"), TermStatus::DomainLocked);
        add(&mut self.terms, "central nervous system", "central nervous system", "biomedical", TermRelation::PreferredTerm, None, Some("CNS"), TermStatus::DomainLocked);
        add(&mut self.terms, "cardiovascular disease", "cardiovascular disease", "biomedical", TermRelation::PreferredTerm, None, Some("CVD"), TermStatus::DomainLocked);
        add(&mut self.terms, "immune system", "immune system", "biomedical", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "blood pressure", "blood pressure", "biomedical", TermRelation::PreferredTerm, None, Some("BP"), TermStatus::DomainLocked);
        add(&mut self.terms, "heart rate", "heart rate", "biomedical", TermRelation::PreferredTerm, None, Some("HR"), TermStatus::DomainLocked);
        add(&mut self.terms, "clinical trial", "clinical trial", "biomedical", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "adverse effect", "adverse effect", "biomedical", TermRelation::PreferredTerm, Some("side effect"), None, TermStatus::DomainLocked);
        add(&mut self.terms, "side effect", "side effect", "biomedical", TermRelation::AcceptableVariant, Some("adverse effect"), None, TermStatus::Active);
        add(&mut self.terms, "white blood cell", "white blood cell", "biomedical", TermRelation::PreferredTerm, Some("leukocyte"), Some("WBC"), TermStatus::DomainLocked);
        add(&mut self.terms, "red blood cell", "red blood cell", "biomedical", TermRelation::PreferredTerm, Some("erythrocyte"), Some("RBC"), TermStatus::DomainLocked);
        add(&mut self.terms, "amino acid", "amino acid", "biomedical", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "monoclonal antibody", "monoclonal antibody", "biomedical", TermRelation::PreferredTerm, None, Some("mAb"), TermStatus::DomainLocked);

        // Single-word biomedical terms
        add(&mut self.terms, "apoptosis", "apoptosis", "biomedical", TermRelation::ProtectedTerm, None, None, TermStatus::Protected);
        add(&mut self.terms, "cytokine", "cytokine", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "oncogene", "oncogene", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "metabolite", "metabolite", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "phagocytosis", "phagocytosis", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "mitochondria", "mitochondria", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "carcinogen", "carcinogen", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "homeostasis", "homeostasis", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "ischemia", "ischemia", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "necrosis", "necrosis", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "pathogen", "pathogen", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "pathology", "pathology", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "pathophysiology", "pathophysiology", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "pharmacokinetics", "pharmacokinetics", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "hypertension", "hypertension", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "receptor", "receptor", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "enzyme", "enzyme", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "antibody", "antibody", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "syndrome", "syndrome", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "diagnosis", "diagnosis", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "prognosis", "prognosis", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "symptom", "symptom", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "mutation", "mutation", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "infection", "infection", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "bacteria", "bacterium", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "virus", "virus", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "protein", "protein", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "tissue", "tissue", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "cellular", "cellular", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "organism", "organism", "biomedical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);

        // Substitutable biomedical relations
        add(&mut self.terms, "malignant", "malignant", "biomedical", TermRelation::PreferredTerm, Some("cancerous"), None, TermStatus::Active);
        add(&mut self.terms, "cancerous", "cancerous", "biomedical", TermRelation::AcceptableVariant, Some("malignant"), None, TermStatus::Active);
        add(&mut self.terms, "dosage", "dosage", "biomedical", TermRelation::PreferredTerm, Some("dose"), None, TermStatus::Active);
        add(&mut self.terms, "dose", "dose", "biomedical", TermRelation::AcceptableVariant, Some("dosage"), None, TermStatus::Active);
        add(&mut self.terms, "physician", "physician", "biomedical", TermRelation::PreferredTerm, Some("doctor"), None, TermStatus::Active);
        add(&mut self.terms, "doctor", "doctor", "biomedical", TermRelation::AcceptableVariant, Some("physician"), None, TermStatus::Active);
        add(&mut self.terms, "efficacy", "efficacy", "biomedical", TermRelation::PreferredTerm, Some("effectiveness"), None, TermStatus::Active);
        add(&mut self.terms, "effectiveness", "effectiveness", "biomedical", TermRelation::AcceptableVariant, Some("efficacy"), None, TermStatus::Active);
        add(&mut self.terms, "therapy", "therapy", "biomedical", TermRelation::PreferredTerm, Some("treatment"), None, TermStatus::Active);
        add(&mut self.terms, "treatment", "treatment", "biomedical", TermRelation::AcceptableVariant, Some("therapy"), None, TermStatus::Active);
        add(&mut self.terms, "disease", "disease", "biomedical", TermRelation::PreferredTerm, Some("disorder"), None, TermStatus::Active);
        add(&mut self.terms, "disorder", "disorder", "biomedical", TermRelation::AcceptableVariant, Some("disease"), None, TermStatus::Active);
        add(&mut self.terms, "illness", "illness", "biomedical", TermRelation::AcceptableVariant, Some("disease"), None, TermStatus::Active);
        add(&mut self.terms, "patient", "patient", "biomedical", TermRelation::PreferredTerm, Some("individual"), None, TermStatus::Active);

        // --- 3. COMPUTER SCIENCE & AI ---
        add(&mut self.terms, "natural language processing", "natural language processing", "computer_science", TermRelation::PreferredTerm, None, Some("NLP"), TermStatus::DomainLocked);
        add(&mut self.terms, "nlp", "nlp", "computer_science", TermRelation::StandardAbbreviation, Some("natural language processing"), None, TermStatus::DomainLocked);
        add(&mut self.terms, "machine learning", "machine learning", "computer_science", TermRelation::PreferredTerm, None, Some("ML"), TermStatus::DomainLocked);
        add(&mut self.terms, "ml", "ml", "computer_science", TermRelation::StandardAbbreviation, Some("machine learning"), None, TermStatus::DomainLocked);
        add(&mut self.terms, "artificial intelligence", "artificial intelligence", "computer_science", TermRelation::PreferredTerm, None, Some("AI"), TermStatus::DomainLocked);
        add(&mut self.terms, "large language model", "large language model", "computer_science", TermRelation::PreferredTerm, None, Some("LLM"), TermStatus::DomainLocked);
        add(&mut self.terms, "neural network", "neural network", "computer_science", TermRelation::PreferredTerm, None, Some("NN"), TermStatus::DomainLocked);
        add(&mut self.terms, "deep learning", "deep learning", "computer_science", TermRelation::PreferredTerm, None, Some("DL"), TermStatus::DomainLocked);
        add(&mut self.terms, "convolutional neural network", "convolutional neural network", "computer_science", TermRelation::PreferredTerm, None, Some("CNN"), TermStatus::DomainLocked);
        add(&mut self.terms, "recurrent neural network", "recurrent neural network", "computer_science", TermRelation::PreferredTerm, None, Some("RNN"), TermStatus::DomainLocked);
        add(&mut self.terms, "graph neural network", "graph neural network", "computer_science", TermRelation::PreferredTerm, None, Some("GNN"), TermStatus::DomainLocked);
        add(&mut self.terms, "abstract syntax tree", "abstract syntax tree", "computer_science", TermRelation::PreferredTerm, None, Some("AST"), TermStatus::DomainLocked);
        add(&mut self.terms, "just-in-time compilation", "just-in-time compilation", "computer_science", TermRelation::PreferredTerm, None, Some("JIT"), TermStatus::DomainLocked);
        add(&mut self.terms, "operating system", "operating system", "computer_science", TermRelation::PreferredTerm, None, Some("OS"), TermStatus::DomainLocked);
        add(&mut self.terms, "application programming interface", "application programming interface", "computer_science", TermRelation::PreferredTerm, None, Some("API"), TermStatus::DomainLocked);
        add(&mut self.terms, "reinforcement learning", "reinforcement learning", "computer_science", TermRelation::PreferredTerm, None, Some("RL"), TermStatus::DomainLocked);
        add(&mut self.terms, "supervised learning", "supervised learning", "computer_science", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "unsupervised learning", "unsupervised learning", "computer_science", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "gradient descent", "gradient descent", "computer_science", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "data structure", "data structure", "computer_science", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        
        // Single-word computer science terms
        add(&mut self.terms, "compiler", "compiler", "computer_science", TermRelation::ProtectedTerm, None, None, TermStatus::Protected);
        add(&mut self.terms, "deadlock", "deadlock", "computer_science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "mutex", "mutex", "computer_science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "semaphore", "semaphore", "computer_science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "recursion", "recursion", "computer_science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "polymorphism", "polymorphism", "computer_science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "bytecode", "bytecode", "computer_science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "hashmap", "hashmap", "computer_science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "tokenizer", "tokenizer", "computer_science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "dataset", "dataset", "computer_science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "algorithm", "algorithm", "computer_science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "backpropagation", "backpropagation", "computer_science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "hyperparameter", "hyperparameter", "computer_science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "embedding", "embedding", "computer_science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "transformer", "transformer", "computer_science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "overfitting", "overfitting", "computer_science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);

        // Substitutable CS relations
        add(&mut self.terms, "latency", "latency", "computer_science", TermRelation::PreferredTerm, Some("delay"), None, TermStatus::Active);
        add(&mut self.terms, "delay", "delay", "computer_science", TermRelation::AcceptableVariant, Some("latency"), None, TermStatus::Active);
        add(&mut self.terms, "throughput", "throughput", "computer_science", TermRelation::PreferredTerm, Some("bandwidth"), None, TermStatus::Active);
        add(&mut self.terms, "bandwidth", "bandwidth", "computer_science", TermRelation::AcceptableVariant, Some("throughput"), None, TermStatus::Active);
        add(&mut self.terms, "execute", "execute", "computer_science", TermRelation::PreferredTerm, Some("run"), None, TermStatus::Active);
        add(&mut self.terms, "run", "run", "computer_science", TermRelation::AcceptableVariant, Some("execute"), None, TermStatus::Active);
        add(&mut self.terms, "compute", "compute", "computer_science", TermRelation::PreferredTerm, Some("calculate"), None, TermStatus::Active);
        add(&mut self.terms, "calculate", "calculate", "computer_science", TermRelation::AcceptableVariant, Some("compute"), None, TermStatus::Active);
        add(&mut self.terms, "method", "method", "computer_science", TermRelation::PreferredTerm, Some("technique"), None, TermStatus::Active);
        add(&mut self.terms, "technique", "technique", "computer_science", TermRelation::AcceptableVariant, Some("method"), None, TermStatus::Active);
        add(&mut self.terms, "model", "model", "computer_science", TermRelation::PreferredTerm, Some("architecture"), None, TermStatus::Active);
        add(&mut self.terms, "architecture", "architecture", "computer_science", TermRelation::AcceptableVariant, Some("model"), None, TermStatus::Active);

        // --- 4. LEGAL & JURISPRUDENCE ---
        add(&mut self.terms, "habeas corpus", "habeas corpus", "legal", TermRelation::ProtectedTerm, None, None, TermStatus::Protected);
        add(&mut self.terms, "prima facie", "prima facie", "legal", TermRelation::ProtectedTerm, None, None, TermStatus::Protected);
        add(&mut self.terms, "mens rea", "mens rea", "legal", TermRelation::ProtectedTerm, None, None, TermStatus::Protected);
        add(&mut self.terms, "due process", "due process", "legal", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "burden of proof", "burden of proof", "legal", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "reasonable doubt", "reasonable doubt", "legal", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "intellectual property", "intellectual property", "legal", TermRelation::PreferredTerm, None, Some("IP"), TermStatus::DomainLocked);
        add(&mut self.terms, "subpoena", "subpoena", "legal", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "tort", "tort", "legal", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "affidavit", "affidavit", "legal", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "jurisdiction", "jurisdiction", "legal", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "indictment", "indictment", "legal", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "injunction", "injunction", "legal", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "litigation", "litigation", "legal", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "plaintiff", "plaintiff", "legal", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "defendant", "defendant", "legal", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);

        // Substitutable legal relations
        add(&mut self.terms, "attorney", "attorney", "legal", TermRelation::PreferredTerm, Some("lawyer"), None, TermStatus::Active);
        add(&mut self.terms, "lawyer", "lawyer", "legal", TermRelation::AcceptableVariant, Some("attorney"), None, TermStatus::Active);
        add(&mut self.terms, "counsel", "counsel", "legal", TermRelation::AcceptableVariant, Some("attorney"), None, TermStatus::Active);
        add(&mut self.terms, "statute", "statute", "legal", TermRelation::PreferredTerm, Some("regulation"), None, TermStatus::Active);
        add(&mut self.terms, "regulation", "regulation", "legal", TermRelation::AcceptableVariant, Some("statute"), None, TermStatus::Active);
        add(&mut self.terms, "verdict", "verdict", "legal", TermRelation::PreferredTerm, Some("ruling"), None, TermStatus::Active);
        add(&mut self.terms, "ruling", "ruling", "legal", TermRelation::AcceptableVariant, Some("verdict"), None, TermStatus::Active);
        add(&mut self.terms, "legislation", "legislation", "legal", TermRelation::PreferredTerm, Some("law"), None, TermStatus::Active);
        add(&mut self.terms, "law", "law", "legal", TermRelation::AcceptableVariant, Some("legislation"), None, TermStatus::Active);

        // --- 5. CULINARY & GASTRONOMY ---
        add(&mut self.terms, "fine dining", "fine dining", "culinary", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "tasting menu", "tasting menu", "culinary", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "head chef", "head chef", "culinary", TermRelation::PreferredTerm, Some("executive chef"), None, TermStatus::Active);
        add(&mut self.terms, "executive chef", "executive chef", "culinary", TermRelation::AcceptableVariant, Some("head chef"), None, TermStatus::Active);
        add(&mut self.terms, "line cook", "line cook", "culinary", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "brigade system", "brigade system", "culinary", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "James Beard award", "James Beard award", "culinary", TermRelation::ProtectedTerm, None, None, TermStatus::Protected);
        add(&mut self.terms, "Michelin inspector", "Michelin inspector", "culinary", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "Michelin star", "Michelin star", "culinary", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "biodynamic wine", "biodynamic wine", "culinary", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "tap water", "tap water", "culinary", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "sommelier", "sommelier", "culinary", TermRelation::PreferredTerm, Some("wine director"), None, TermStatus::Active);
        add(&mut self.terms, "wine director", "wine director", "culinary", TermRelation::AcceptableVariant, Some("sommelier"), None, TermStatus::Active);
        add(&mut self.terms, "stagiaire", "stagiaire", "culinary", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "stagiaires", "stagiaire", "culinary", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "gastronomy", "gastronomy", "culinary", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "sous-chef", "sous-chef", "culinary", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "restaurateur", "restaurateur", "culinary", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "micro-cilantro", "micro-cilantro", "culinary", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "cilantro", "cilantro", "culinary", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "turnip", "turnip", "culinary", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "steak", "steak", "culinary", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "cuisine", "cuisine", "culinary", TermRelation::PreferredTerm, Some("cooking style"), None, TermStatus::Active);
        add(&mut self.terms, "produce", "produce", "culinary", TermRelation::PreferredTerm, Some("ingredients"), None, TermStatus::Active);
        add(&mut self.terms, "service", "service", "culinary", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);

        // --- 6. CREATIVE ARTS ---
        add(&mut self.terms, "fine art", "fine art", "creative_arts", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "performing arts", "performing arts", "creative_arts", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "visual arts", "visual arts", "creative_arts", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "conceptual art", "conceptual art", "creative_arts", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "chiaroscuro", "chiaroscuro", "creative_arts", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "triptych", "triptych", "creative_arts", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "fresco", "fresco", "creative_arts", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "sculpture", "sculpture", "creative_arts", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "choreography", "choreography", "creative_arts", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "curator", "curator", "creative_arts", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "sonata", "sonata", "creative_arts", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "symphony", "symphony", "creative_arts", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "exhibition", "exhibition", "creative_arts", TermRelation::PreferredTerm, Some("gallery show"), None, TermStatus::Active);

        // --- 7. SCIENCE ---
        add(&mut self.terms, "quantum mechanics", "quantum mechanics", "science", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "general relativity", "general relativity", "science", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "thermodynamics", "thermodynamics", "science", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "periodic table", "periodic table", "science", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "chemical reaction", "chemical reaction", "science", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "spectroscopy", "spectroscopy", "science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "photosynthesis", "photosynthesis", "science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "entropy", "entropy", "science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "neutrino", "neutrino", "science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "catalyst", "catalyst", "science", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "hypothesis", "hypothesis", "science", TermRelation::PreferredTerm, Some("conjecture"), None, TermStatus::Active);
        add(&mut self.terms, "experiment", "experiment", "science", TermRelation::PreferredTerm, Some("trial"), None, TermStatus::Active);

        // --- 8. POLITICS ---
        add(&mut self.terms, "foreign policy", "foreign policy", "politics", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "electoral college", "electoral college", "politics", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "executive order", "executive order", "politics", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "filibuster", "filibuster", "politics", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "gerrymandering", "gerrymandering", "politics", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "referendum", "referendum", "politics", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "parliament", "parliament", "politics", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "bipartisan", "bipartisan", "politics", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "politician", "politician", "politics", TermRelation::PreferredTerm, Some("legislator"), None, TermStatus::Active);
        add(&mut self.terms, "election", "election", "politics", TermRelation::PreferredTerm, Some("ballot"), None, TermStatus::Active);

        // --- 9. SPORTS ---
        add(&mut self.terms, "world cup", "world cup", "sports", TermRelation::ProtectedTerm, None, None, TermStatus::Protected);
        add(&mut self.terms, "championship title", "championship title", "sports", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "penalty kick", "penalty kick", "sports", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "free throw", "free throw", "sports", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "quarterback", "quarterback", "sports", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "goalkeeper", "goalkeeper", "sports", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "decathlon", "decathlon", "sports", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "tournament", "tournament", "sports", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "athlete", "athlete", "sports", TermRelation::PreferredTerm, Some("competitor"), None, TermStatus::Active);

        // --- 10. EDUCATION ---
        add(&mut self.terms, "higher education", "higher education", "education", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "standardized testing", "standardized testing", "education", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "curriculum development", "curriculum development", "education", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "pedagogy", "pedagogy", "education", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "syllabus", "syllabus", "education", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "dissertation", "dissertation", "education", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "matriculation", "matriculation", "education", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "student", "student", "education", TermRelation::PreferredTerm, Some("pupil"), None, TermStatus::Active);

        // --- 11. PHILOSOPHY & PSYCHOLOGY ---
        add(&mut self.terms, "cognitive dissonance", "cognitive dissonance", "philosophy_psychology", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "categorical imperative", "categorical imperative", "philosophy_psychology", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "epistemology", "epistemology", "philosophy_psychology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "phenomenology", "phenomenology", "philosophy_psychology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "psychoanalysis", "psychoanalysis", "philosophy_psychology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "subconscious", "subconscious", "philosophy_psychology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "ontology", "ontology", "philosophy_psychology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "cognition", "cognition", "philosophy_psychology", TermRelation::PreferredTerm, Some("mental process"), None, TermStatus::Active);

        // --- 12. MILITARY ---
        add(&mut self.terms, "rules of engagement", "rules of engagement", "military", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "chain of command", "chain of command", "military", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "special forces", "special forces", "military", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "reconnaissance", "reconnaissance", "military", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "artillery", "artillery", "military", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "battalion", "battalion", "military", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "infantry", "infantry", "military", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "brigade", "brigade", "military", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "commander", "commander", "military", TermRelation::PreferredTerm, Some("officer"), None, TermStatus::Active);

        // --- 13. JOURNALISM & MEDIA ---
        add(&mut self.terms, "press release", "press release", "journalism_media", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "investigative journalism", "investigative journalism", "journalism_media", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "breaking news", "breaking news", "journalism_media", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "byline", "byline", "journalism_media", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "op-ed", "op-ed", "journalism_media", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "broadsheet", "broadsheet", "journalism_media", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "reporter", "reporter", "journalism_media", TermRelation::PreferredTerm, Some("journalist"), None, TermStatus::Active);

        // --- 14. TRAVEL & TOURISM ---
        add(&mut self.terms, "travel itinerary", "travel itinerary", "travel_tourism", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "tourist destination", "tourist destination", "travel_tourism", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "boarding pass", "boarding pass", "travel_tourism", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "itinerary", "itinerary", "travel_tourism", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "excursion", "excursion", "travel_tourism", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "concierge", "concierge", "travel_tourism", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "traveler", "traveler", "travel_tourism", TermRelation::PreferredTerm, Some("tourist"), None, TermStatus::Active);

        // --- 15. AVIATION & AEROSPACE ---
        add(&mut self.terms, "flight dynamics", "flight dynamics", "aviation_aerospace", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "air traffic control", "air traffic control", "aviation_aerospace", TermRelation::PreferredTerm, None, Some("ATC"), TermStatus::DomainLocked);
        add(&mut self.terms, "flight plan", "flight plan", "aviation_aerospace", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "avionics", "avionics", "aviation_aerospace", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "telemetry", "telemetry", "aviation_aerospace", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "fuselage", "fuselage", "aviation_aerospace", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "altimeter", "altimeter", "aviation_aerospace", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "spacecraft", "spacecraft", "aviation_aerospace", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "mach", "mach", "aviation_aerospace", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "aircraft", "aircraft", "aviation_aerospace", TermRelation::PreferredTerm, Some("airplane"), None, TermStatus::Active);

        // --- 16. ARCHITECTURE & CONSTRUCTION ---
        add(&mut self.terms, "building design", "building design", "architecture_construction", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "civil engineering", "civil engineering", "architecture_construction", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "load-bearing wall", "load-bearing wall", "architecture_construction", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "cantilever", "cantilever", "architecture_construction", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "brutalism", "brutalism", "architecture_construction", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "masonry", "masonry", "architecture_construction", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "facade", "facade", "architecture_construction", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "hvac", "hvac", "architecture_construction", TermRelation::StandardAbbreviation, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "easement", "easement", "architecture_construction", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "building", "building", "architecture_construction", TermRelation::PreferredTerm, Some("structure"), None, TermStatus::Active);

        // --- 17. GAMING & ESPORTS ---
        add(&mut self.terms, "game mechanics", "game mechanics", "gaming_esports", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "competitive play", "competitive play", "gaming_esports", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "frame data", "frame data", "gaming_esports", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "hitbox", "hitbox", "gaming_esports", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "aggro", "aggro", "gaming_esports", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "esports", "esports", "gaming_esports", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "matchmaking", "matchmaking", "gaming_esports", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "respawn", "respawn", "gaming_esports", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "speedrun", "speedrun", "gaming_esports", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "dps", "dps", "gaming_esports", TermRelation::StandardAbbreviation, None, None, TermStatus::DomainLocked);

        // --- 18. AGRICULTURE & BOTANY ---
        add(&mut self.terms, "crop science", "crop science", "agriculture_botany", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "plant biology", "plant biology", "agriculture_botany", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "hydroponics", "hydroponics", "agriculture_botany", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "grafting", "grafting", "agriculture_botany", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "agronomy", "agronomy", "agriculture_botany", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "cultivar", "cultivar", "agriculture_botany", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "photosynthesis", "photosynthesis", "agriculture_botany", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "npk", "npk", "agriculture_botany", TermRelation::StandardAbbreviation, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "farmer", "farmer", "agriculture_botany", TermRelation::PreferredTerm, Some("grower"), None, TermStatus::Active);
        add(&mut self.terms, "crop", "crop", "agriculture_botany", TermRelation::PreferredTerm, Some("harvest"), None, TermStatus::Active);

        // --- 19. FASHION & TEXTILES ---
        add(&mut self.terms, "haute couture", "haute couture", "fashion_textiles", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "bias cut", "bias cut", "fashion_textiles", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "apparel design", "apparel design", "fashion_textiles", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "selvage", "selvage", "fashion_textiles", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "warp", "warp", "fashion_textiles", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "weft", "weft", "fashion_textiles", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "silhouette", "silhouette", "fashion_textiles", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "bespoke", "bespoke", "fashion_textiles", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "garment", "garment", "fashion_textiles", TermRelation::PreferredTerm, Some("clothing"), None, TermStatus::Active);
        add(&mut self.terms, "fabric", "fabric", "fashion_textiles", TermRelation::PreferredTerm, Some("textile"), None, TermStatus::Active);

        // --- 20. THEOLOGY & RELIGION ---
        add(&mut self.terms, "religious studies", "religious studies", "theology_religion", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "exegesis", "exegesis", "theology_religion", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "ecclesiastical", "ecclesiastical", "theology_religion", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "dogma", "dogma", "theology_religion", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "liturgy", "liturgy", "theology_religion", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "secular", "secular", "theology_religion", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "orthodox", "orthodox", "theology_religion", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "scripture", "scripture", "theology_religion", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "clergy", "clergy", "theology_religion", TermRelation::PreferredTerm, Some("ministry"), None, TermStatus::Active);
        add(&mut self.terms, "sermon", "sermon", "theology_religion", TermRelation::PreferredTerm, Some("homily"), None, TermStatus::Active);

        // --- 21. AUDIO ENGINEERING ---
        add(&mut self.terms, "sound design", "sound design", "audio_engineering", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "music production", "music production", "audio_engineering", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "reverb", "reverb", "audio_engineering", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "equalization", "equalization", "audio_engineering", TermRelation::DomainSpecific, None, Some("EQ"), TermStatus::DomainLocked);
        add(&mut self.terms, "lossless", "lossless", "audio_engineering", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "polyrhythm", "polyrhythm", "audio_engineering", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "attenuation", "attenuation", "audio_engineering", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "spectrogram", "spectrogram", "audio_engineering", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "sound", "sound", "audio_engineering", TermRelation::PreferredTerm, Some("audio"), None, TermStatus::Active);
        add(&mut self.terms, "recording", "recording", "audio_engineering", TermRelation::PreferredTerm, Some("audio track"), None, TermStatus::Active);

        // --- 22. MARITIME & NAUTICAL ---
        add(&mut self.terms, "naval operations", "naval operations", "maritime_nautical", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "dead reckoning", "dead reckoning", "maritime_nautical", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "starboard", "starboard", "maritime_nautical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "ballast", "ballast", "maritime_nautical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "keel", "keel", "maritime_nautical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "bulkhead", "bulkhead", "maritime_nautical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "halyard", "halyard", "maritime_nautical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "nautical mile", "nautical mile", "maritime_nautical", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "vessel", "vessel", "maritime_nautical", TermRelation::PreferredTerm, Some("ship"), None, TermStatus::Active);
        add(&mut self.terms, "ship", "ship", "maritime_nautical", TermRelation::PreferredTerm, Some("vessel"), None, TermStatus::Active);

        // --- 23. AUTOMOTIVE & MOTORSPORT ---
        add(&mut self.terms, "car mechanics", "car mechanics", "automotive_motorsport", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "internal combustion", "internal combustion", "automotive_motorsport", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "camshaft", "camshaft", "automotive_motorsport", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "chassis", "chassis", "automotive_motorsport", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "drivetrain", "drivetrain", "automotive_motorsport", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "oversteer", "oversteer", "automotive_motorsport", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "slipstream", "slipstream", "automotive_motorsport", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "pit stop", "pit stop", "automotive_motorsport", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "car", "car", "automotive_motorsport", TermRelation::PreferredTerm, Some("automobile"), None, TermStatus::Active);
        add(&mut self.terms, "vehicle", "vehicle", "automotive_motorsport", TermRelation::PreferredTerm, Some("automobile"), None, TermStatus::Active);

        // --- 24. FILM & TELEVISION ---
        add(&mut self.terms, "cinematography", "cinematography", "film_television", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "color grading", "color grading", "film_television", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "mise-en-scène", "mise-en-scène", "film_television", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "mise-en-scene", "mise-en-scene", "film_television", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "foley", "foley", "film_television", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "anamorphic", "anamorphic", "film_television", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "best boy", "best boy", "film_television", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "storyboard", "storyboard", "film_television", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "movie", "movie", "film_television", TermRelation::PreferredTerm, Some("motion picture"), None, TermStatus::Active);
        add(&mut self.terms, "motion picture", "motion picture", "film_television", TermRelation::PreferredTerm, Some("film"), None, TermStatus::Active);

        // --- 25. LINGUISTICS & PHILOLOGY ---
        add(&mut self.terms, "natural language", "natural language", "linguistics_philology", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "language structure", "language structure", "linguistics_philology", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "phoneme", "phoneme", "linguistics_philology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "morphology", "morphology", "linguistics_philology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "fricative", "fricative", "linguistics_philology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "diphthong", "diphthong", "linguistics_philology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "syntax", "syntax", "linguistics_philology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "conjugation", "conjugation", "linguistics_philology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "dialect", "dialect", "linguistics_philology", TermRelation::PreferredTerm, Some("vernacular"), None, TermStatus::Active);
        add(&mut self.terms, "grammar", "grammar", "linguistics_philology", TermRelation::PreferredTerm, Some("syntax"), None, TermStatus::Active);

        // --- 26. REAL ESTATE & PROPERTY ---
        add(&mut self.terms, "real estate", "real estate", "real_estate_property", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "closing costs", "closing costs", "real_estate_property", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "escrow", "escrow", "real_estate_property", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "lien", "lien", "real_estate_property", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "appraisal", "appraisal", "real_estate_property", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "gentrification", "gentrification", "real_estate_property", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "sublet", "sublet", "real_estate_property", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "condominium", "condominium", "real_estate_property", TermRelation::DomainSpecific, None, Some("condo"), TermStatus::DomainLocked);
        add(&mut self.terms, "landlord", "landlord", "real_estate_property", TermRelation::PreferredTerm, Some("lessor"), None, TermStatus::Active);
        add(&mut self.terms, "tenant", "tenant", "real_estate_property", TermRelation::PreferredTerm, Some("lessee"), None, TermStatus::Active);

        // --- 27. LOGISTICS & SUPPLY CHAIN ---
        add(&mut self.terms, "supply chain", "supply chain", "logistics_supply_chain", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "bill of lading", "bill of lading", "logistics_supply_chain", TermRelation::PreferredTerm, None, Some("BOL"), TermStatus::DomainLocked);
        add(&mut self.terms, "distribution center", "distribution center", "logistics_supply_chain", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "intermodal", "intermodal", "logistics_supply_chain", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "procurement", "procurement", "logistics_supply_chain", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "freight", "freight", "logistics_supply_chain", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "sku", "sku", "logistics_supply_chain", TermRelation::StandardAbbreviation, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "warehousing", "warehousing", "logistics_supply_chain", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "shipment", "shipment", "logistics_supply_chain", TermRelation::PreferredTerm, Some("consignment"), None, TermStatus::Active);
        add(&mut self.terms, "inventory", "inventory", "logistics_supply_chain", TermRelation::PreferredTerm, Some("stock"), None, TermStatus::Active);

        // --- 28. FITNESS & KINESIOLOGY ---
        add(&mut self.terms, "physical fitness", "physical fitness", "fitness_kinesiology", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "sports science", "sports science", "fitness_kinesiology", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "hypertrophy", "hypertrophy", "fitness_kinesiology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "anabolic", "anabolic", "fitness_kinesiology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "deadlift", "deadlift", "fitness_kinesiology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "isometric", "isometric", "fitness_kinesiology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "glycogen", "glycogen", "fitness_kinesiology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "superset", "superset", "fitness_kinesiology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "exercise", "exercise", "fitness_kinesiology", TermRelation::PreferredTerm, Some("workout"), None, TermStatus::Active);
        add(&mut self.terms, "workout", "workout", "fitness_kinesiology", TermRelation::PreferredTerm, Some("training"), None, TermStatus::Active);

        // --- 29. OCCULT & ASTROLOGY ---
        add(&mut self.terms, "natal chart", "natal chart", "occult_astrology", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "modern witchcraft", "modern witchcraft", "occult_astrology", TermRelation::PreferredTerm, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "retrograde", "retrograde", "occult_astrology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "sigil", "sigil", "occult_astrology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "divination", "divination", "occult_astrology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "manifestation", "manifestation", "occult_astrology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "esoteric", "esoteric", "occult_astrology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "tarot", "tarot", "occult_astrology", TermRelation::DomainSpecific, None, None, TermStatus::DomainLocked);
        add(&mut self.terms, "horoscope", "horoscope", "occult_astrology", TermRelation::PreferredTerm, Some("astrological chart"), None, TermStatus::Active);
        add(&mut self.terms, "ritual", "ritual", "occult_astrology", TermRelation::PreferredTerm, Some("ceremony"), None, TermStatus::Active);
    }
}
