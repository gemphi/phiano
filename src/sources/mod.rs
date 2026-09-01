pub mod api;
pub mod dialogue;
pub mod json;
pub mod local;
pub mod phi4;
pub mod wiktionary;

/// DictionarySource - a source of word definitions for bootstrapping the facet.
///
/// Implementations include local files, JSON dictionaries, API sources,
/// Wiktionary dumps, and Phi-4 references.
pub trait DictionarySource {
    /// Returns all (word, definition) pairs from this source.
    fn fetch_all(&self) -> Vec<(String, String)>;

    /// Returns all definitions for a single word from this source.
    fn fetch_definitions(&self, word: &str) -> Vec<String>;
}

/// Strips dictionary apparatus from a definition before it reaches the trainer.
///
/// Webster's entries carry part-of-speech markers, etymology brackets, sense
/// numbers and citation attributions. Trained as if they were content, those
/// tokens acquire positions in the manifold and then have to be blocked at
/// generation time by a hardcoded `boilerplate` list — which suppresses the
/// symptom while leaving the words wrongly *placed*. Cleaning at ingestion
/// fixes the cause.
pub fn clean_definition(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut depth_sq = 0i32;
    let mut depth_par = 0i32;

    for ch in raw.chars() {
        match ch {
            '[' => depth_sq += 1,
            ']' => depth_sq = (depth_sq - 1).max(0),
            '(' => depth_par += 1,
            ')' => depth_par = (depth_par - 1).max(0),
            _ if depth_sq == 0 && depth_par == 0 => out.push(ch),
            _ => {}
        }
    }

    // Drop leading sense numbers ("1.", "2.") and standalone apparatus tokens.
    let cleaned: Vec<String> = out
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
        .filter(|w| !w.is_empty())
        .filter(|w| !w.chars().all(|c| c.is_ascii_digit()))
        .filter(|w| !is_apparatus(w))
        .collect();

    cleaned.join(" ")
}

/// Senses kept from an entry.
///
/// Webster's stacks every historical sense of a word into one entry — *cat* runs
/// from the animal through a type of sailing vessel to a constellation. Later
/// senses are real but rare, and composing all of them places the word at the
/// centre of mass of meanings nobody uses. Three keeps the polysemy that matters
/// and drops the tail.
const MAX_SENSES: usize = 3;

/// The most senses a word is allowed to have.
///
/// The architecture is 64 phase channels, and the claim this bound expresses is
/// that every level of language has a *bounded* inventory of forms — a word has
/// so many uses, a sentence so many shapes — so each level's identity is a type
/// index rather than an open set. That is CLU's position applied to language:
/// the type is finite and known, and instances are values of it.
///
/// Whether 64 is the right bound is an empirical question this does not settle.
/// What it settles is that the number is *finite and declared*, which is what
/// lets a sense be an index instead of a string.
pub const MAX_WORD_SENSES: usize = 64;

/// The definitional core of a dictionary entry.
///
/// [`clean_definition`] removes *apparatus* — brackets, parentheses, part-of-
/// speech markers. This removes *illustration*, which is a much larger share of
/// a Webster's entry and the reason two imported mechanisms failed:
///
/// ```text
/// cat: 1. (Zool.) An animal of various species of the genera Felis and Lynx.
///      The domestic cat is Felis domestica. The European wild cat ...
///      Note: The domestic cat includes many varieties ... as, the Angora cat;
///      the Maltese cat ...  2. (Naut.) A strong vessel with a narrow stern ...
/// ```
///
/// Only the first sentence of sense 1 defines *cat*. Everything after it is
/// elaboration, editorial note, usage example and cross-reference — words that
/// are *about* the entry rather than part of it. Kept, they inflate every
/// definer set, which is why the definition graph came out at 47.5∶1
/// weak∶strong against dict2vec's ~9∶1, and the grounding kernel at 49.6% of
/// entries against the literature's ~10%. Both numbers are measurable in
/// seconds (`cargo run --bin defstats`), and both are acceptance criteria.
///
/// The rule is structural, not a word list: take the first sentence of each of
/// the first [`MAX_SENSES`] senses, cutting each at the first illustrative
/// marker. A hardcoded list of poets — which is what the apparatus list had
/// grown — suppresses the symptom for the poets someone remembered.
pub fn definition_core(raw: &str) -> String {
    let mut kept: Vec<String> = Vec::new();

    for sense in split_senses(raw).into_iter().take(MAX_SENSES) {
        let body = strip_sense_head(&sense);
        let trimmed = cut_at_illustration(body);
        // The gloss is the first sentence. A quotation, when one follows, is a
        // *later* sentence — so taking the first sentence excludes attributions
        // without having to recognise the author.
        let gloss = first_sentence(trimmed);
        let cleaned = clean_definition(gloss);
        // Two content words is the floor for a usable gloss; below that the
        // sense was apparatus all the way down.
        if cleaned.split_whitespace().count() >= 2 {
            kept.push(cleaned);
        }
    }

    // A word whose every sense was too short still has to reach the manifold
    // somehow, so fall back to the old behaviour rather than dropping it.
    match kept.is_empty() {
        true => clean_definition(raw),
        false => kept.join(" "),
    }
}

