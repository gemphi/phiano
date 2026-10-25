#[allow(unused_imports)]
use super::*;

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
        if !facet.has_ngrams() && !facet.lexicon.is_empty() {
            Self::bootstrap_bigrams(&mut facet);
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
    pub(super) fn bootstrap_bigrams(facet: &mut Facet) {
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
                    if line.is_empty() {
                        continue;
                    }
                    let _ = rl.add_history_entry(&line);
                    self.iterate(&line);
                    if line.eq_ignore_ascii_case("exit") || line.eq_ignore_ascii_case("quit") {
                        break;
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
}
