use crate::config::{ALPHA, FORMAT_VERSION};
use crate::facet::{Facet, Vocab, WordId};
use crate::phasor::SpectralPhasor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, ErrorKind, Result};

/// Header metadata prepended to binary .chroma files.
///
/// Written and read on its own, ahead of the body, so the version can be
/// inspected before deciding how to parse the rest.
#[derive(Serialize, Deserialize, Debug)]
pub struct ChromaHeader {
    pub version: u32,
    pub vocabulary_size: usize,
    pub fine_structure_alpha: f64,
}

/// On-disk phasor.
///
/// `phase` is omitted: it is by construction the angle of channel 0, so storing
/// it duplicates eight bytes per word for a value that is recovered exactly.
/// `amplitude` narrows to f32 — its useful range is [0.3, 2.0] and it is
/// compared, never accumulated, so 24 bits of mantissa is far more than the
/// quantity carries.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct DiskPhasor {
    packed: [u64; 8],
    amplitude: f32,
    band_n: u32,
    count: u32,
}

impl DiskPhasor {
    fn from(p: &SpectralPhasor) -> Self {
        Self {
            packed: p.packed(),
            amplitude: p.amplitude as f32,
            band_n: p.band_n,
            count: p.count,
        }
    }

    fn into_phasor(self) -> SpectralPhasor {
        SpectralPhasor::from_packed(self.packed, self.amplitude as f64, self.band_n, self.count)
    }
}

// ── v3: interned ───────────────────────────────────────────────────────────

