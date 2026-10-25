//! Does role binding beat the bag on real data?
//!
//! `cargo run --release --bin roles -- [chunks] [corpus]`
//!
//! Positional binding failed three times (definitions, LM context, sentence
//! composition), and the diagnosis was that position is a **nonce** role: the
//! same concept lands at a different angle in every context, so superposed
//! copies cancel. A role rotation is shared — every `genus` binds at the same
//! angle everywhere — so the relation becomes a direction rather than noise.
//!
//! That is a claim about recoverability, and this measures it on the dictionary
//! rather than on a fixture.
//!
//! # The task
//!
//! Take the relation benchmark's **hypernym** family — `(dog, animal)`,
//! `(oak, tree)`, `(salmon, fish)` — which is exactly the genus relation.
//! Compose each specific term from the rules extracted from its own Webster's
//! gloss, then ask the composition what its genus is. Correct means the true
//! hypernym comes back.
//!
//! Two scorers on identical pairs:
//!
//! * **role query** — `identity ⊖ genus`, then nearest word.
//! * **bag** — the same fillers superposed with *no* role rotation, then
//!   nearest word. The control: everything role binding has, minus the roles.
//!
//! If they tie, the rotation is decoration. Only a gap is evidence.

mod coherence;
mod discovery;
mod floor;
mod genus;
mod function;
mod lexical;
mod bands;
mod polysemy;
mod senses;
mod verdict;
mod setup;
#[allow(unused_imports)]
use coherence::*;
#[allow(unused_imports)]
use discovery::*;
#[allow(unused_imports)]
use floor::*;
#[allow(unused_imports)]
use genus::*;
#[allow(unused_imports)]
use function::*;
#[allow(unused_imports)]
use lexical::*;
#[allow(unused_imports)]
use bands::*;
#[allow(unused_imports)]
use polysemy::*;
#[allow(unused_imports)]
use senses::*;
#[allow(unused_imports)]
use verdict::*;
#[allow(unused_imports)]
use setup::*;

use phiano::chunker::ChunkStore;
use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiano::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::metrics::relation::RelationBenchmark;
use phiano::phasor::SpectralPhasor;
use phiano::roles::{Role, RuleSet, RoleTable};
use phiano::sources::definition_core;
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;
use std::collections::HashMap;

fn main() {
    let Setup { chunks, facet, glosses, rules, fillers_of } = setup();

    genus_experiment(&facet, &rules, &fillers_of);

    // The CLU question: can the types be discovered rather than declared?
    discovery_experiment(&facet);

    // And the blocker discovery exposed: can coherence be trained?
    coherence_experiment(&facet, &rules);

    // The unblocking attempt: let the language name the relations.
    lexical_experiment(&facet, &glosses);

    // And the gap that explains why it stays faint.
    polysemy_experiment(&facet, &chunks);

    // Does the headline survive restricting to vocabulary that was trained?
    floor_experiment(&facet);

    // Hierarchy, as a demonstration rather than a metric.
    println!("\n--- genus chains ---");
    for w in ["cow", "dog", "oak", "money", "salmon"] {
        let path = RoleTable::ascend(&rules, w, Role::Genus, 5);
        if path.len() > 1 {
            println!("  {}", path.join(" → "));
        }
    }
}
