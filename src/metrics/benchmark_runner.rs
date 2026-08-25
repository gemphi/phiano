/// Benchmark runner: runs all metrics and produces a comprehensive report.

use crate::facet::Facet;
use crate::metrics::{
    adaptation::Adaptation,
    adversarial::Adversarial,
    arc::{ArcBenchmark, ArcResults},
    baseline::Baselines,
    generalization::Generalization,
    novelty_benchmark::NoveltyBenchmark,
    ood_detection::OodDetector,
    shortcut_detection::ShortcutDetector,
};
use crate::trainer::Trainer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport {
    pub baselines: (f64, f64, f64),
    pub brittleness: f64,
    pub ood_score: f64,
    pub adaptation_efficiency: f64,
    pub novel_task_score: f64,
    pub generalization: super::generalization::GeneralizationReport,
    pub arc_results: Option<ArcResults>,
    pub shortcut_warnings: Vec<super::shortcut_detection::ShortcutWarning>,
}

#[derive(Debug, Default)]
pub struct BenchmarkRunner;

impl BenchmarkRunner {
    /// Runs all benchmarks and returns a comprehensive report.
    pub fn run_all(facet: &mut Facet, trainer: &Trainer) -> BenchmarkReport {
        let prompt = "rust ownership borrowing lifetime";
        let baselines = Baselines::all(facet, prompt);
        let brittleness = Adversarial::brittleness(facet, prompt);
        let ood = OodDetector::score(facet, prompt);
        let adaptation = Adaptation::efficiency(facet, trainer, "ownership borrowing", 16);
        let novel = NoveltyBenchmark::task_score(facet, trainer, "quantum entanglement physics");

        let train_words: Vec<String> = facet.lexicon.keys().take(20).cloned().collect();
        let test_words: Vec<String> = facet.lexicon.keys().skip(20).take(10).cloned().collect();
        let novel_words: Vec<String> = facet.lexicon.keys().skip(30).take(10).cloned().collect();
        let generalization = Generalization::assess(facet, &train_words, &test_words, &novel_words);

        let arc_results = {
            let tasks = ArcBenchmark::load_tasks("data/arc_tasks.json");
            if tasks.is_empty() {
                None
            } else {
                Some(ArcBenchmark::evaluate(facet, trainer, &tasks))
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
        }
    }
}