/// Borrowed body for writing. Serializing by reference avoids cloning the whole
/// model into a second copy before every save.
#[derive(Serialize)]
struct BodyV3Ref<'a> {
    words: &'a [String],
    /// `(word id, phasor)` — the word itself lives once, in `words`.
    lexicon: Vec<(WordId, DiskPhasor)>,
    bigrams: Vec<(WordId, &'a Vec<(WordId, u32)>)>,
    trigrams: Vec<((WordId, WordId), &'a Vec<(WordId, u32)>)>,
    phase_lags: Vec<((WordId, WordId), f32)>,
    grounded_version: u32,
}

#[derive(Deserialize)]
struct BodyV3 {
    words: Vec<String>,
    lexicon: Vec<(WordId, DiskPhasor)>,
    bigrams: Vec<(WordId, Vec<(WordId, u32)>)>,
    trigrams: Vec<((WordId, WordId), Vec<(WordId, u32)>)>,
    phase_lags: Vec<((WordId, WordId), f32)>,
    grounded_version: u32,
}

// ── v2: string-keyed n-grams ───────────────────────────────────────────────

#[derive(Deserialize)]
struct BodyV2 {
    lexicon: HashMap<String, SpectralPhasor>,
    #[serde(default)]
    bigrams: HashMap<String, HashMap<String, u32>>,
    #[serde(default)]
    trigrams: HashMap<String, HashMap<String, u32>>,
    #[serde(default)]
    phase_lags: HashMap<String, HashMap<String, f64>>,
    #[serde(default)]
    grounded_version: u32,
}

// ── v1: lexicon only ───────────────────────────────────────────────────────

#[derive(Deserialize)]
struct BodyV1 {
    lexicon: HashMap<String, SpectralPhasor>,
}

/// Storage - facade for persisting and loading facets to/from disk.
pub struct Storage;

impl Storage {
    /// Saves a facet to a binary .chroma file, atomically.
    ///
    /// `rename` is atomic on every mainstream filesystem, so an interrupted save
    /// can no longer leave a truncated model that fails to load and silently
    /// starts the next session from an empty lexicon.
    pub fn save(facet: &Facet, path: &str) -> Result<()> {
        let tmp = format!("{}.tmp", path);

        // Every lexicon word needs an id. Words that only ever appeared in the
        // lexicon (never in an n-gram) have none yet, so they are appended here
        // rather than requiring a mutable facet to save.
        let mut words: Vec<String> = facet.vocab.words().to_vec();
        let mut extra: HashMap<&str, WordId> = HashMap::new();
        for w in facet.lexicon.keys() {
            if facet.vocab.id(w).is_none() {
                extra.insert(w.as_str(), words.len() as WordId);
                words.push(w.clone());
            }
        }
        let id_of = |w: &str| -> WordId {
            facet.vocab.id(w).unwrap_or_else(|| extra[w])
        };

        let body = BodyV3Ref {
            words: &words,
            lexicon: facet.lexicon.iter().map(|(w, p)| (id_of(w), DiskPhasor::from(p))).collect(),
            bigrams: facet.bigrams.iter().map(|(k, v)| (*k, v)).collect(),
            trigrams: facet.trigrams.iter().map(|(k, v)| (*k, v)).collect(),
            phase_lags: facet.phase_lags.iter().map(|(k, v)| (*k, *v)).collect(),
            grounded_version: facet.grounded_version,
        };

        {
            let file = File::create(&tmp)?;
            let mut writer = BufWriter::new(file);
            let header = ChromaHeader {
                version: FORMAT_VERSION,
                vocabulary_size: facet.vocabulary_size(),
                fine_structure_alpha: ALPHA,
            };
            bincode::serialize_into(&mut writer, &header)
                .map_err(|e| Error::new(ErrorKind::Other, e))?;
            bincode::serialize_into(&mut writer, &body)
                .map_err(|e| Error::new(ErrorKind::Other, e))?;
        }

        std::fs::rename(&tmp, path)
    }

    /// Loads a facet, migrating older formats forward.
    pub fn load(path: &str) -> Result<Facet> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let header: ChromaHeader = bincode::deserialize_from(&mut reader)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        if header.version > FORMAT_VERSION {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "{} is format v{}, this build reads v{}",
                    path, header.version, FORMAT_VERSION
                ),
            ));
        }
        if (header.fine_structure_alpha - ALPHA).abs() > 1e-12 {
            eprintln!(
                "  [warn] {} was trained with alpha={:.9}, this build uses {:.9}",
                path, header.fine_structure_alpha, ALPHA
            );
        }

        match header.version {
            3 => {
                let b: BodyV3 = bincode::deserialize_from(&mut reader)
                    .map_err(|e| Error::new(ErrorKind::Other, e))?;
                let vocab = Vocab::from_words(b.words);
                let lexicon = b
                    .lexicon
                    .into_iter()
                    .filter_map(|(id, p)| vocab.word(id).map(|w| (w.to_string(), p.into_phasor())))
                    .collect();
                Ok(Facet {
                    lexicon,
                    vocab,
                    bigrams: b.bigrams.into_iter().collect(),
                    trigrams: b.trigrams.into_iter().collect(),
                    phase_lags: b.phase_lags.into_iter().collect(),
                    grounded_version: b.grounded_version,
                    sample_pool: Vec::new(),
                })
            }
            2 => {
                let b: BodyV2 = bincode::deserialize_from(&mut reader)
                    .map_err(|e| Error::new(ErrorKind::Other, e))?;
                eprintln!("  [migrate] converting v2 string-keyed n-grams to interned ids");
                Ok(Self::from_string_keyed(
                    b.lexicon,
                    b.bigrams,
                    b.trigrams,
                    b.phase_lags,
                    b.grounded_version,
                ))
            }
            _ => {
                let b: BodyV1 = bincode::deserialize_from(&mut reader)
                    .map_err(|e| Error::new(ErrorKind::Other, e))?;
                Ok(Self::from_string_keyed(
                    b.lexicon,
                    HashMap::new(),
                    HashMap::new(),
                    HashMap::new(),
                    0,
                ))
            }
        }
    }

    /// Rebuilds an interned facet from the old string-keyed tables.
    fn from_string_keyed(
        lexicon: HashMap<String, SpectralPhasor>,
        bigrams: HashMap<String, HashMap<String, u32>>,
        trigrams: HashMap<String, HashMap<String, u32>>,
        phase_lags: HashMap<String, HashMap<String, f64>>,
        grounded_version: u32,
    ) -> Facet {
        let mut facet = Facet::new();
        facet.lexicon = lexicon;
        facet.grounded_version = grounded_version;

        for (a, followers) in bigrams {
            let ai = facet.vocab.intern(&a);
            let mut list: Vec<(WordId, u32)> = followers
                .into_iter()
                .map(|(b, c)| (facet.vocab.intern(&b), c))
                .collect();
            list.sort_unstable_by_key(|(k, _)| *k);
            facet.bigrams.insert(ai, list);
        }

        for (key, followers) in trigrams {
            // v2 trigram keys were the two context words joined by a space.
            let mut parts = key.splitn(2, ' ');
            let (a, b) = match (parts.next(), parts.next()) {
                (Some(a), Some(b)) => (a.to_string(), b.to_string()),
                _ => continue,
            };
            let k = (facet.vocab.intern(&a), facet.vocab.intern(&b));
            let mut list: Vec<(WordId, u32)> = followers
                .into_iter()
                .map(|(c, n)| (facet.vocab.intern(&c), n))
                .collect();
            list.sort_unstable_by_key(|(k, _)| *k);
            facet.trigrams.insert(k, list);
        }

        for (a, targets) in phase_lags {
            let ai = facet.vocab.intern(&a);
            for (b, v) in targets {
                let bi = facet.vocab.intern(&b);
                facet.phase_lags.insert((ai, bi), v as f32);
            }
        }

        facet
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trainer::Trainer;

    #[test]
    fn test_roundtrip_preserves_ngrams_and_lexicon() {
        let mut facet = Facet::new();
        let t = Trainer::new(0.05);
        for s in ["the cat sat on the mat", "the dog ran in the park"] {
            t.train_sentence(&mut facet, s);
        }

        let path = std::env::temp_dir().join("phiano_storage_roundtrip.chroma");
        let path = path.to_str().unwrap();
        Storage::save(&facet, path).unwrap();
        let back = Storage::load(path).unwrap();

        assert_eq!(back.vocabulary_size(), facet.vocabulary_size());
        assert_eq!(back.ngram_entries(), facet.ngram_entries());
        assert!((back.bigram_probability("the", "cat") - facet.bigram_probability("the", "cat")).abs() < 1e-12);
        for (w, p) in &facet.lexicon {
            let q = back.lexicon.get(w).expect("word survived the roundtrip");
            assert!((p.phase - q.phase).abs() < 1e-12);
            assert!(p.resonance(q) > 0.999);
        }
    }

    #[test]
    fn test_words_outside_the_ngram_tables_survive() {
        // A word added to the lexicon but never seen in a bigram has no id yet.
        let mut facet = Facet::new();
        facet.get_or_init("orphan");
        let path = std::env::temp_dir().join("phiano_storage_orphan.chroma");
        let path = path.to_str().unwrap();
        Storage::save(&facet, path).unwrap();
        let back = Storage::load(path).unwrap();
        assert!(back.contains_word("orphan"));
    }
}
