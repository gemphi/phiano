#[allow(unused_imports)]
use super::*;

/// The pairs the text placed either side of each marker, and the pool every
/// scorer ranks inside.
pub(super) struct Mined {
    pub(super) mined: HashMap<usize, HashMap<(String, String), usize>>,
    pub(super) pool: Vec<String>,
}

/// Mine the pairs the text actually placed either side of each marker.
pub(super) fn mine_pairs(t: &Trained) -> Mined {
    let (toks, vocab, rel, shipped) = (&t.toks, &t.vocab, &t.rel, &t.shipped);
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for d in toks {
        for w in d {
            *counts.entry(w.as_str()).or_insert(0) += 1;
        }
    }
    let known = |w: &str| {
        counts.get(w).copied().unwrap_or(0) >= COUNT_FLOOR
            && rel.words.contains_key(w)
            && shipped.lexicon.contains_key(w)
    };

    let mut mined: HashMap<usize, HashMap<(String, String), usize>> = HashMap::new();
    for d in toks {
        for i in 0..d.len() {
            for j in (i + 2)..(i + 1 + MARKER_MAX_GAP).min(d.len()) {
                let k = match vocab.channel(d, i, j) {
                    Some(k) if k >= OFFSET_CHANNELS => k,
                    _ => continue,
                };
                if !known(&d[i]) || !known(&d[j]) || d[i] == d[j] {
                    continue;
                }
                *mined
                    .entry(k)
                    .or_default()
                    .entry((d[i].clone(), d[j].clone()))
                    .or_insert(0) += 1;
            }
        }
    }

    let mut pool: Vec<&String> = counts
        .keys()
        .filter(|w| known(w))
        .map(|w| toks.iter().flatten().find(|t| t.as_str() == *w).unwrap())
        .collect();
    pool.sort();
    pool.dedup();
    pool.truncate(POOL);
    let pool: Vec<String> = pool.into_iter().cloned().collect();

    println!(
        "\n{} sentences, pool {}, {} marker channels mined\n",
        toks.len(),
        pool.len(),
        mined.len()
    );

    Mined { mined, pool }
}
