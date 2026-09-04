pub mod api;
pub mod db;
pub mod detector;
pub mod types;

pub use api::TerminologyApiClient;
pub use db::{TerminologyDb, DEFAULT_TERMINOLOGY_VERSION};
pub use detector::DomainDetector;
pub use types::{
    DomainTerm, MatchedTermSpan, TermRelation, TermStatus, TerminologyConfig, TerminologyMetrics,
    TerminologyRejectionDiagnostic,
};
