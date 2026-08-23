use crate::command::Context;

/// Help — prints the list of available commands.
pub struct Help;

impl Help {
    /// Prints the help text to stdout.
    pub fn apply(&self, _ctx: &mut Context) -> bool {
        println!("Core commands:");
        println!("  learn \"text\"               — Train on a sentence (online learning)");
        println!("  define <word>              — Fetch & learn a word's definition");
        println!("  eval \"text\"                — Score text: coherence, novelty, resonance");
        println!("  compose \"prompt\" [rounds]  — Recursive sector composition");
        println!("  generate \"prompt\"          — Phase-guided sequence generation (Phase 2)");
        println!("  instruct \"instruction\"     — Execute prompt via instruction engine (Phase 4)");
        println!("  reason \"problem\"           — Phase-space pathfinding reasoning chain (Phase 6)");
        println!("  layers [query]             — Inspect 4-layer hierarchical phase depth (Phase 3)");
        println!("  synthetic [pairs|pipeline] — Self-curriculum synthetic generation (Phase 5)");
        println!("  oscillator eval \"text\"        — Evaluate text in oscillator mode");
        println!("  oscillator wheel            — Show the oscillator color wheel");
        println!("  oscillator sphere \"text\"    — Show sphere projection for text");
        println!("  oscillator compare \"text\"   — Compare transform vs oscillator models");
        println!("  oscillator train \"text\" [n] — Train using oscillator sync (n epochs)");
        println!("  persona add <name> \"ex1\" ... — Create persona from examples");
        println!("  persona from <name> \"text\"   — Create persona from a text block");
        println!("  persona impersonate <name> \"prompt\" — Compose as persona");
        println!("  persona match \"unknown text\" — Attribute text to closest persona");
        println!("  persona chat <name>          — Chat with a persona interactively");
        println!("  synonym <word> [n]         — Find n nearest resonant words");
        println!("  resonance \"text\" [n]       — Find words resonating with a sentence wave");
        println!("  wave \"text\"                — Show the sentence's complex wave");
        println!("  save                       — Save facet to disk");
        println!("  load                       — Load facet from disk");
        println!("  stats                      — Show facet statistics");
        println!("  help                       — Show this help");
        println!("  exit                       — Save and quit");
        println!();
        println!("Drivers (source/device commands):");
        println!("  ingest <file.txt>          — Bulk ingest local definitions");
        println!("  ingest-json <file.json> [n]— Bulk ingest JSON dictionary");
        println!("  ingest-wiktionary <file>   — Bulk ingest Wiktionary JSON/JSONL dump");
        println!("  chunk <dictionary.json>    — Split large dictionary into data/chunks/");
        println!("  train [epochs]             — Train from all chunks in parallel");
        println!();
        println!("  Any unrecognized input is treated as text to learn from.");
        println!();
        true
    }
}
