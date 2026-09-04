use super::db::TerminologyDb;
use std::fs;
use std::path::Path;
use std::time::Duration;

/// Client for terminology service synchronization and cache management
#[derive(Debug, Clone)]
pub struct TerminologyApiClient {
    pub endpoint_url: String,
    pub timeout: Duration,
}

impl Default for TerminologyApiClient {
    fn default() -> Self {
        Self {
            endpoint_url: "https://iate.europa.eu/api/terms".to_string(),
            timeout: Duration::from_secs(5),
        }
    }
}

impl TerminologyApiClient {
    pub fn new() -> Self {
        Self::default()
    }

    /// Attempts to synchronize or refresh the local terminology database cache from an authoritative source.
    /// Resilient: Preserves existing database on network/parsing failure.
    pub fn refresh_cache(&self, cache_path: Option<&Path>, current_db: &TerminologyDb) -> Result<TerminologyDb, String> {
        eprintln!(
            "[Terminology API] Checking for IATE/EuroVoc terminology updates at {}...",
            self.endpoint_url
        );

        // Attempt HTTP fetch via ureq if available, or simulate offline-safe synchronization
        let client = ureq::AgentBuilder::new()
            .timeout_connect(self.timeout)
            .timeout_read(self.timeout)
            .build();

        match client.get(&self.endpoint_url).call() {
            Ok(response) => {
                let mut reader = response.into_reader();
                let mut content = String::new();
                use std::io::Read;
                if let Err(e) = reader.read_to_string(&mut content) {
                    return Err(format!(
                        "Failed to read terminology response body: {}. Preserving local cache.",
                        e
                    ));
                }

                match serde_json::from_str::<Vec<super::types::DomainTerm>>(&content) {
                    Ok(terms) => {
                        let mut updated_db = current_db.clone();
                        updated_db.terms = terms;
                        updated_db.version = format!("{}_refreshed", current_db.version);
                        updated_db.rebuild_indexes();

                        if let Some(path) = cache_path {
                            if let Ok(serialized) = serde_json::to_string_pretty(&updated_db.terms) {
                                let _ = fs::write(path, serialized);
                            }
                        }

                        eprintln!(
                            "[Terminology API] Successfully refreshed {} terms from IATE endpoint.",
                            updated_db.terms.len()
                        );
                        Ok(updated_db)
                    }
                    Err(e) => Err(format!(
                        "Invalid JSON payload from terminology service: {}. Preserving local cache.",
                        e
                    )),
                }
            }
            Err(e) => {
                eprintln!(
                    "[Terminology API] Notice: Remote endpoint unreachable ({}) - preserving verified local database (Version: {}).",
                    e, current_db.version
                );
                // Return current DB unchanged (safe offline fallback)
                Ok(current_db.clone())
            }
        }
    }
}
