/// Benchmark runner: runs all metrics and produces a comprehensive report.

use crate::eval::Evaluator;
use crate::facet::Facet;
use crate::metrics::{
    adaptation::adaptation_efficiency,
    adversarial::brittleness_score,
    arc::{evaluate_arc, load_arc_tasks, ArcResults},
    baseline::all_baselines,
    generalization::assess_generalization,
    novelty_benchmark::novel_task_score,
    ood_detection::ood_score,
    shortcut_detection::detect_shortcuts,
};
use crate::trainer::Trainer;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
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

/// Runs all benchmarks and returns a comprehensive report.
pub fn run_all(facet: &mut Facet, trainer: &Trainer) -> BenchmarkReport {
    let prompt = "rust ownership borrowing lifetime";
    let baselines = all_baselines(facet, prompt);
    let brittleness = brittleness_score(facet, prompt);
    let ood = ood_score(facet, prompt);
    let adaptation = adaptation_efficiency(facet, trainer, "ownership borrowing", 16);
    let novel = novel_task_score(facet, trainer, "quantum entanglement physics");

    let train_words: Vec<String> = facet.lexicon.keys().take(20).cloned().collect();
    let test_words: Vec<String> = facet.lexicon.keys().skip(20).take(10).cloned().collect();
    let novel_words: Vec<String> = facet.lexicon.keys().skip(30).take(10).cloned().collect();
    let generalization = assess_generalization(facet, &train_words, &test_words, &novel_words);

    let arc_results = {
        let tasks = load_arc_tasks("data/arc_tasks.json");
        if tasks.is_empty() {
            None
        } else {
            Some(evaluate_arc(facet, trainer, &tasks))
        }
    };

    let response = format!("{} is a concept", prompt);
    let shortcuts = detect_shortcuts(facet, prompt, &response);

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
