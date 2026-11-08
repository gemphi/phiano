//! Searle markers for phiano: phiton owns the type and the classifier; this
//! names the data file (`data/searle_markers.json`) and caches the read.

use std::sync::OnceLock;

pub use phiton::p64::SearleMarker;

static MARKERS: OnceLock<SearleMarker> = OnceLock::new();

/// Loads from data/searle_markers.json (cached via OnceLock).
pub fn load() -> &'static SearleMarker {
    MARKERS.get_or_init(|| SearleMarker::from_file(&crate::data::path("searle_markers.json")))
}
