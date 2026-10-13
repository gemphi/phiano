#[allow(unused_imports)]
use super::*;

impl Model {

    /// One full cycle: envision → apply → eval → iterate → scale.
    ///
    /// Parses the command, creates a context, dispatches to the appropriate
    /// command handler, then runs the envision phase to detect gaps.
    pub(super) fn iterate(&mut self, line: &str) {
        let parts: Vec<&str> = line.splitn(2, char::is_whitespace).collect();
        let cmd = parts[0].to_lowercase();

        if cmd == "exit" || cmd == "quit" {
            self.scale();
            return;
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
    pub(super) fn recall_context(&self, line: &str) -> Vec<String> {
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
    pub(super) fn scale_quiet(&self) {
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
    pub(super) fn envision(&mut self, text: &str) {
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