/// The gloss of each numbered sense, separately.
///
/// [`definition_core`] concatenates the first [`MAX_SENSES`] senses into one
/// string, which is how every measurement in this project came to be taken on a
/// representation that cannot hold polysemy: *cat* has eight numbered senses in
/// Webster's — the animal, a strong sailing vessel, a double tripod, the
/// constellation — and they were merged into a single gloss, composed to a
/// single centroid, and stored at a single point.
///
/// A word is not one thing. This returns the senses so each can have its own
/// phasor, which is the bounded per-word type inventory the architecture was
/// always described as having and never had.
///
/// Capped at [`MAX_WORD_SENSES`]: the claim is that a word's uses are *bounded*,
/// not that they are unlimited, and a cap is what makes the sense index a type
/// rather than an ever-growing list.
pub fn definition_senses(raw: &str) -> Vec<String> {
    split_senses(raw)
        .into_iter()
        .take(MAX_WORD_SENSES)
        .filter_map(|sense| {
            let body = strip_sense_head(&sense);
            let gloss = first_sentence(cut_at_illustration(body));
            let cleaned = clean_definition(gloss);
            match cleaned.split_whitespace().count() >= 2 {
                true => Some(cleaned),
                false => None,
            }
        })
        .collect()
}

/// Splits an entry on its sense numbers (`1.`, `2.`, ...).
///
/// Only a digit run followed by a period and a space starts a sense, so decimal
/// numbers and abbreviations inside a gloss do not split it.
fn split_senses(raw: &str) -> Vec<String> {
    let bytes: Vec<char> = raw.chars().collect();
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut i = 0usize;

    while i < bytes.len() {
        let starts_sense = bytes[i].is_ascii_digit()
            && (i == 0 || bytes[i - 1] == ' ' || bytes[i - 1] == '\n')
            && {
                let mut j = i;
                while j < bytes.len() && bytes[j].is_ascii_digit() {
                    j += 1;
                }
                j < bytes.len() && bytes[j] == '.' && bytes.get(j + 1) == Some(&' ')
            };

        if starts_sense && !cur.trim().is_empty() {
            out.push(std::mem::take(&mut cur));
        }
        cur.push(bytes[i]);
        i += 1;
    }
    if !cur.trim().is_empty() {
        out.push(cur);
    }
    match out.is_empty() {
        true => vec![raw.to_string()],
        false => out,
    }
}

/// Removes a sense's leading number and subject label.
///
/// `"1. (Zool.)  An animal..."` becomes `"An animal..."`. Without this the
/// sentence splitter stops at the period after the sense number and every gloss
/// comes back as the digit.
fn strip_sense_head(sense: &str) -> &str {
    let s = sense.trim_start();
    // Leading sense number: digits then '.'
    let s = match s.find(|c: char| !c.is_ascii_digit()) {
        Some(n) if n > 0 && s[n..].starts_with('.') => s[n + 1..].trim_start(),
        _ => s,
    };
    // Leading lettered subsense: "(a)" / "(b)".
    let s = match s.starts_with('(') && s.len() > 3 && s.as_bytes()[2] == b')' {
        true => s[3..].trim_start(),
        false => s,
    };
    // Leading subject label: "(Zool.)", "(Naut.)", "(Astron.)".
    match s.starts_with('(') {
        true => match s.find(')') {
            Some(n) => s[n + 1..].trim_start(),
            None => s,
        },
        false => s,
    }
}

/// Truncates a sense at the first marker that introduces illustration rather
/// than definition.
///
/// These are Webster's own structural signals, not guesses: `Note:` opens an
/// editorial digression, `as,` opens a list of usage examples, `See` and `Cf.`
/// open cross-references, `Syn.` opens a synonym list.
fn cut_at_illustration(sense: &str) -> &str {
    const MARKERS: [&str; 8] = ["Note:", "NOTE:", "; as,", ", as,", " See ", " Cf. ", "Syn.", "Usage:"];
    let mut end = sense.len();
    for m in MARKERS {
        if let Some(pos) = sense.find(m) {
            end = end.min(pos);
        }
    }
    &sense[..end]
}

/// The first sentence of a fragment.
///
/// Abbreviations ending in a period (`Felis domestica.`) are rare enough inside
/// a first gloss that a period followed by whitespace is a reliable boundary; a
/// single-letter token before the period is treated as an initial and skipped.
fn first_sentence(text: &str) -> &str {
    let chars: Vec<char> = text.chars().collect();
    let mut idx = 0usize;
    for (i, c) in chars.iter().enumerate() {
        if (*c == '.' || *c == ';') && chars.get(i + 1).is_none_or(|n| n.is_whitespace()) {
            // "J. Milton" — a lone capital before the period is an initial.
            let initial = i >= 2
                && chars[i - 1].is_ascii_uppercase()
                && !chars[i - 2].is_alphanumeric();
            if !initial {
                idx = i;
                break;
            }
        }
    }
    match idx {
        0 => text,
        n => {
            let byte_end: usize = chars[..n].iter().map(|c| c.len_utf8()).sum();
            &text[..byte_end]
        }
    }
}

