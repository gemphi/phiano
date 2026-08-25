/// Deployment monitoring: detects drift and regression in production.
/// Implements Ch 14.5's monitoring discipline.

use crate::facet::Facet;
use crate::lifelong::history::BenchmarkHistory;
use crate::metrics::benchmark_runner::BenchmarkReport;
use crate::metrics::ood_detection::ood_score;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Alert {
    pub alert_type: String,
    pub message: String,
    pub severity: f64,
}

pub struct ModelMonitor {
    pub history: BenchmarkHistory,
    pub alerts: Vec<Alert>,
}

impl Default for ModelMonitor {
    fn default() -> Self {
        Self {
            history: BenchmarkHistory::new(),
            alerts: Vec::new(),
        }
    }
}

impl ModelMonitor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Detects if recent inputs are drifting from training distribution.
    pub fn check_drift(&self, facet: &Facet, recent_inputs: &[String]) -> Option<Alert> {
        let mut max_ood = 0.0;
        for input in recent_inputs {
            let score = ood_score(facet, input);
            if score > max_ood {
                max_ood = score;
            }
        }

        if max_ood > 0.7 {
            Some(Alert {
                alert_type: "distribution_drift".to_string(),
                message: format!("Input drift detected: OOD score {:.4}", max_ood),
                severity: max_ood,
            })
        } else {
            None
        }
    }

    /// Detects if performance has regressed from a previous benchmark.
    pub fn check_regression(&self, current: &BenchmarkReport) -> Option<Alert> {
        let prev = self.history.latest()?;
        let delta = current.baselines.2 - prev.report.baselines.2;

        if delta < -0.05 {
            Some(Alert {
                alert_type: "performance_regression".to_string(),
                message: format!("Coherence regression: {:.4} → {:.4} ({:+.4})",
                    prev.report.baselines.2, current.baselines.2, delta),
                severity: (-delta).min(1.0),
            })
        } else {
            None
        }
    }

    /// Records a benchmark and checks for alerts.
    pub fn update(&mut self, report: BenchmarkReport) -> Vec<Alert> {
        let mut new_alerts = Vec::new();

        if let Some(alert) = self.check_regression(&report) {
            new_alerts.push(alert);
        }

        self.history.record(report);
        self.alerts.extend(new_alerts.clone());
        new_alerts
    }
}
