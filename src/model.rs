use crate::chunker::ChunkStore;
use crate::command::{Context, Dispatcher};
use crate::config;
use crate::cognitive::{definition_ground_phases, CognitiveCore};
use crate::envision::Envision;
use crate::facet::Facet;
use crate::memory::Memo;
use crate::persona::World;
use crate::storage::Storage;
use crate::tokenizer::Tokenizer;
use crate::trainer::Trainer;
use rustyline::Editor;
use rustyline::history::DefaultHistory;
use std::fs;

/// Model — the recursive learning agent.
///
/// Operates in a continuous cycle:
///
///   envision → apply → eval → iterate → scale
///
/// Each user input triggers this cycle. The model envisions what it
/// doesn't know, applies training, evaluates understanding, iterates
/// on gaps, and scales by persisting knowledge.
pub struct Model {
    /// The facet — lexicon of words mapped to complex phasors.
    pub facet: Facet,
    /// The trainer — Kuramoto phase attraction learning engine.
    pub trainer: Trainer,
    /// The memo — 16-layer memory log of all interactions.
    pub memo: Memo,
    /// The world — collection of personas for impersonation.
    pub world: World,
    /// The running multi-turn context wave buffer.
    pub context_buffer: crate::generate::ContextWaveBuffer,
    /// The cognitive core — 16 Searle-inspired agents.
    pub cognitive_core: CognitiveCore,
}

impl Model {
    /// Creates a new model, loading the facet and memory from disk if available.
    ///
    /// If no saved data exists, starts with an empty facet and memory.
    pub fn new() -> Self {
        let mut facet = match Storage::load(config::CHROMA_FILE) {
            Ok(m) => {
                println!("  [loaded] {} words", m.vocabulary_size());
                m
            }
            Err(_) => {
                let _ = fs::create_dir_all("data");
                Facet::new()
            }
        };

        // Bootstrap bigrams from chunk data if empty (legacy model compat)
        if facet.bigrams.is_empty() && !facet.lexicon.is_empty() {
            Self::bootstrap_bigrams(&mut facet);
        }

        // Definition-grounded phase re-seeding
        // Replaces word.len()*PHI with definition centroid phases
        if !facet.lexicon.is_empty() {
            definition_ground_phases(&mut facet, &ChunkStore::new("data/chunks"));
        }

        let cognitive_core = CognitiveCore::new(ChunkStore::new("data/chunks"));

        let memo = Memo::load_from_file(config::MEMORY_FILE)
            .unwrap_or_else(|_| Memo::new());

        Self {
            facet,
            trainer: Trainer::new(config::LEARNING_RATE),
            memo,
            world: World::new(),
            context_buffer: crate::generate::ContextWaveBuffer::new(4096),
            cognitive_core,
        }
    }

    /// Bootstraps bigram co-occurrence counts from chunk dictionary data.
    /// Only records word-pair adjacencies — does NOT retrain phases.
    /// This is fast (~2-5 seconds) and populates the transition model
    /// for generation and composition.
    fn bootstrap_bigrams(facet: &mut Facet) {
        use std::time::Instant;
        let start = Instant::now();
        let chunk_store = crate::chunker::ChunkStore::new("data/chunks");
        let entries = chunk_store.load_all();
        if entries.is_empty() {
            return;
        }

        println!("  [bigram] Bootstrapping from {} definitions...", entries.len());
        let mut count = 0usize;
        for (_word, def) in &entries {
            let tokens = Tokenizer::tokenize(def);
            for window in tokens.windows(2) {
                // Only record if both words are already in the lexicon
                if facet.lexicon.contains_key(&window[0]) && facet.lexicon.contains_key(&window[1]) {
                    facet.record_bigram(&window[0], &window[1]);
                    count += 1;
                }
            }
        }

        println!(
            "  [bigram] {} co-occurrences recorded in {:?}",
            count,
            start.elapsed()
        );
    }

    /// Runs the REPL loop — each input is one iteration of the cycle.
    ///
    /// Reads lines from stdin until the user types `exit` or `quit`.
    /// Each line is processed through the `iterate` method.
    pub fn run(&mut self) {
        let mut rl = Editor::<(), DefaultHistory>::new()
            .expect("Failed to init readline");

        loop {
            match rl.readline("phiano> ") {
                Ok(raw) => {
                    let line = raw.trim().to_string();
                    if line.is_empty() {
                        continue;
                    }
                    let _ = rl.add_history_entry(&line);
                    self.iterate(&line);
                    if line.eq_ignore_ascii_case("exit") || line.eq_ignore_ascii_case("quit") {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    }

    /// One full cycle: envision → apply → eval → iterate → scale.
    ///
    /// Parses the command, creates a context, dispatches to the appropriate
    /// command handler, then runs the envision phase to detect gaps.
    fn iterate(&mut self, line: &str) {
        let parts: Vec<&str> = line.splitn(2, char::is_whitespace).collect();
        let cmd = parts[0].to_lowercase();

        if cmd == "exit" || cmd == "quit" {
            self.scale();
            return;
        }

        let arg = parts.get(1).map(|s| s.trim()).unwrap_or("");
        let mut ctx = Context {
            manifold: &mut self.facet,
            trainer: &self.trainer,
            memory: &mut self.memo,
            world: &mut self.world,
            context_buffer: &mut self.context_buffer,
            arg,
            line,
        };

        if !Dispatcher::dispatch(line, &mut ctx) {
            return;
        }

        let wave = crate::wave::Wave::text(&self.facet, line);
        self.memo.record((wave.re, wave.im), line);

        self.envision(line);
    }

    /// Envision phase: detect knowledge gaps and project what to learn next.
    ///
    /// After each command, checks for unknown words in the input and
    /// suggests related known words that might help define them.
    fn envision(&self, text: &str) {
        if let Some(v) = Envision::new().detect_gaps(&self.facet, text) {
            println!("{}", v);
        }
    }

    /// Scale phase: persist the facet and memory to disk.
    ///
    /// Called on exit to ensure all learned knowledge is saved.
    pub fn scale(&self) {
        let _ = Storage::save(&self.facet, config::CHROMA_FILE);
        let _ = self.memo.save_to_file(config::MEMORY_FILE);
        println!(
            "  [saved] {} ({} words)",
            config::CHROMA_FILE,
            self.facet.vocabulary_size(),
        );
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
