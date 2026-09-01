use crate::chunker::ChunkStore;
use crate::command::{Context, Dispatcher};
use crate::config;
use crate::cognitive::{CognitiveCore, DefinitionGrounder};
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

/// Model - the recursive learning agent.
///
/// Operates in a continuous cycle:
///
///   envision → apply → eval → iterate → scale
///
/// Each user input triggers this cycle. The model envisions what it
/// doesn't know, applies training, evaluates understanding, iterates
/// on gaps, and scales by persisting knowledge.
pub struct Model {
    /// Turns since the last automatic checkpoint.
    turns_since_save: usize,
    /// The facet - lexicon of words mapped to complex phasors.
    pub facet: Facet,
    /// The trainer - Kuramoto phase attraction learning engine.
    pub trainer: Trainer,
    /// The memo - 16-layer memory log of all interactions.
    pub memo: Memo,
    /// The world - collection of personas for impersonation.
    pub world: World,
    /// The running multi-turn context wave buffer.
    pub context_buffer: crate::generate::ContextWaveBuffer,
    /// The cognitive core - 16 Searle-inspired agents.
    pub cognitive_core: CognitiveCore,
    /// Journal of taught corrections, replayed after grounding.
    pub corrections: crate::correction::CorrectionLog,
    /// Gap detection, carrying its ledger across turns.
    pub envisioner: Envision,
    /// Local dictionary, consulted before the user is asked about a gap.
    pub chunk_store: ChunkStore,
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
            Err(e) => {
                // Never fail silently into an empty model: a corrupt or
                // future-format file used to look exactly like a first run.
                if std::path::Path::new(config::CHROMA_FILE).exists() {
                    eprintln!(
                        "  [WARN] could not load {}: {} — starting from an empty lexicon",
                        config::CHROMA_FILE, e
                    );
                }
                let _ = fs::create_dir_all("data");
                Facet::new()
            }
        };

        // Fill phase channels for any phasor saved before the lexicon carried
        // them, preserving each word's learned base phase on channel 0.
        let migrated = facet.migrate_channels();
        if migrated > 0 {
            println!("  [migrate] {} phasors given phase channels", migrated);
        }

        // Bootstrap bigrams from chunk data if empty (legacy model compat)
        match !facet.has_ngrams() && !facet.lexicon.is_empty() {
            true => Self::bootstrap_bigrams(&mut facet),
            false => {}
        }

        // Definition-grounded phase re-seeding. Runs once per grounding
        // version rather than on every startup: it is a full pass over the
        // dictionary, and re-running it meant the model that started was never
        // quite the model that was saved.
        if !facet.lexicon.is_empty() && facet.grounded_version < config::GROUNDING_VERSION {
            DefinitionGrounder::ground_best(&mut facet, &ChunkStore::new("data/chunks"));
        }

        let cognitive_core = CognitiveCore::new(ChunkStore::new("data/chunks"));

        let memo = Memo::load_from_file(config::MEMORY_FILE)
            .unwrap_or_else(|_| Memo::new());

        // Replay what the user taught. Grounding and bootstrap both rewrite
        // phases from source data, which would otherwise silently undo every
        // correction the user has ever made.
        let corrections = crate::correction::CorrectionLog::load(config::CORRECTION_FILE);
        if !corrections.is_empty() {
            let trainer = Trainer::new(config::LEARNING_RATE);
            let n = corrections.replay(&mut facet, &trainer);
            println!("  [corrections] replayed {} taught correction(s)", n);
        }

        Self {
            turns_since_save: 0,
            corrections,
            envisioner: Envision::new(),
            chunk_store: ChunkStore::new("data/chunks"),
            facet,
            trainer: Trainer::new(config::LEARNING_RATE),
            memo,
            world: World::new(),
            context_buffer: crate::generate::ContextWaveBuffer::new(4096),
            cognitive_core,
        }
    }

    /// Bootstraps bigram co-occurrence counts from chunk dictionary data.
    /// Only records word-pair adjacencies - does NOT retrain phases.
    /// This is fast (~2-5 seconds) and populates the transition model
    /// for generation and composition.
    fn bootstrap_bigrams(facet: &mut Facet) {
        use std::time::Instant;
        let start = Instant::now();
        let chunk_store = crate::chunker::ChunkStore::new("data/chunks");
        let entries = chunk_store.load_all();
        match entries.is_empty() {
            true => return,
            false => {}
        }

        println!("  [bigram] Bootstrapping from {} definitions...", entries.len());
        let mut count = 0usize;
        for (_word, def) in &entries {
            let tokens = Tokenizer::tokenize(def);
            for window in tokens.windows(2) {
                // Only record if both words are already in the lexicon
                match facet.lexicon.contains_key(&window[0]) && facet.lexicon.contains_key(&window[1]) {
                    true => {
                        facet.record_bigram(&window[0], &window[1]);
                        count += 1;
                    }
                    false => {}
                }
            }
        }

        println!(
            "  [bigram] {} co-occurrences recorded in {:?}",
            count,
            start.elapsed()
        );
    }

    /// Runs the REPL loop - each input is one iteration of the cycle.
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
                    match line.is_empty() {
                        true => continue,
                        false => {}
                    }
                    let _ = rl.add_history_entry(&line);
                    self.iterate(&line);
                    match line.eq_ignore_ascii_case("exit") || line.eq_ignore_ascii_case("quit") {
                        true => break,
                        false => {}
                    }
                }
                // Ctrl-C, Ctrl-D or a closed stream. Persist before leaving:
                // this path previously exited without saving, so an interrupted
                // session lost everything it had learned.
                Err(_) => {
                    println!();
                    self.scale();
                    break;
                }
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

        match cmd == "exit" || cmd == "quit" {
            true => {
                self.scale();
                return;
            }
            false => {}
        }

        let arg = parts.get(1).map(|s| s.trim()).unwrap_or("");

        // Prime the context wave with relevant past turns before dispatching.
        let recalled = self.recall_context(line);
        for past in &recalled {
            self.context_buffer.push_turn(&self.facet, past);
        }

        let mut ctx = Context {
            manifold: &mut self.facet,
            trainer: &self.trainer,
            memory: &mut self.memo,
            world: &mut self.world,
            context_buffer: &mut self.context_buffer,
            cognitive_core: &self.cognitive_core,
            corrections: &mut self.corrections,
            gaps: Some(&self.envisioner.ledger),
            arg,
            line,
        };

        match Dispatcher::dispatch(line, &mut ctx) {
            true => {}
            false => return,
        }

        // Record the interaction under its order-sensitive wave, so that recall
        // can tell "dog bites man" from "man bites dog".
        let wave = crate::wave::Wave::text_bound(&self.facet, line);
        self.memo.record_grounded(&self.facet, (wave.re, wave.im), line);

        self.envision(line);

        // Checkpoint periodically. Persistence used to happen only on a clean
        // `exit`, so a crash or a power loss discarded the session.
        self.turns_since_save += 1;
        if self.turns_since_save >= config::CHECKPOINT_EVERY_TURNS {
            self.scale_quiet();
            self.turns_since_save = 0;
        }
    }

    /// Pulls the most relevant past interactions into the working context.
    ///
    /// The memory log has recorded every interaction's wave since the beginning;
    /// nothing read it back. This is what makes the session conversational
    /// rather than stateless per turn.
    fn recall_context(&self, line: &str) -> Vec<String> {
        if self.memo.is_empty() {
            return Vec::new();
        }
        let q = crate::wave::Wave::text_bound(&self.facet, line);
        self.memo
            .recall_weighted((q.re, q.im), config::RECALL_K, config::RECALL_HALF_LIFE_MS)
            .into_iter()
            .filter(|e| !e.text.is_empty() && e.text != line)
            .map(|e| e.text.clone())
            .collect()
    }

    /// Persists without printing a banner — used for automatic checkpoints.
    fn scale_quiet(&self) {
        let _ = Storage::save(&self.facet, config::CHROMA_FILE);
        let _ = self.memo.save_to_file(config::MEMORY_FILE);
        let _ = self.corrections.save(config::CORRECTION_FILE);
    }

    /// Envision phase: detect knowledge gaps and close them where possible.
    ///
    /// Unknown words are resolved against the local dictionary first and only
    /// escalated to the user when that fails, so the loop is autonomous in the
    /// common case and respectful of attention in the rare one. Anything it
    /// resolves is learned properly, definition chain and all.
    fn envision(&mut self, text: &str) {
        let vision = self.envisioner.detect_gaps(
            &mut self.facet,
            Some(&self.chunk_store),
            text,
        );

        if let Some(v) = vision {
            for word in &v.auto_learned {
                self.trainer.learn_definition_chain(
                    &mut self.facet,
                    &self.chunk_store,
                    word,
                    config::DEFINITION_CHAIN_DEPTH,
                );
            }
            println!("{}", v);
        }
    }

    /// Scale phase: persist the facet and memory to disk.
    ///
    /// Called on exit to ensure all learned knowledge is saved.
    pub fn scale(&self) {
        if let Err(e) = Storage::save(&self.facet, config::CHROMA_FILE) {
            eprintln!("  [ERROR] could not save {}: {}", config::CHROMA_FILE, e);
            return;
        }
        if let Err(e) = self.memo.save_to_file(config::MEMORY_FILE) {
            eprintln!("  [ERROR] could not save {}: {}", config::MEMORY_FILE, e);
        }
        if let Err(e) = self.corrections.save(config::CORRECTION_FILE) {
            eprintln!("  [ERROR] could not save {}: {}", config::CORRECTION_FILE, e);
        }
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
