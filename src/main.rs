//! The REPL binary.
//!
//! This file used to re-declare all forty-five modules with `mod`, which meant
//! the whole crate compiled twice — once as the library every experiment binary
//! links against, and again inside this binary, where the REPL touches maybe a
//! third of it. The second copy produced a hundred and seven dead-code warnings
//! for code that is very much alive on the library side, and those drowned the
//! real ones. It now links the library like every other binary does.

use phiano::command::help::Help;
use phiano::command::Context;
use phiano::model::Model;
use phiano::server;

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
        corrections: &mut model.corrections,
        gaps: Some(&model.envisioner.ledger),
        arg: "",
        line: "",
    });

    model.run();
    model.scale();
    println!("  Goodbye.");
}
