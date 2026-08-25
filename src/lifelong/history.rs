/// Benchmark history tracking: records benchmark results over time.
/// Serializes to data/benchmark_history.json.

use crate::metrics::benchmark_runner::BenchmarkReport;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkEntry {
    pub timestamp: String,
    pub report: BenchmarkReport,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BenchmarkHistory {
    pub entries: Vec<BenchmarkEntry>,
}

impl BenchmarkHistory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a benchmark report with a timestamp.
    pub fn record(&mut self, report: BenchmarkReport) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs().to_string())
            .unwrap_or_else(|_| "0".to_string());
        self.entries.push(BenchmarkEntry { timestamp, report });
    }

    /// Returns the coherence trend over time.
    pub fn trend(&self) -> Vec<f64> {
        self.entries
            .iter()
            .map(|e| e.report.baselines.2)
            .collect()
    }

    /// Saves history to a JSON file.
    pub fn save(&self, path: &str) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, json);
        }
    }

    /// Loads history from a JSON file.
    pub fn load(path: &str) -> Self {
        match std::fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Returns the most recent entry.
    pub fn latest(&self) -> Option<&BenchmarkEntry> {
        self.entries.last()
    }
}
