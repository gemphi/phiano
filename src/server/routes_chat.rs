/// Chat endpoint with real-time learning and definition chain learning.

use super::SharedModel;
use super::types::*;
use super::routes_wiki::parse_wiki_extract;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;

pub async fn chat(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<ChatResponse>, StatusCode> {
    let prompt = &req.text;
    let tokens = crate::tokenizer::Tokenizer::tokenize(prompt);

    let unknown: Vec<String> = {
        let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        tokens.iter().filter(|t| !guard.facet.lexicon.contains_key(*t)).cloned().collect()
    };

    let defs_count = learn_unknown_definitions(&state, &unknown)?;

    let (wiki_learned, _) = try_wikipedia_learning(prompt, &tokens, &state).await;

    let (response, speech_act, dof, coherence, vocab, words_learned) = {
        let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let mut ctx_buf = crate::generate::ContextWaveBuffer::new(4096);
        let result = guard.cognitive_core.process(&guard.facet, &mut ctx_buf, prompt);
        let wl = tokens.iter().filter(|t| guard.facet.lexicon.contains_key(*t)).count();
        (result.synthesized_output, result.speech_act, result.direction_of_fit,
         result.coherence, guard.facet.vocabulary_size(), wl)
    };

    Ok(Json(ChatResponse {
        response, speech_act, direction_of_fit: dof, words_learned,
        definitions_learned: defs_count, wiki_learned, vocabulary: vocab, coherence,
    }))
}

fn learn_unknown_definitions(
    state: &super::SharedModel,
    unknown: &[String],
) -> Result<usize, StatusCode> {
    if unknown.is_empty() { return Ok(0); }
    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let chunk_store = crate::chunker::ChunkStore::new("data/chunks");
    let model = &mut *guard;
    let mut count = 0;
    for word in unknown {
        let learned = model.trainer.learn_definition_chain(
            &mut model.facet, &chunk_store, word, 3,
        );
        count += learned.len();
    }
    Ok(count)
}

async fn try_wikipedia_learning(
    prompt: &str,
    tokens: &[String],
    state: &super::SharedModel,
) -> (Option<String>, bool) {
    let unknown_exists = tokens.iter().any(|t| {
        let guard = state.lock().ok();
        guard.map(|g| !g.facet.lexicon.contains_key(t)).unwrap_or(false)
    });

    if !unknown_exists && !prompt.contains("what is") && !prompt.contains("explain") {
        return (None, false);
    }

    let main_topic = tokens.iter().filter(|t| t.len() > 3).next().cloned()
        .unwrap_or_else(|| prompt.to_string());
    let topic_clean = main_topic.replace(' ', "_");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("PhianoBot/0.1 (educational research)")
        .build();

    let client = match client { Ok(c) => c, Err(_) => return (None, false) };

    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&prop=extracts&exintro=true&explaintext=true&titles={}&format=json&redirects=1",
        topic_clean
    );

    let resp = match client.get(&url).send().await { Ok(r) => r, Err(_) => return (None, false) };
    if !resp.status().is_success() { return (None, false); }

    let text = match resp.text().await { Ok(t) => t, Err(_) => return (None, false) };
    let (title, extract) = match parse_wiki_extract(&text, &main_topic) {
        Ok(r) => r, Err(_) => return (None, false),
    };

    let truncated = if extract.len() > 1500 { extract[..1500].to_string() } else { extract };
    let wiki_str = format!("{} ({} chars)", title, truncated.len());

    if let Ok(mut guard) = state.lock() {
        let model = &mut *guard;
        model.trainer.train_online(&mut model.facet, &truncated);
    }

    (Some(wiki_str), true)
}
