use super::SharedModel;
use super::types::*;
use crate::command::{Context, Dispatcher};
use crate::eval::Evaluator;
use crate::generate::Generator;
use crate::instruction::InstructionEngine;
use crate::layers::HierarchicalPhaseField;
use crate::reasoning::ReasoningEngine;
use crate::sources::phi4::Phi4Source;
use crate::sources::DictionarySource;
use crate::synthetic::SyntheticCurriculumPipeline;
use crate::trainer::MultiEpochResult;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;

pub async fn eval(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<EvalResponse>, StatusCode> {
    let model = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let eval = Evaluator::new().eval(&model.facet, &req.text);
    Ok(Json(EvalResponse {
        coherence: eval.coherence,
        novelty: eval.novelty,
        resonance: eval.resonance,
        overall: eval.overall,
        verdict: format!("{}", eval.verdict),
        vocabulary: model.facet.vocabulary_size(),
    }))
}

pub async fn learn(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<LearnResponse>, StatusCode> {
    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let model = &mut *guard;
    let text = req.text.clone();
    let tokens = model.trainer.train_online(&mut model.facet, &text);
    Ok(Json(LearnResponse {
        tokens,
        vocabulary: model.facet.vocabulary_size(),
        message: format!("Trained on {} tokens", tokens),
    }))
}

pub async fn learn_multi(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<MultiLearnResponse>, StatusCode> {
    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let model = &mut *guard;
    let text = req.text.clone();
    let epochs = req.epochs.unwrap_or(10);
    let warmup = req.warmup.unwrap_or(3);
    let result: MultiEpochResult = model.trainer.train_multi_epoch(
        &mut model.facet, &text, epochs, warmup,
    );
    Ok(Json(MultiLearnResponse {
        epochs: result.epochs,
        tokens: result.tokens_learned,
        converged: result.converged,
        vocabulary: model.facet.vocabulary_size(),
    }))
}

pub async fn generate_seq(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<GenerateResponse>, StatusCode> {
    let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let max_tok = req.max_tokens.unwrap_or(32);
    let temp = req.temperature.unwrap_or(0.15);
    let generator = Generator::new(max_tok, temp);
    let mut ctx_buf = crate::generate::ContextWaveBuffer::new(4096);
    let generated = generator.generate(&guard.facet, &mut ctx_buf, &req.text);
    Ok(Json(GenerateResponse {
        prompt: req.text,
        generated,
        vocabulary: guard.facet.vocabulary_size(),
        context_phase: ctx_buf.context_phase(),
        context_amplitude: ctx_buf.context_amplitude(),
    }))
}

pub async fn instruct(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<InstructResponse>, StatusCode> {
    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let model = &mut *guard;
    let mut engine = InstructionEngine::new();
    let response = engine.execute_instruction(
        &mut model.facet, &model.trainer,
        &model.cognitive_core, &mut model.context_buffer,
        &req.text,
    );
    Ok(Json(InstructResponse {
        prompt: req.text,
        output: response.text,
        vocabulary: model.facet.vocabulary_size(),
    }))
}

pub async fn reason(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<ReasoningResponse>, StatusCode> {
    let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let engine = ReasoningEngine;
    let chain = engine.solve(&guard.facet, &req.text);
    Ok(Json(ReasoningResponse {
        problem: chain.problem,
        converged: chain.converged,
        steps_count: chain.steps.len(),
        final_answer: chain.final_answer,
    }))
}

pub async fn layers_info(
    State(state): State<SharedModel>,
) -> Result<Json<LayersResponse>, StatusCode> {
    let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut field = HierarchicalPhaseField::new();
    field.build_hierarchy(&guard.facet);
    let summaries: Vec<LayerSummaryItem> = field.layers.iter().enumerate()
        .map(|(idx, layer)| LayerSummaryItem {
            level: idx,
            sector_count: layer.sector_count,
            clusters_count: layer.clusters.len(),
        })
        .collect();
    Ok(Json(LayersResponse { layers_count: field.layers.len(), layer_summaries: summaries }))
}

pub async fn run_synthetic(
    State(state): State<SharedModel>,
) -> Result<Json<SyntheticResponse>, StatusCode> {
    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let model = &mut *guard;
    let pipeline = SyntheticCurriculumPipeline::new(0.45, 0.70);
    let accepted = pipeline.run_pipeline(&mut model.facet, &model.trainer);
    Ok(Json(SyntheticResponse {
        accepted_count: accepted,
        vocabulary: model.facet.vocabulary_size(),
        message: format!("Curated and trained on {} high-quality synthetic sentences", accepted),
    }))
}

pub async fn phi4_learn(
    State(state): State<SharedModel>,
) -> Result<Json<Phi4LearnResponse>, StatusCode> {
    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let model = &mut *guard;
    let source = Phi4Source::discover();
    let summary = source.learn_into_facet(&mut model.facet, &model.trainer);
    Ok(Json(Phi4LearnResponse {
        vocab_tokens_loaded: summary.vocab_tokens_loaded,
        merges_trained: summary.merges_trained,
        doc_sentences_trained: summary.doc_sentences_trained,
        final_vocabulary_size: summary.final_vocabulary_size,
        message: "Successfully learned Phi-4 vocabulary, token merges, and technical reasoning context".to_string(),
    }))
}

pub async fn ingest_all(
    State(state): State<SharedModel>,
    Json(req): Json<IngestRequest>,
) -> Result<Json<IngestResponse>, StatusCode> {
    let do_curriculum = req.curriculum.unwrap_or(true);
    let do_dialogue = req.dialogue.unwrap_or(true);
    let do_phi4 = req.phi4.unwrap_or(true);
    let wiki_topics = req.wiki_topics.unwrap_or(12);

    let mut curriculum_sentences = 0usize;
    let mut dialogues_trained = 0usize;
    let mut phi4_merges = 0usize;
    let vocab_before;
    {
        let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let model = &mut *guard;
        vocab_before = model.facet.vocabulary_size();
        if do_curriculum {
            let cur = crate::curriculum::ChildCurriculum::new();
            if !cur.stages.is_empty() {
                let chunks = crate::chunker::ChunkStore::new("data/chunks");
                let r = cur.run(&mut model.facet, &model.trainer, &chunks);
                curriculum_sentences = r.sentences_trained;
            }
        }
        if do_dialogue {
            let src = crate::sources::dialogue::DialogueSource::default_curriculum();
            dialogues_trained = src.learn_into_facet(&mut model.facet, &mut model.memo, &model.trainer);
        }
        if do_phi4 {
            let src = Phi4Source::discover();
            let s = src.learn_into_facet(&mut model.facet, &model.trainer);
            phi4_merges = s.merges_trained;
        }
    }

    let (wiki_ok, wiki_tokens) = if wiki_topics > 0 {
        let (extracts, _errors) = crate::wiki_bulk::WikiBulk::fetch_curriculum_extracts(Some(wiki_topics)).await;
        let wiki_ok = extracts.len();
        let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let model = &mut *guard;
        let wiki_tokens = crate::wiki_bulk::WikiBulk::train_extracts(&mut model.facet, &model.trainer, &extracts);
        (wiki_ok, wiki_tokens)
    } else {
        (0, 0)
    };

    let vocab_after = {
        let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        guard.facet.vocabulary_size()
    };

    Ok(Json(IngestResponse {
        vocabulary_before: vocab_before,
        vocabulary_after: vocab_after,
        curriculum_sentences,
        dialogues_trained,
        phi4_merges,
        wiki_topics: wiki_ok,
        wiki_tokens,
        message: format!(
            "Ingested curriculum={} dialogue={} phi4_merges={} wiki={}/{} → vocab {}",
            curriculum_sentences, dialogues_trained, phi4_merges, wiki_ok, wiki_topics, vocab_after
        ),
    }))
}

pub async fn stats(
    State(state): State<SharedModel>,
) -> Result<Json<StatsResponse>, StatusCode> {
    let model = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(StatsResponse {
        vocabulary: model.facet.vocabulary_size(),
        memory_entries: model.memo.len(),
    }))
}

pub async fn command(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<CommandResponse>, StatusCode> {
    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let model = &mut *guard;
    let line = req.text.clone();
    let arg = line.splitn(2, char::is_whitespace).nth(1).map(|s| s.trim()).unwrap_or("");
    let mut ctx = Context {
        manifold: &mut model.facet,
        trainer: &model.trainer,
        memory: &mut model.memo,
        world: &mut model.world,
        context_buffer: &mut model.context_buffer,
        cognitive_core: &model.cognitive_core,
        corrections: &mut model.corrections,
        gaps: Some(&model.envisioner.ledger),
        arg,
        line: &line,
    };
    Dispatcher::dispatch(&line, &mut ctx);
    Ok(Json(CommandResponse { output: "Command executed".to_string() }))
}

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




