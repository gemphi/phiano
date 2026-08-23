use crate::sources::local::LocalSource;
use crate::sources::DictionarySource;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;

/// Free Dictionary API & Multi-Source Auto-Feed Engine.
///
/// Fetches rich word definitions on-demand from:
/// 1. Free Dictionary API (api.dictionaryapi.dev)
/// 2. Morphological Lemmatization (past tense -ed, -ing, plurals -s, adverbs -ly)
/// 3. Wikipedia REST API Summary for proper nouns and encyclopedia terms
/// 4. Auto-trains the facet in real-time.
pub struct ApiSource {
    pub cache_path: String,
    pub timeout_secs: u64,
}

impl ApiSource {
    pub fn new(cache_path: &str) -> Self {
        Self {
            cache_path: cache_path.to_string(),
            timeout_secs: 10,
        }
    }

    /// Build morphological candidate stems for inflected words.
    pub fn lemmatize_candidates(word: &str) -> Vec<(String, &'static str)> {
        let mut candidates = Vec::new();
        let w = word.to_lowercase();

        // 1. -ed past tense / participle (e.g. warmed -> warm, created -> create)
        if w.ends_with("ed") && w.len() > 3 {
            let base1 = &w[..w.len() - 2]; // warmed -> warm
            candidates.push((base1.to_string(), "Past tense & past participle of"));
            let base2 = &w[..w.len() - 1]; // loved -> love, created -> create
            candidates.push((base2.to_string(), "Past tense of"));
            if base1.ends_with('i') && base1.len() > 2 { // carried -> carry
                let base3 = format!("{}y", &base1[..base1.len() - 1]);
                candidates.push((base3, "Past tense of"));
            }
        }

        // 2. -ing continuous / participle (e.g. warming -> warm, making -> make)
        if w.ends_with("ing") && w.len() > 4 {
            let base1 = &w[..w.len() - 3];
            candidates.push((base1.to_string(), "Present participle & gerund of"));
            let base2 = format!("{}e", base1);
            candidates.push((base2, "Present participle of"));
        }

        // 3. -s / -es / -ies plurals (e.g. coins -> coin, monies -> money, watches -> watch)
        if w.ends_with("ies") && w.len() > 4 {
            let base = format!("{}y", &w[..w.len() - 3]);
            candidates.push((base, "Plural form of"));
        } else if w.ends_with("es") && w.len() > 3 {
            let base = &w[..w.len() - 2];
            candidates.push((base.to_string(), "Plural / 3rd person singular of"));
        } else if w.ends_with('s') && w.len() > 2 && !w.ends_with("ss") {
            let base = &w[..w.len() - 1];
            candidates.push((base.to_string(), "Plural form of"));
        }

        // 4. -ly adverbs (e.g. warmly -> warm)
        if w.ends_with("ly") && w.len() > 3 {
            let base = &w[..w.len() - 2];
            candidates.push((base.to_string(), "Adverbial form of"));
        }

        candidates
    }

    /// Fetch rich formatted definition for a single word from the API.
    pub fn fetch_word_rich(&self, word: &str) -> Option<String> {
        let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(self.timeout_secs))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) Phiano/1.0")
            .build()
            .ok()?;

        let response = client.get(&url).send().ok()?;
        if !response.status().is_success() {
            return None;
        }

        let api_entries: Vec<ApiEntry> = response.json().ok()?;
        if api_entries.is_empty() {
            return None;
        }

        let entry = &api_entries[0];
        let mut out = String::new();
        out.push_str(&format!("{}\n", entry.word));
        if let Some(phonetic) = &entry.phonetic {
            out.push_str(&format!("({})\n", phonetic));
        }

        let mut def_count = 1;
        for meaning in &entry.meanings {
            let pos_abbrev = match meaning.part_of_speech.as_str() {
                "noun" => "n.",
                "verb" => "v.",
                "transitive verb" => "tr.v.",
                "intransitive verb" => "intr.v.",
                "adjective" => "adj.",
                "adverb" => "adv.",
                other => other,
            };
            out.push_str(&format!("{}. ", pos_abbrev));

            for (idx, def) in meaning.definitions.iter().enumerate() {
                let sub_num = if meaning.definitions.len() > 1 {
                    format!("{}.{}. ", def_count, (b'a' + (idx as u8)) as char)
                } else {
                    format!("{}. ", def_count)
                };
                out.push_str(&format!("{}{}", sub_num, def.definition));
                if let Some(ex) = &def.example {
                    out.push_str(&format!(" Example: \"{}\"", ex));
                }
                out.push('\n');
            }
            def_count += 1;
        }

        Some(out)
    }

    /// Fetch summary extract from Wikipedia for encyclopedia terms or proper nouns.
    pub fn fetch_wikipedia_summary(&self, word: &str) -> Option<String> {
        let url = format!("https://en.wikipedia.org/api/rest_v1/page/summary/{}", word);

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(self.timeout_secs))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) Phiano/1.0")
            .build()
            .ok()?;

        let response = client.get(&url).send().ok()?;
        if !response.status().is_success() {
            return None;
        }

        #[derive(Deserialize)]
        struct WikiSummary {
            title: String,
            extract: Option<String>,
            description: Option<String>,
        }

        let summary: WikiSummary = response.json().ok()?;
        let extract = summary.extract?;
        if extract.trim().is_empty() {
            return None;
        }

        let mut out = format!("{}\n(Encyclopedia & Knowledge Base)\n", summary.title);
        if let Some(desc) = summary.description {
            out.push_str(&format!("n. [{}]: ", desc));
        }
        out.push_str(&extract);
        Some(out)
    }

    /// Fetch list of plain definition strings for training.
    pub fn fetch_word(&self, word: &str) -> Vec<String> {
        let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

        let client = match reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(self.timeout_secs))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) Phiano/1.0")
            .build()
        {
            Ok(c) => c,
            Err(_) => return vec![],
        };

        let response = match client.get(&url).send() {
            Ok(r) => r,
            Err(_) => return vec![],
        };

        if !response.status().is_success() {
            return vec![];
        }

        let api_entries: Vec<ApiEntry> = match response.json() {
            Ok(e) => e,
            Err(_) => return vec![],
        };

        let mut definitions = Vec::new();
        for entry in api_entries {
            for meaning in entry.meanings {
                for def in meaning.definitions {
                    definitions.push(def.definition);
                }
            }
        }

        if !definitions.is_empty() {
            self.cache_word(word, &definitions);
        }

        definitions
    }

    /// Append a word's definitions to the local cache file.
    fn cache_word(&self, word: &str, definitions: &[String]) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.cache_path)
        {
            for def in definitions {
                let _ = writeln!(file, "{}: {}", word, def);
            }
        }
    }
}

#[derive(Deserialize)]
struct ApiEntry {
    word: String,
    phonetic: Option<String>,
    meanings: Vec<ApiMeaning>,
}

#[derive(Deserialize)]
struct ApiMeaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    definitions: Vec<ApiDefinition>,
}

#[derive(Deserialize)]
struct ApiDefinition {
    definition: String,
    example: Option<String>,
}

impl DictionarySource for ApiSource {
    fn fetch_all(&self) -> Vec<(String, String)> {
        let local = LocalSource::new(&self.cache_path);
        local.fetch_all()
    }

    fn fetch_definitions(&self, word: &str) -> Vec<String> {
        let local = LocalSource::new(&self.cache_path);
        let cached = local.fetch_definitions(word);
        if !cached.is_empty() {
            return cached;
        }

        self.fetch_word(word)
    }
}
