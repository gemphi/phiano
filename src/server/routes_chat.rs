/// Chat endpoint with real-time learning, grounded knowledge retrieval, and conversational synthesis.

use super::SharedModel;
use super::types::*;
use super::routes_wiki::WikiParser;
use crate::config::{
    CHUNK_STORE_DIR,
    DEFINITION_CHAIN_DEPTH, WIKI_SNIPPET_MAX_CHARS,
};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use super::chat_intent::ChatIntent;

pub async fn chat(
    State(state): State<SharedModel>,
    Json(req): Json<TextRequest>,
) -> Result<Json<ChatResponse>, StatusCode> {
    let prompt = req.text.trim();
    match prompt.is_empty() {
        true => return Ok(Json(ChatResponse {
            response: "Hello! How can I help you today?".to_string(),
            speech_act: "expressive".to_string(),
            direction_of_fit: "none".to_string(),
            words_learned: 0,
            definitions_learned: 0,
            wiki_learned: None,
            vocabulary: 0,
            coherence: 1.0,
        })),
        false => {}
    }

    let tokens = crate::tokenizer::Tokenizer::tokenize(prompt);

    // 1. Online Learning of the prompt into the manifold
    let (words_learned, vocab_size) = {
        let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let model = &mut *guard;
        let wl = model.trainer.train_online(&mut model.facet, prompt);
        (wl, model.facet.vocabulary_size())
    };

    // 2. Identify unknown words and learn their definition chains
    let unknown: Vec<String> = {
        let guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        tokens.iter().filter(|t| !guard.facet.lexicon.contains_key(*t)).cloned().collect()
    };
    let defs_count = learn_unknown_definitions(&state, &unknown)?;

    // 3. Wikipedia lookup for explanatory topics if needed
    let (wiki_learned, wiki_content) = try_wikipedia_learning(prompt, &tokens, &state).await;

    // 4. Generate structured conversational response via ChatIntent
    let (response, speech_act, dof, coherence) = {
        let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let model = &mut *guard;
        
        let cog_result = model.cognitive_core.process(&model.facet, &mut model.context_buffer, prompt);
        
        let intent = ChatIntent::classify(prompt, &tokens, &cog_result);
        if let ChatIntent::SelfCorrection { statement, correction } = &intent {
            model.trainer.correct_mistake(&mut model.facet, statement, correction);
        }
        let fluent_response = intent.generate_response(model, &cog_result, wiki_content.as_deref());

        // Record response into context wave buffer and 16-layer memory hierarchy
        model.context_buffer.push_turn(&model.facet, &fluent_response);
        let resp_tokens = crate::tokenizer::Tokenizer::tokenize(&fluent_response);
        let resp_wave = crate::wave::Wave::sentence(&model.facet, &resp_tokens);
        model.memo.record((resp_wave.re, resp_wave.im), &fluent_response);

        (fluent_response, cog_result.speech_act, cog_result.direction_of_fit, cog_result.coherence)
    };

    Ok(Json(ChatResponse {
        response,
        speech_act,
        direction_of_fit: dof,
        words_learned,
        definitions_learned: defs_count,
        wiki_learned,
        vocabulary: vocab_size,
        coherence,
    }))
}

fn learn_unknown_definitions(
    state: &super::SharedModel,
    unknown: &[String],
) -> Result<usize, StatusCode> {
    match unknown.is_empty() {
        true => return Ok(0),
        false => {}
    }
    let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let chunk_store = crate::chunker::ChunkStore::new(CHUNK_STORE_DIR);
    let model = &mut *guard;
    let mut count = 0;
    for word in unknown {
        let learned = model.trainer.learn_definition_chain(
            &mut model.facet, &chunk_store, word, DEFINITION_CHAIN_DEPTH,
        );
        count += learned.len();
    }
    Ok(count)
}

async fn try_wikipedia_learning(
    prompt: &str,
    tokens: &[String],
    state: &super::SharedModel,
) -> (Option<String>, Option<String>) {
    let p_lower = prompt.to_lowercase();
    match p_lower.contains("what is") || p_lower.contains("explain") || p_lower.contains("who is") || p_lower.contains("tell me about") {
        false => return (None, None),
        true => {}
    }

    let topic = ChatIntent::extract_topic_term(tokens, prompt);
    let topic_clean = topic.replace(' ', "_");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .user_agent("PhianoBot/0.1 (educational research)")
        .build();

    let client = match client { Ok(c) => c, Err(_) => return (None, None) };

    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&prop=extracts&exintro=true&explaintext=true&titles={}&format=json&redirects=1",
        topic_clean
    );

    let resp = match client.get(&url).send().await { Ok(r) => r, Err(_) => return (None, None) };
    match resp.status().is_success() {
        true => {}
        false => return (None, None),
    }

    let text = match resp.text().await { Ok(t) => t, Err(_) => return (None, None) };
    let (title, extract) = match WikiParser::parse_extract(&text, &topic) {
        Ok(r) => r, Err(_) => return (None, None),
    };

    let truncated = match extract.len() > WIKI_SNIPPET_MAX_CHARS {
        true => extract[..WIKI_SNIPPET_MAX_CHARS].to_string(),
        false => extract,
    };
    let wiki_str = format!("{} ({} chars)", title, truncated.len());

    match state.lock() {
        Ok(mut guard) => {
            let model = &mut *guard;
            model.trainer.train_online(&mut model.facet, &truncated);
        }
        Err(_) => {}
    }

    (Some(wiki_str), Some(truncated))
}

