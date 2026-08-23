/// Wikipedia bulk ingestion — downloads and trains on multiple Wikipedia
/// articles in sequence for large-scale knowledge acquisition.

use crate::chunker::ChunkStore;
use crate::facet::Facet;
use crate::trainer::Trainer;

/// A curated set of topics for comprehensive knowledge ingestion.
/// Organized by domain for structured learning.
pub const CURRICULUM_TOPICS: &[&[&str]] = &[
    // Science
    &[
        "Physics", "Chemistry", "Biology", "Astronomy", "Geology",
        "Mathematics", "Statistics", "Calculus", "Algebra", "Geometry",
        "Quantum mechanics", "Relativity", "Thermodynamics", "Entropy",
        "Evolution", "Genetics", "DNA", "Cell (biology)", "Protein",
        "Photosynthesis", "Ecosystem", "Climate change", "Atom",
    ],
    // Technology
    &[
        "Computer science", "Programming language", "Algorithm",
        "Data structure", "Operating system", "Database",
        "Internet", "World Wide Web", "Cryptography", "Artificial intelligence",
        "Machine learning", "Deep learning", "Neural network",
        "Transformer (machine learning model)", "Attention mechanism",
        "Backpropagation", "Gradient descent", "Natural language processing",
        "Computer vision", "Reinforcement learning",
    ],
    // Philosophy
    &[
        "Philosophy", "Epistemology", "Metaphysics", "Ethics", "Logic",
        "Consciousness", "Intentionality", "Phenomenology",
        "Existentialism", "Pragmatism", "Rationalism", "Empiricism",
        "Philosophy of mind", "Philosophy of language", "Aesthetics",
    ],
    // History & Society
    &[
        "History", "Civilization", "Democracy", "Capitalism",
        "Socialism", "Revolution", "Colonialism", "World War I",
        "World War II", "Cold War", "Renaissance", "Enlightenment",
        "Industrial Revolution", "Information Age", "Globalization",
    ],
    // Arts & Literature
    &[
        "Literature", "Poetry", "Novel", "Fiction", "Mythology",
        "Music", "Painting", "Sculpture", "Architecture", "Theatre",
        "Film", "Photography", "Dance", "Opera",
    ],
];

/// Result of a bulk ingestion run.
#[derive(Debug, Clone, serde::Serialize)]
pub struct IngestionResult {
    pub topics_attempted: usize,
    pub topics_succeeded: usize,
    pub total_tokens_trained: usize,
    pub vocabulary_before: usize,
    pub vocabulary_after: usize,
    pub errors: Vec<String>,
}

/// Fetches a Wikipedia article extract via the REST API.
async fn fetch_wiki_extract(client: &reqwest::Client, topic: &str) -> Result<(String, String), String> {
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&prop=extracts&exintro=false&explaintext=true&titles={}&format=json&redirects=1",
        topic.replace(' ', "_")
    );

    let resp = client.get(&url).send().await
        .map_err(|e| format!("request failed: {}", e))?;

    if resp.status().as_u16() == 429 {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let resp2 = client.get(&url).send().await
            .map_err(|e| format!("retry failed: {}", e))?;
        if !resp2.status().is_success() {
            return Err(format!("status: {}", resp2.status()));
        }
        return parse_wiki_response(resp2.text().await.map_err(|e| e.to_string())?, topic);
    }

    if !resp.status().is_success() {
        return Err(format!("status: {}", resp.status()));
    }

    parse_wiki_response(resp.text().await.map_err(|e| e.to_string())?, topic)
}

fn parse_wiki_response(text: String, topic: &str) -> Result<(String, String), String> {
    let json: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| format!("json parse: {}", e))?;

    let pages = json.get("query")
        .and_then(|q| q.get("pages"))
        .and_then(|p| p.as_object())
        .ok_or("no pages in response")?;

    for (_, page) in pages {
        let title = page.get("title")
            .and_then(|t| t.as_str())
            .unwrap_or(topic);
        if let Some(extract) = page.get("extract").and_then(|e| e.as_str()) {
            if !extract.is_empty() {
                return Ok((title.to_string(), extract.to_string()));
            }
        }
    }

    Err("no extract found".to_string())
}

/// Runs bulk Wikipedia ingestion: downloads articles for all curriculum
/// topics and trains the facet on them.
pub async fn bulk_ingest(
    facet: &mut Facet,
    trainer: &Trainer,
    _chunk_store: &ChunkStore,
    max_topics: Option<usize>,
) -> IngestionResult {
    let vocab_before = facet.vocabulary_size();
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .user_agent("PhianoBot/0.1 (educational research)")
        .build()
        .expect("failed to build HTTP client");

    let mut all_topics: Vec<&str> = Vec::new();
    for group in CURRICULUM_TOPICS {
        for topic in *group {
            all_topics.push(topic);
        }
    }

    if let Some(max) = max_topics {
        all_topics.truncate(max);
    }

    let total = all_topics.len();
    let mut succeeded = 0usize;
    let mut total_tokens = 0usize;
    let mut errors = Vec::new();

    for (i, topic) in all_topics.iter().enumerate() {
        print!("  [ingest {}/{}] {}... ", i + 1, total, topic);

        match fetch_wiki_extract(&client, topic).await {
            Ok((title, extract)) => {
                let truncated = if extract.len() > 5000 {
                    &extract[..5000]
                } else {
                    &extract
                };
                let tokens = trainer.train_sentence(facet, truncated);
                total_tokens += tokens;
                succeeded += 1;
                println!("OK ({} tokens)", tokens);
            }
            Err(e) => {
                println!("FAIL: {}", e);
                errors.push(format!("{}: {}", topic, e));
            }
        }

        // Small delay to be respectful to Wikipedia API
        if (i + 1) % 10 == 0 {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }

    IngestionResult {
        topics_attempted: total,
        topics_succeeded: succeeded,
        total_tokens_trained: total_tokens,
        vocabulary_before: vocab_before,
        vocabulary_after: facet.vocabulary_size(),
        errors,
    }
}
