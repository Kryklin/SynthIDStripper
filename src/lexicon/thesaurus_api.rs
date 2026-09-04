use crate::types::{CoarsePos, PosTag};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Deserialize)]
struct DatamuseItem {
    word: String,
    #[serde(default)]
    tags: Vec<String>,
}

pub struct ThesaurusApiClient {
    cache: Mutex<HashMap<(String, CoarsePos), Vec<String>>>,
    enabled: bool,
}

impl ThesaurusApiClient {
    pub fn new(enabled: bool) -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
            enabled,
        }
    }

    /// Fetches context-compatible synonyms from the Datamuse Thesaurus API.
    /// (Datamuse is a computational linguistics lexical database, NOT an AI generation model).
    pub fn fetch_synonyms(&self, word: &str, pos: PosTag) -> Vec<String> {
        if !self.enabled {
            return Vec::new();
        }

        let coarse = pos.coarse_pos();
        let lower = word.to_lowercase();

        // Check cache
        if let Ok(guard) = self.cache.lock() {
            if let Some(cached) = guard.get(&(lower.clone(), coarse)) {
                return cached.clone();
            }
        }

        let pos_tag_filter = match coarse {
            CoarsePos::Noun => "n",
            CoarsePos::Verb => "v",
            CoarsePos::Adjective => "adj",
            CoarsePos::Adverb => "adv",
            _ => "",
        };

        // Datamuse query: rel_syn (direct synonyms) with metadata (defs, parts of speech)
        let url = format!(
            "https://api.datamuse.com/words?rel_syn={}&md=pd&max=15",
            urlencoding(&lower)
        );

        let response = ureq::get(&url)
            .timeout(std::time::Duration::from_millis(1500))
            .call();

        let mut results = Vec::new();

        if let Ok(resp) = response {
            if let Ok(items) = resp.into_json::<Vec<DatamuseItem>>() {
                for item in items {
                    // Filter by matching POS tag if available
                    let matches_pos = if pos_tag_filter.is_empty() {
                        true
                    } else {
                        item.tags.iter().any(|t| t == pos_tag_filter)
                    };

                    // Exclude multi-word phrases (e.g. "at the same time") to keep single-word lexical substitutions
                    let is_single_word = !item.word.contains(' ') && !item.word.contains('-');
                    let is_different = !item.word.eq_ignore_ascii_case(&lower);

                    if matches_pos && is_single_word && is_different && item.word.len() > 1 {
                        results.push(item.word);
                    }
                }
            }
        }

        // Cache results
        if let Ok(mut guard) = self.cache.lock() {
            guard.insert((lower, coarse), results.clone());
        }

        results
    }
}

fn urlencoding(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{:02X}", b));
        }
    }
    out
}
