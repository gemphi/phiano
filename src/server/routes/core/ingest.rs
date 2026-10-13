#[allow(unused_imports)]
use super::*;

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
        let (extracts, _errors) = crate::wiki::WikiBulk::fetch_curriculum_extracts(Some(wiki_topics)).await;
        let wiki_ok = extracts.len();
        let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let model = &mut *guard;
        let wiki_tokens = crate::wiki::WikiBulk::train_extracts(&mut model.facet, &model.trainer, &extracts);
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
    let arg = line.split_once(char::is_whitespace).map(|(_, s)| s.trim()).unwrap_or("");
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
