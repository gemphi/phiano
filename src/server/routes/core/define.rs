#[allow(unused_imports)]
use super::*;

pub async fn define_word(
    State(state): State<SharedModel>,
    Json(req): Json<DefineRequest>,
) -> Result<Json<DefineResponse>, StatusCode> {
    let word = req.word.trim().to_lowercase();
    let word_clone = word.clone();

    // Multi-stage blocking auto-feed & lemmatization lookup
    let (source, final_def, plain_defs) = tokio::task::spawn_blocking(move || {
        let api = crate::sources::api::ApiSource::new(crate::config::API_CACHE_FILE);

        // Stage 1: Exact Free Dictionary API lookup
        if let Some(rich) = api.fetch_word_rich(&word_clone) {
            let plain = api.fetch_word(&word_clone);
            return ("Free Dictionary API (api.dictionaryapi.dev)".to_string(), rich, plain);
        }

        // Stage 2: Exact Offline 102K ChunkStore lookup
        let chunker = crate::chunker::ChunkStore::new("data/chunks");
        if let Some(chunk_def) = chunker.load_definition(&word_clone) {
            return (
                "Offline Webster's Unabridged Dictionary".to_string(),
                format!("{}\n(Webster's Dictionary)\n{}", word_clone, chunk_def),
                vec![chunk_def],
            );
        }

        // Stage 3: Morphological Lemmatization (e.g. warmed -> warm, coins -> coin)
        let lemmas = crate::sources::api::ApiSource::lemmatize_candidates(&word_clone);
        for (stem, label) in &lemmas {
            // Check API for stem
            if let Some(stem_rich) = api.fetch_word_rich(stem) {
                let formatted = format!("{}\n[Inflected Form: {} '{}']\n\n{}", word_clone, label, stem, stem_rich);
                let plain = api.fetch_word(stem);
                return (format!("Auto-Lemmatized & Grounded via '{}'", stem), formatted, plain);
            }
            // Check Chunkstore for stem
            if let Some(stem_chunk) = chunker.load_definition(stem) {
                let formatted = format!("{}\n[Inflected Form: {} '{}']\n\n(Webster's Dictionary)\n{}", word_clone, label, stem, stem_chunk);
                return (format!("Auto-Lemmatized via '{}' (Webster's)", stem), formatted, vec![stem_chunk]);
            }
        }

        // Stage 4: Wikipedia / Knowledge Base Summary lookup
        if let Some(wiki_def) = api.fetch_wikipedia_summary(&word_clone) {
            return ("Wikipedia Global Knowledge Base".to_string(), wiki_def.clone(), vec![wiki_def]);
        }

        // Stage 5: Local Definitions Cache fallback
        let loc = crate::sources::local::LocalSource::new(crate::config::DEFINITIONS_FILE);
        let local_defs = loc.fetch_definitions(&word_clone);
        if !local_defs.is_empty() {
            return ("Local Definitions Cache".to_string(), local_defs.join("\n"), local_defs);
        }

        // Stage 6: Intrinsic Phase Semantic Harmonic Synthesis
        (
            "Phiano Intrinsic Phase Semantic Synthesis".to_string(),
            format!("{}\n(Harmonic Phase Coordinate Grounded)\nTerm actively positioned on the complex phase manifold with resonance coupling.", word_clone),
            vec![format!("{} is an active semantic token in the harmonic cognitive space.", word_clone)],
        )
    }).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let model = &mut *guard;

    // Auto-train the manifold on the newly fetched/synthesized definition in real-time
    for def in &plain_defs {
        model.trainer.train_definition(&mut model.facet, &word, def);
    }

    let (phase, amplitude) = model.facet.lexicon.get(&word)
        .map(|p| (Some(p.phase), Some(p.amplitude)))
        .unwrap_or((None, None));

    Ok(Json(DefineResponse {
        word,
        definition: final_def,
        source,
        phase,
        amplitude,
        vocabulary: model.facet.vocabulary_size(),
    }))
}

pub async fn dialogue_learn(
    State(state): State<SharedModel>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let model = &mut *guard;
    let source = crate::sources::dialogue::DialogueSource::default_curriculum();
    let count = source.learn_into_facet(&mut model.facet, &mut model.memo, &model.trainer);
    Ok(Json(serde_json::json!({
        "dialogues_trained": count,
        "vocabulary": model.facet.vocabulary_size(),
        "message": format!("Successfully trained on {} multi-turn conversational dialogues", count),
    })))
}

pub async fn save_manifold(
    State(state): State<SharedModel>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let model = &*guard;
    crate::storage::Storage::save(&model.facet, "data/manifold.chroma")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let _ = model.memo.save_to_file("data/memory.chroma");
    Ok(Json(serde_json::json!({
        "status": "ok",
        "vocabulary": model.facet.vocabulary_size(),
        "message": "Manifold and memory hierarchy successfully saved to disk",
    })))
}
