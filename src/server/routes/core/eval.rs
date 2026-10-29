#[allow(unused_imports)]
use super::*;

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
    let mut ctx_buf = phient::generate::ContextWaveBuffer::new(4096);
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
