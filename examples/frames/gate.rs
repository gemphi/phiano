#[allow(unused_imports)]
use super::*;

/// One row per (pair, frame): the same pair appears several times, each with a
/// different C. That repetition is the experiment.
pub(super) struct Rows {
    pub(super) pairs: Vec<(String, String)>,
    pub(super) context: Vec<String>,
}

/// The gate: mine the pairs and the frames they occur in, and stop unless the
/// corpus delivers what the dictionary could not — pairs that recur in several
/// *different* sentences.
pub(super) fn gate(t: &Trained) -> Rows {
    let (facet, corpus) = (&t.facet, &t.corpus);
    let above = |w: &str| facet.lexicon.get(w).is_some_and(|p| p.count >= COUNT_FLOOR);
    let mut frames: HashMap<(String, String), Vec<usize>> = HashMap::new();
    let toks: Vec<Vec<String>> = corpus.iter().map(|s| Tokenizer::tokenize(s)).collect();

    for (si, t) in toks.iter().enumerate() {
        for i in 0..t.len() {
            if !above(&t[i]) {
                continue;
            }
            for j in (i + 1)..(i + 1 + WINDOW).min(t.len()) {
                if t[j] == t[i] || !above(&t[j]) {
                    continue;
                }
                frames
                    .entry((t[i].clone(), t[j].clone()))
                    .or_default()
                    .push(si);
            }
        }
    }

    let total_pairs = frames.len();
    let recurring: usize = frames.values().filter(|v| v.len() >= MIN_FRAMES).count();
    let mean_frames: f64 =
        frames.values().map(|v| v.len() as f64).sum::<f64>() / total_pairs.max(1) as f64;

    println!(
        "\n  === the gate ===\n\
         \x20 {} distinct pairs, {:.2} frames each on average\n\
         \x20 {} pairs ({:.1}%) occur in {} or more different sentences\n\
         \x20 the dictionary, for comparison, gives one gloss per head — a pair\n\
         \x20 there has exactly one frame by construction",
        total_pairs,
        mean_frames,
        recurring,
        recurring as f64 / total_pairs.max(1) as f64 * 100.0,
        MIN_FRAMES
    );
    if recurring < 2000 {
        eprintln!("\n  too few recurring pairs to test C. Stopping rather than reporting noise.");
        std::process::exit(1);
    }

    let mut ranked: Vec<((String, String), Vec<usize>)> = frames
        .into_iter()
        .filter(|(_, v)| v.len() >= MIN_FRAMES)
        .collect();
    ranked.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| a.0.cmp(&b.0)));

    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut context: Vec<String> = Vec::new();
    let mut distinct = 0usize;
    'outer: for (p, sents) in &ranked {
        let before = pairs.len();
        for si in sents.iter().take(MAX_ROWS_PER_PAIR) {
            // The frame with the pair's own words taken out.
            let c: Vec<String> = toks[*si]
                .iter()
                .filter(|w| **w != p.0 && **w != p.1)
                .cloned()
                .collect();
            if c.len() < 3 {
                continue;
            }
            pairs.push(p.clone());
            context.push(c.join(" "));
        }
        if pairs.len() > before {
            distinct += 1;
        }
        if pairs.len() >= MAX_ROWS {
            break 'outer;
        }
    }
    println!(
        "  scoring {} (pair, frame) rows over {} distinct pairs, at most {} frames each",
        pairs.len(),
        distinct,
        MAX_ROWS_PER_PAIR
    );

    Rows { pairs, context }
}
