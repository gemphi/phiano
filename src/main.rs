mod attention;
mod chunker;
mod cognitive;
mod command;
mod compose;
mod config;
mod curriculum;
mod drivers;
mod envision;
mod eval;
mod facet;
mod generate;
mod instruction;
mod layers;
mod memory;
mod model;
mod oscillator;
mod phase_flow;
mod persona;
mod phasor;
mod reasoning;
mod server;
mod sources;
mod storage;
mod synthetic;
mod tokenizer;
mod trainer;
mod wave;
mod wiki_bulk;

use command::help::Help;
use command::Context;
use model::Model;

/// Entry point - initializes the model, prints help, and runs the REPL.
///
/// "Phiano" - from *piano* (Italian: soft/loud), a phase instrument.
/// Words are keys, phasors are notes, sentences are chords.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let web_mode = args.iter().any(|a| a == "--web");
    let port: u16 = args
        .iter()
        .position(|a| a == "--port")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    println!("╔════════════════════════════════════════════════════════╗");
    println!("║  PHIANO - from *piano*: a phase instrument for language ║");
    println!("║  Words are keys · phasors are notes · sentences chords  ║");
    println!("║  Recursive learning: envision → apply → eval            ║");
    println!("║                   → iterate → scale                    ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    let mut model = Model::new();
    println!("Vocabulary: {} words\n", model.facet.vocabulary_size());

    if web_mode {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
        rt.block_on(server::run(model, port));
        return;
    }

    Help.apply(&mut Context {
        manifold: &mut model.facet,
        trainer: &model.trainer,
        memory: &mut model.memo,
        world: &mut model.world,
        context_buffer: &mut model.context_buffer,
        cognitive_core: &model.cognitive_core,
        arg: "",
        line: "",
    });

    model.run();
    model.scale();
    println!("  Goodbye.");
}
