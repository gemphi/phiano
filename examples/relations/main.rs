//! Relation-accuracy runner.
//!
//! `cargo run --release --bin relations -- [max_entries] [epochs]`
//!
//! Trains the manifold on dictionary definitions and asks whether it places
//! related words in related positions: is `woman` near `man`, and is the step
//! from `man` to `woman` the same step as from `grandfather` to `grandmother`?
//!
//! Every score is printed against its chance baseline, because a number without
//! one cannot be judged.

mod footprint;
mod objective;
mod report;
mod setup;
mod train;
mod verdict;
#[allow(unused_imports)]
use footprint::*;
#[allow(unused_imports)]
use objective::*;
#[allow(unused_imports)]
use report::*;
#[allow(unused_imports)]
use setup::*;
#[allow(unused_imports)]
use train::*;
#[allow(unused_imports)]
use verdict::*;

use phiano::chunker::ChunkStore;
use phiano::cognitive::DefinitionGrounder;
use phiano::config::LEARNING_RATE;
use phiano::facet::Facet;
use phiano::metrics::relation::{RelationBenchmark, RelationFamily, RelationReport};
use phiano::trainer::Trainer;

fn main() {
    let families = RelationBenchmark::default_families();
    let Setup { epochs, store, selected } = setup(&families);

    let mut trained = train_regimes(&selected, epochs);
    print_objective_table(&trained, &families);

    // Detailed per-family view for the strongest regime, plus the grounding
    // ablation, using the co-occurrence model for continuity with earlier runs.
    let mut facet = trained.remove(0).1;
    let before = RelationBenchmark::evaluate(&facet, &families);
    print_report("per-family, co-occurrence training", &before);

    DefinitionGrounder::ground_phases(&mut facet, &store);
    println!("dispersion after grounding: {:.4}", facet.phase_dispersion());
    let after = RelationBenchmark::evaluate(&facet, &families);
    print_report("per-family, after definition grounding", &after);

    print_verdict(&before, &after);
    print_footprint(&facet);

    if let Ok(j) = serde_json::to_string_pretty(&(&before, &after)) {
        let _ = std::fs::write("data/relations.json", j);
        println!("\nfull report written to data/relations.json");
    }
}