/// Dictionary metadata tokens — grammatical labels, editorial abbreviations and
/// citation attributions that are about the entry rather than part of it.
fn is_apparatus(word: &str) -> bool {
    matches!(
        word,
        "n" | "v" | "adj" | "adv" | "prep" | "conj" | "interj" | "pron"
            | "noun" | "verb" | "adjective" | "adverb" | "participle"
            | "plural" | "singular" | "pl" | "sing" | "imp" | "pp"
            | "obs" | "obsolete" | "archaic" | "rare" | "colloq" | "dial"
            | "cf" | "viz" | "ie" | "eg" | "etym" | "etymology"
            | "syn" | "opp" | "abbr" | "var" | "cap" | "usu"
            | "webster" | "unabridged" | "shak" | "milton" | "dryden"
            | "spenser" | "tennyson" | "chaucer" | "pope" | "bacon"
            | "see" | "sometimes" | "formerly"
    )
}

#[cfg(test)]
mod clean_tests {
    use super::*;

    #[test]
    fn test_brackets_and_apparatus_are_removed() {
        let raw = "1. (Zool.) A small [OE. catte] furry animal; n. -- Shak.";
        let c = clean_definition(raw);
        assert!(!c.contains("zool"), "parenthetical apparatus removed: {}", c);
        assert!(!c.contains("catte"), "etymology bracket removed: {}", c);
        assert!(!c.contains("shak"), "citation removed: {}", c);
        assert!(!c.split_whitespace().any(|w| w == "n"), "pos marker removed: {}", c);
        assert!(c.contains("furry") && c.contains("animal"), "content survives: {}", c);
    }

    /// A1's golden file: the definitional core of a real Webster's entry must
    /// be the gloss, and must not contain the note, the usage examples, the
    /// cross-references or the quoted lines.
    #[test]
    fn test_definition_core_keeps_the_gloss_and_drops_illustration() {
        let cat = "1. (Zoöl.)  An animal of various species of the genera Felis \
                   and Lynx. The domestic cat is Felis domestica. The European wild \
                   cat (Felis catus) is much larger than the domestic cat. In the \
                   United States the name wild cat is commonly applied to the bay \
                   lynx (Lynx rufus) See Wild cat, and Tiger cat. Note: The domestic \
                   cat includes many varieties named from their place of origin or \
                   from some peculiarity; as, the Angora cat; the Maltese cat. \
                   2. (Naut.) A strong vessel with a narrow stern.";
        let core = definition_core(cat);

        assert!(core.contains("animal") && core.contains("species"), "gloss kept: {}", core);
        assert!(!core.contains("angora"), "usage examples dropped: {}", core);
        assert!(!core.contains("maltese"), "usage examples dropped: {}", core);
        assert!(!core.contains("varieties"), "editorial note dropped: {}", core);
        assert!(!core.contains("domestica"), "elaboration sentences dropped: {}", core);
        assert!(core.contains("vessel"), "sense 2 gloss kept: {}", core);
    }

    /// A quotation and its attribution are later sentences, so taking the first
    /// sentence excludes them without needing to know who the poet was. The
    /// apparatus list had grown a hardcoded roll of poets doing this job badly.
    #[test]
    fn test_quotations_and_attributions_are_dropped_structurally() {
        let car = "3. A chariot of war or of triumph; a vehicle of splendor. \
                   The gilded car of day. Milton. The towering car, the sable \
                   steeds. Tennyson.";
        let core = definition_core(car);
        assert!(core.contains("chariot"), "gloss kept: {}", core);
        assert!(!core.contains("gilded"), "quotation dropped: {}", core);
        assert!(!core.contains("sable"), "quotation dropped: {}", core);

        // Structural, not a name list: an invented poet must be dropped too.
        let made_up = "1. A small furry animal. The cat sat upon the mat. Quillfeather.";
        let c2 = definition_core(made_up);
        assert!(c2.contains("furry"), "gloss kept: {}", c2);
        assert!(!c2.contains("quillfeather"), "unknown attribution dropped: {}", c2);
    }

    /// Only the first senses are composed, or a word ends up at the centre of
    /// mass of meanings nobody uses.
    #[test]
    fn test_sense_count_is_capped() {
        let words = [
            "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel",
        ];
        let many: String = words
            .iter()
            .enumerate()
            .map(|(i, w)| format!("{}. A sense meaning {} entirely. ", i + 1, w))
            .collect();
        let core = definition_core(&many);

        for kept in &words[..MAX_SENSES] {
            assert!(core.contains(kept), "sense {} must be kept: {}", kept, core);
        }
        for dropped in &words[MAX_SENSES..] {
            assert!(!core.contains(dropped), "sense {} must be dropped: {}", dropped, core);
        }
    }

    #[test]
    fn test_plain_text_is_untouched_apart_from_case() {
        assert_eq!(clean_definition("An adult female person"), "an adult female person");
    }
}
