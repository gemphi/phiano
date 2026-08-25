/// Wikipedia learning and search route handlers.

use super::SharedModel;
use super::types::*;
use crate::eval::Evaluator;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;

/// Parses Wikipedia API responses into title and extract text.
pub struct WikiParser;

impl WikiParser {
    /// Parses a Wikipedia API response to extract the title and text extract.
    pub fn parse_extract(resp_text: &str, topic: &str) -> Result<(String, String), StatusCode> {
    let api_data: serde_json::Value = serde_json::from_str(resp_text)
        .map_err(|e| {
            eprintln!("[wiki/learn] JSON parse failed: {}", e);
            StatusCode::BAD_GATEWAY
        })?;

    let pages = api_data.get("query")
        .and_then(|q| q.get("pages"))
        .and_then(|p| p.as_object())
        .ok_or(StatusCode::NOT_FOUND)?;

    let page = pages.values().next().ok_or(StatusCode::NOT_FOUND)?;

    let title = page.get("title").and_then(|v| v.as_str()).unwrap_or(topic).to_string();
    let extract = page.get("extract").and_then(|v| v.as_str()).unwrap_or("").to_string();

        if extract.is_empty() { return Err(StatusCode::NOT_FOUND); }
        Ok((title, extract))
    }
}

pub async fn wiki_learn(
    State(state): State<SharedModel>,
    Json(req): Json<WikiRequest>,
) -> Result<Json<WikiLearnResponse>, StatusCode> {
    let topic = req.topic.trim().replace(' ', "_");
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .user_agent("PhianoBot/0.1 (educational research; contact@example.com)")
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&prop=extracts&exintro=true&explaintext=true&titles={}&format=json&redirects=1",
        topic
    );

    let resp = client.get(&url).send().await
        .map_err(|e| { eprintln!("[wiki/learn] request failed: {}", e); StatusCode::BAD_GATEWAY })?;

    let (title, extract) = if resp.status().as_u16() == 429 {
        eprintln!("[wiki/learn] rate limited, retrying after 2s...");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let resp2 = client.get(&url).send().await
            .map_err(|e| { eprintln!("[wiki/learn] retry failed: {}", e); StatusCode::BAD_GATEWAY })?;
        if !resp2.status().is_success() { return Err(StatusCode::BAD_GATEWAY); }
        WikiParser::parse_extract(&resp2.text().await.map_err(|_| StatusCode::BAD_GATEWAY)?, &topic)?
    } else {
        if !resp.status().is_success() { return Err(StatusCode::BAD_GATEWAY); }
        WikiParser::parse_extract(&resp.text().await.map_err(|_| StatusCode::BAD_GATEWAY)?, &topic)?
    };

    let extract_truncated = if extract.len() > 2000 { extract[..2000].to_string() } else { extract.clone() };

    let (tokens, vocab_before, vocab_after, eval_result) = {
        let mut guard = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let model = &mut *guard;
        let vocab_before = model.facet.vocabulary_size();
        let epochs = req.epochs.unwrap_or(3);
        let tokens = if epochs > 1 {
            model.trainer.train_multi_epoch(&mut model.facet, &extract_truncated, epochs, 1).tokens_learned
        } else {
            model.trainer.train_online(&mut model.facet, &extract_truncated)
        };
        let vocab_after = model.facet.vocabulary_size();
        let eval = Evaluator::new().eval(&model.facet, &extract_truncated);
        (tokens, vocab_before, vocab_after, eval)
    };

    let display_extract = if extract_truncated.len() > 500 {
        format!("{}...", &extract_truncated[..500])
    } else { extract_truncated };

    Ok(Json(WikiLearnResponse {
        topic: req.topic.clone(),
        title: title.clone(),
        extract: display_extract,
        tokens_trained: tokens,
        vocabulary_before: vocab_before,
        vocabulary_after: vocab_after,
        coherence: eval_result.coherence,
        novelty: eval_result.novelty,
        resonance: eval_result.resonance,
        verdict: format!("{}", eval_result.verdict),
        message: format!("Learned from Wikipedia article \"{}\" - {} tokens trained", title, tokens),
    }))
}

pub async fn wiki_search(
    State(_state): State<SharedModel>,
    Json(req): Json<WikiRequest>,
) -> Result<Json<WikiSearchResponse>, StatusCode> {
    let query = req.topic.trim();
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("PhianoBot/0.1 (educational research; contact@example.com)")
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let url = format!("https://en.wikipedia.org/w/rest.php/v1/search/page?q={}&limit=5", query);
    let resp = client.get(&url).send().await
        .map_err(|e| { eprintln!("[wiki/search] request failed: {}", e); StatusCode::BAD_GATEWAY })?;

    if !resp.status().is_success() { return Err(StatusCode::BAD_GATEWAY); }

    let resp_text = resp.text().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    let search_data: serde_json::Value = serde_json::from_str(&resp_text)
        .map_err(|e| { eprintln!("[wiki/search] JSON parse failed: {}", e); StatusCode::BAD_GATEWAY })?;

    let pages = search_data.get("pages").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let results = pages.iter().filter_map(|entry| {
        let title = entry.get("title")?.as_str()?.to_string();
        let description = entry.get("excerpt").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let key = entry.get("key")?.as_str()?.to_string();
        Some(WikiSearchResult { title, description, url: format!("https://en.wikipedia.org/wiki/{}", key) })
    }).collect();

    Ok(Json(WikiSearchResponse { query: query.to_string(), results }))
}
