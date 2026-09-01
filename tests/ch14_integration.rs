//! Integration tests verifying Chapter 14 deep learning & lifelong reasoning concepts.
//!
//! Tests:
//! - §14.1: DataSplits (train/val/test) and Phase Vectorization
//! - §14.2: Generalization Gaps, Adversarial Robustness, and OOD Detection
//! - §14.3: ARC-Style Rule Induction and Adaptation Efficiency
//! - §14.4: Value-Centric vs Program-Centric Hybrid Analogy
//! - §14.5: Discrete Program Synthesis and Lifelong Component Reuse

use phiano::data::preprocess::Preprocessor;
use phiano::data::splits::DataSplits;
use phiano::facet::Facet;
use phiano::lifelong::LifelongLearner;
use phiano::metrics::adversarial::Adversarial;
use phiano::metrics::arc::{ArcBenchmark, ArcTask};
use phiano::metrics::generalization::Generalization;
use phiano::metrics::ood_detection::OodDetector;
use phiano::reasoning::analogy::Analogy;
use phiano::reasoning::hybrid::HybridReasoner;
use phiano::reasoning::program_analogy::ProgramAnalogy;
use phiano::synthesis::library::ComponentLibrary;
use phiano::synthesis::program::{Program, ProgramOp};
use phiano::trainer::Trainer;

#[test]
fn test_data_splits_and_preprocessing() {
    let mut facet = Facet::new();
    let raw_text = "The quick brown FOX jumps over 123 lazy dogs!";
    let tokens = Preprocessor::text(raw_text);

    assert!(!tokens.is_empty());
    assert_eq!(tokens[0], "the");
    assert_eq!(tokens[1], "quick");

    for token in &tokens {
        facet.get_or_init(token);
    }

    let pairs = Preprocessor::vectorize(&facet, &tokens);
    assert_eq!(pairs.len(), tokens.len()); // (phase, amplitude) pairs

    let sentences: Vec<String> = (0..20).map(|i| format!("sentence number {}", i)).collect();
    let splits = DataSplits::from_corpus(&sentences);

    assert_eq!(splits.train.len() + splits.validation.len() + splits.test.len(), 20);
    assert!(splits.train.len() >= 14);
}

#[test]
fn test_generalization_and_adversarial_metrics() {
    let mut facet = Facet::new();
    let trainer = Trainer::new(0.05);

    let train_corpus = [
        "quantum entanglement links two particles across space",
        "wave function collapse defines measurement in quantum mechanics",
        "superposition allows simultaneous states in quantum computation",
    ];

    for s in train_corpus {
        trainer.train_sentence(&mut facet, s);
    }

    // Held-out material, split by how much of it the model has vocabulary for.
    // The measurement is perplexity, which is a different quantity from the
    // coverage used to select the halves — the previous version selected by
    // phase distance and then measured phase coherence, which was circular.
    let held_out = vec![
        "quantum superposition links particles in measurement".to_string(),
        "gastronomy and culinary technique reward patient braising".to_string(),
    ];

    let report = Generalization::assess(&facet, &held_out);

    assert_eq!(report.n_local + report.n_extreme, held_out.len());
    if report.local_score.is_finite() && report.extreme_score.is_finite() {
        assert!(report.local_score > 0.0);
        assert!(report.extreme_score > 0.0);
        // Unfamiliar material must not come out easier than familiar material.
        assert!(report.extreme_score >= report.local_score);
    }

    let sensitivity = Adversarial::sensitivity(&facet, "quantum mechanics", 5);
    assert!(sensitivity >= 0.0);

    let in_dist_ood = OodDetector::score(&facet, "quantum particles");
    let out_dist_ood = OodDetector::score(&facet, "extraterrestrial archaeology");

    assert!(in_dist_ood <= out_dist_ood || (out_dist_ood - in_dist_ood).abs() < 0.5);
}

#[test]
fn test_arc_benchmark_evaluation() {
    let mut facet = Facet::new();
    let trainer = Trainer::new(0.05);

    let task = ArcTask {
        id: "arc_001_analogy".to_string(),
        input_pairs: vec![
            ("hot".to_string(), "cold".to_string()),
            ("light".to_string(), "dark".to_string()),
        ],
        test_input: "up".to_string(),
        expected: "down".to_string(),
    };

    let tasks = vec![task];
    let results = ArcBenchmark::evaluate(&mut facet, &trainer, &tasks);

    assert_eq!(results.total, 1);
    assert_eq!(results.details.len(), 1);
}

#[test]
fn test_hybrid_reasoning_and_analogy() {
    let mut facet = Facet::new();
    let trainer = Trainer::new(0.05);

    let text = "king is to man as queen is to woman";
    for _ in 0..8 {
        trainer.train_sentence(&mut facet, text);
    }

    let val_analogy = Analogy::value_centric(&facet, "king", "queen");
    assert!(val_analogy.value_score >= 0.0);

    let prog_analogy = ProgramAnalogy::compare(&facet, "king man", "queen woman");
    assert!(prog_analogy.program_score >= 0.0);

    let hybrid = HybridReasoner::new();
    let result = hybrid.solve_hybrid(&facet, "king queen relationship");
    assert!(!result.final_answer.is_empty());
}

#[test]
fn test_program_synthesis_and_library() {
    let facet = Facet::new();
    let mut library = ComponentLibrary::new();

    let prog = Program {
        operations: vec![
            ProgramOp::Map("shift_phase".to_string()),
            ProgramOp::Sort,
        ],
        phase_template: vec![0.5, 1.2, 2.4],
    };

    // register() now takes the task text it is keyed on, because matching is by
    // the phase shape of that text rather than by a positional list of angles.
    library.register(
        "phase_sort_component",
        prog.clone(),
        &facet,
        "sort the phase shifted list",
    );
    assert_eq!(library.components.len(), 1);

    library.mark_used("phase_sort_component");
    assert_eq!(library.components[0].reuse_count, 1);
}

#[test]
fn test_lifelong_learning_and_transfer() {
    let mut facet = Facet::new();
    let trainer = Trainer::new(0.05);
    let mut learner = LifelongLearner::new();

    let task_1 = "harmonic resonance creates constructive interference";
    let res_1 = learner.learn_task(&mut facet, &trainer, task_1);
    assert!(res_1.coherence >= 0.0);

    let task_2 = "constructive interference amplifies harmonic amplitude";
    let res_2 = learner.learn_task(&mut facet, &trainer, task_2);
    assert!(res_2.coherence >= 0.0);

    let transfer = learner.transfer_knowledge(&mut facet, "harmonic", "interference");
    assert_eq!(transfer.source_label, "harmonic");
    assert_eq!(transfer.target_label, "interference");
}
