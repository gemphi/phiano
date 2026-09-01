/// Benchmark runner: runs all metrics and produces a comprehensive report.

use crate::facet::Facet;
use crate::metrics::{
    adaptation::Adaptation,
    adversarial::Adversarial,
    arc::{ArcBenchmark, ArcResults},
    baseline::{BaselineScores, Baselines},
    generalization::{Generalization, GeneralizationReport},
    novelty_benchmark::NoveltyBenchmark,
    ood_detection::OodDetector,
    shortcut_detection::ShortcutDetector,
};
use crate::trainer::Trainer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport {
    /// Held-out perplexity of every baseline. `None` when no evaluation corpus
    /// is on disk — reported as absent rather than filled with a placeholder.
    pub baselines: Option<BaselineScores>,
    pub brittleness: f64,
    pub ood_score: f64,
    pub adaptation_efficiency: f64,
    pub novel_task_score: f64,
    pub generalization: GeneralizationReport,
    pub arc_results: Option<ArcResults>,
    pub shortcut_warnings: Vec<super::shortcut_detection::ShortcutWarning>,
    /// 1.0 = phases spread uniformly, 0.0 = every word at one angle.
    /// Logged beside every score because coherence rises as this falls.
    pub phase_dispersion: f64,
    /// The same measure over the frequent band only — the words every scored
    /// task actually draws on. The global figure is a tail average and stays
    /// near 1.0 while the band collapses, so this is the one the alarm watches.
    /// `serde(default)` because reports written before this field existed
    /// deserialise as 0.0, which the monitor would read as total collapse.
    #[serde(default = "one")]
    pub band_dispersion: f64,
    /// Sector-occupancy inequality; rises as the manifold concentrates.
    pub sector_gini: f64,
}

fn one() -> f64 {
    1.0
}

#[derive(Debug, Default)]
pub struct BenchmarkRunner;

impl BenchmarkRunner {
    /// Runs all benchmarks and returns a comprehensive report.
    pub fn run_all(facet: &mut Facet, trainer: &Trainer) -> BenchmarkReport {
        let prompt = "rust ownership borrowing lifetime";
        let split = Baselines::load_split();

        let baselines = split.as_ref().map(|s| Baselines::suite(facet, s));
        let generalization = match &split {
            Some(s) => Generalization::assess(facet, &s.valid),
            None => GeneralizationReport::default(),
        };

        let brittleness = Adversarial::brittleness(facet, prompt);
        let ood = OodDetector::score(facet, prompt);
        let adaptation = Adaptation::efficiency(facet, trainer, "ownership borrowing", 16);
        let novel = NoveltyBenchmark::task_score(facet, trainer, "quantum entanglement physics");

        let arc_results = {
            let tasks = ArcBenchmark::load_tasks("data/arc_tasks.json");
            match tasks.is_empty() {
                true => None,
                false => Some(ArcBenchmark::evaluate(facet, trainer, &tasks)),
            }
        };

        let response = format!("{} is a concept", prompt);
        let shortcuts = ShortcutDetector::detect(facet, prompt, &response);

        BenchmarkReport {
            baselines,
            brittleness,
            ood_score: ood,
            adaptation_efficiency: adaptation,
            novel_task_score: novel,
            generalization,
            arc_results,
            shortcut_warnings: shortcuts,
            phase_dispersion: facet.phase_dispersion(),
            band_dispersion: facet.dispersion_top(crate::cognitive::grounding::GUARD_BAND_TOP),
            sector_gini: facet.sector_gini(),
        }
    }
}
