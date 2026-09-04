//! Does a relation channel work on the relation it is named after?
//! `cargo run --release --bin marked [corpus]`
//!
//! `bin/relational` established that traversing one channel composes better than
//! blending sixty-four, and then established that the channels doing the work
//! were all *offset* channels. Every one of its ten relation families peaks on
//! position. The named marker channels — `of`, `in`, `to`, `was` — are built,
//! correct, and idle.
//!
//! The reason is the benchmark and not the architecture. `king` and `queen` do
//! not appear either side of `of` in WikiText-2, so the gender channel has no
//! marker evidence about that pair and falls back to the one relation always
//! available, which is adjacency. Asking a marker channel about pairs it has
//! never seen under its marker measures nothing.
//!
//! So this benchmark takes its pairs from the corpus instead of from a word list.
//! For each marker `m`, the pairs are the words the text actually put either side
//! of `m`. Then the question that matters can be asked properly:
//!
//! 1. **Does the channel compose?** `a : b :: c : ?` on pairs joined by `m`,
//!    traversing `m`'s own channel.
//! 2. **Is the channel specific to its relation?** The same probes scored on
//!    *other* markers' channels. This is the load-bearing one. If channel `k`
//!    beats the others on relation `k`'s pairs, the dimension belongs to the
//!    relation, which is the whole claim. If every channel does equally well,
//!    then all that was learned is which words go together, and the assignment of
//!    relations to dimensions is decoration.
//! 3. **Does it beat the alternatives?** All sixty-four blended, and the shipped
//!    manifold, on identical probes.
//!
//! Every row carries a decoy null — the same scorer, same pool, same code path,
//! asked for a word that is not the answer — because a ratio against `1/N` was
//! wrong by a factor of nine the last time this was measured.
//!
//! # What it found
//!
//! Two things, and they point in opposite directions.
//!
//! **The dimension does belong to the relation.** Own-channel beats rival
//! channels on 10 of 12 markers, sign test p = 0.039, on identical probes with
//! identical pools differing only in which dimension the step is taken in. This
//! is the first direct evidence in the project that channel `k` holds something
//! specific to relation `k` — `bin/dimensions` looked for it in the shipped
//! manifold and found nothing, and `bin/relational` built the mechanism but
//! measured it on pairs that never co-occur under a marker, so its channels had
//! nothing to be specific about. The effect is small: 0.0224 against 0.0166.
//!
//! **And it does not pay.** On these pairs the ordering is the reverse of
//! `bin/relational`'s:
//!
//! ```text
//!                    MRR    vs null
//! own channel     0.0224       4.7x
//! rival channels  0.0166       2.8x
//! all 64 blended  0.1370      36.5x
//! shipped         0.5490     112.7x
//! ```
//!
//! Blending every dimension beats traversing the right one by six times, and the
//! shipped manifold beats it by twenty-four. `bin/relational` reported the
//! opposite — one channel at 5.2x its null while both dense readings sat at or
//! below theirs — and the difference is entirely in which pairs were asked
//! about. There, curated pairs that never co-occur, where sparse evidence is all
//! there is. Here, pairs the corpus actually placed either side of a marker,
//! where there is a great deal of evidence and using all of it wins.
//!
//! So the honest statement is narrower than either benchmark alone. Assigning a
//! relation its own dimension puts something real in that dimension. It is not a
//! better way to answer an analogy than reading every dimension at once, on any
//! pair where reading every dimension is possible.
//!
//! # What it does not show
//!
//! Pairs are mined from the same text the model trained on, so a pair's two words
//! have been coupled directly on that channel. That is why the interesting number
//! here is the *contrast between channels* and not the absolute MRR: a model that
//! merely memorised each pair would place them consistently on every channel it
//! touched, and would show no advantage for the right one.
//!
//! It also flatters the two dense scorers, and the shipped column especially.
//! With all four words of `a : b :: c : ?` drawn from pairs that co-occur, the
//! analogy is close to "name a word that keeps company with `c`", which is the
//! one question a context-trained manifold is built to answer. 0.549 means it
//! returns the exact word first about half the time, and that number should be
//! read as an upper bound on a easier question rather than as analogy accuracy.

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiano::metrics::harness::Harness;
use phiano::relational::{RelationVocab, RelationalFacet, MARKER_MAX_GAP, OFFSET_CHANNELS};
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;
use std::collections::HashMap;

/// A word must be seen this often to be a pair member or a distractor.
const COUNT_FLOOR: u32 = 25;

/// Distinct pairs a marker needs before its channel can be tested.
const MIN_PAIRS: usize = 16;

/// Pairs kept per marker, most frequent first.
const MAX_PAIRS: usize = 32;

/// Markers tested, in channel order — the most general relations first.
const MARKERS_TESTED: usize = 12;

/// Rival channels each probe is also scored on.
const RIVALS: usize = 4;

/// Ranking pool. Smaller than the full vocabulary because the all-channels
/// scorer costs sixty-four times the others, and a rank is a rank.
const POOL: usize = 1500;

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data/prose_corpus.txt".to_string());
    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let corpus: Vec<String> = raw
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| Tokenizer::tokenize(l).len() >= 5)
        .collect();
    if corpus.len() < 1000 {
        eprintln!("corpus too small at {path}. Run `bin/prepare_prose` first.");
        std::process::exit(1);
    }

    let split = Harness::split(corpus, 42);
    let toks: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();

    let base = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    let mut shipped = base.clone();
    Descent::cycle(&mut shipped, &toks, Up::Bag, 4, 0.25, true);

    let vocab = RelationVocab::learn(&toks);
    let rel = RelationalFacet::train(&toks, &vocab, 2);

    // ---- mine the pairs the text actually placed either side of each marker ----
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for d in &toks {
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
    for d in &toks {
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

    println!(
        "\n{} sentences, pool {}, {} marker channels mined\n",
        toks.len(),
        pool.len(),
        mined.len()
    );

    println!(
        "  {:<10} {:>6} {:>7}   {:>15} {:>15} {:>15} {:>15}",
        "", "", "", "own channel", "rival channels", "all 64 blended", "shipped"
    );
    println!(
        "  {:<10} {:>6} {:>7}   {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7}",
        "marker", "pairs", "probes", "MRR", "vs null", "MRR", "vs null", "MRR", "vs null", "MRR",
        "vs null"
    );

    let markers: Vec<usize> = (OFFSET_CHANNELS..PHASE_CHANNELS)
        .filter(|k| mined.get(k).map(|m| m.len()).unwrap_or(0) >= MIN_PAIRS)
        .take(MARKERS_TESTED)
        .collect();

    // Own-versus-rival is a paired comparison per marker, so the honest summary
    // of it is a sign test and not a ratio of two averages: one marker with a lot
    // of probes can carry a mean on its own.
    let (mut wins, mut losses) = (0usize, 0usize);
    let (mut t_own, mut t_riv, mut t_all, mut t_shp) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let (mut n_own, mut n_riv, mut n_all, mut n_shp) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut total = 0usize;

    for &k in &markers {
        let mut pairs: Vec<((String, String), usize)> =
            mined[&k].iter().map(|(p, c)| (p.clone(), *c)).collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        pairs.truncate(MAX_PAIRS);
        let pairs: Vec<(String, String)> = pairs.into_iter().map(|(p, _)| p).collect();

        // The rivals: other marker channels, spread across the range so they are
        // not all neighbours of `k`, and deterministic.
        let rivals: Vec<usize> = markers
            .iter()
            .copied()
            .filter(|&r| r != k)
            .enumerate()
            .filter(|(i, _)| i % (markers.len() / RIVALS).max(1) == 0)
            .map(|(_, r)| r)
            .take(RIVALS)
            .collect();

        // Rank on one channel: the step is an angle there, applied to `c`.
        let rr_on = |ch: usize, a: &str, b: &str, c: &str, target: &str, ex: &[&str]| -> f64 {
            let want = rel.words[c].theta[ch] + rel.words[b].theta[ch] - rel.words[a].theta[ch];
            let t = (rel.words[target].theta[ch] - want).cos();
            let r = 1 + pool
                .iter()
                .filter(|w| w.as_str() != target && !ex.contains(&w.as_str()))
                .filter(|w| (rel.words[**w].theta[ch] - want).cos() > t)
                .count();
            1.0 / r as f64
        };

        let (mut own, mut riv, mut all, mut shp) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
        let (mut nown, mut nriv, mut nall, mut nshp) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
        let mut n = 0usize;
        let stride = (pairs.len() / 5).max(1);

        for (i, (a, b)) in pairs.iter().enumerate() {
            for (j, (c, d)) in pairs.iter().enumerate() {
                if i == j || j % stride != i % stride {
                    continue;
                }
                let ex = [a.as_str(), b.as_str(), c.as_str()];
                let decoy = match pool
                    .iter()
                    .cycle()
                    .skip((i * 7919 + j * 104_729) % pool.len())
                    .take(pool.len())
                    .find(|w| w.as_str() != d && !ex.contains(&w.as_str()))
                {
                    Some(w) => w.as_str(),
                    None => continue,
                };

                own += rr_on(k, a, b, c, d, &ex);
                nown += rr_on(k, a, b, c, decoy, &ex);
                for &r in &rivals {
                    riv += rr_on(r, a, b, c, d, &ex) / rivals.len() as f64;
                    nriv += rr_on(r, a, b, c, decoy, &ex) / rivals.len() as f64;
                }

                // All sixty-four blended, evidence-weighted.
                let score_all = |w: &str| -> f64 {
                    let (pw, pa, pb, pc) =
                        (&rel.words[w], &rel.words[a], &rel.words[b], &rel.words[c]);
                    let (mut dot, mut nw) = (0.0f64, 0.0f64);
                    for kk in 0..PHASE_CHANNELS {
                        let m = pw.mag[kk] * pa.mag[kk].min(pb.mag[kk]).min(pc.mag[kk]);
                        if m > 0.0 {
                            let tgt = pc.theta[kk] + pb.theta[kk] - pa.theta[kk];
                            dot += m * (pw.theta[kk] - tgt).cos();
                            nw += m;
                        }
                    }
                    match nw > 0.0 {
                        true => dot / nw,
                        false => f64::NEG_INFINITY,
                    }
                };
                let rr_all = |target: &str| -> f64 {
                    let t = score_all(target);
                    let r = 1 + pool
                        .iter()
                        .filter(|w| w.as_str() != target && !ex.contains(&w.as_str()))
                        .filter(|w| score_all(w) > t)
                        .count();
                    1.0 / r as f64
                };
                all += rr_all(d);
                nall += rr_all(decoy);

                // The shipped manifold.
                let sq = {
                    let (pa, pb, pc) =
                        (&shipped.lexicon[a], &shipped.lexicon[b], &shipped.lexicon[c]);
                    let mut q = *pc;
                    for kk in 0..PHASE_CHANNELS {
                        q.set_theta(kk, pc.theta(kk) + pb.theta(kk) - pa.theta(kk));
                    }
                    q.sync_phase();
                    q
                };
                let rr_ship = |target: &str| -> f64 {
                    let t = sq.resonance(&shipped.lexicon[target]);
                    let r = 1 + pool
                        .iter()
                        .filter(|w| w.as_str() != target && !ex.contains(&w.as_str()))
                        .filter(|w| sq.resonance(&shipped.lexicon[**w]) > t)
                        .count();
                    1.0 / r as f64
                };
                shp += rr_ship(d);
                nshp += rr_ship(decoy);
                n += 1;
            }
        }
        if n == 0 {
            continue;
        }
        let f = n as f64;
        let ratio = |x: f64, y: f64| x / y.max(f64::MIN_POSITIVE);
        println!(
            "  {:<10} {:>6} {:>7} {:>9.5} {:>6.1}x {:>7.5} {:>6.1}x {:>7.5} {:>6.1}x {:>7.5} {:>6.1}x",
            vocab.name(k),
            pairs.len(),
            n,
            own / f,
            ratio(own, nown),
            riv / f,
            ratio(riv, nriv),
            all / f,
            ratio(all, nall),
            shp / f,
            ratio(shp, nshp),
        );
        match own / f > riv / f {
            true => wins += 1,
            false => losses += 1,
        }
        t_own += own;
        t_riv += riv;
        t_all += all;
        t_shp += shp;
        n_own += nown;
        n_riv += nriv;
        n_all += nall;
        n_shp += nshp;
        total += n;
    }

    if total > 0 {
        let f = total as f64;
        let ratio = |x: f64, y: f64| x / y.max(f64::MIN_POSITIVE);
        println!(
            "  {:<10} {:>6} {:>7} {:>9.5} {:>6.1}x {:>7.5} {:>6.1}x {:>7.5} {:>6.1}x {:>7.5} {:>6.1}x",
            "ALL",
            "",
            total,
            t_own / f,
            ratio(t_own, n_own),
            t_riv / f,
            ratio(t_riv, n_riv),
            t_all / f,
            ratio(t_all, n_all),
            t_shp / f,
            ratio(t_shp, n_shp),
        );
        println!(
            "  {:<10} {:>6} {:>7} {:>9.5} {:>7} {:>7.5} {:>7} {:>7.5} {:>7} {:>7.5}",
            "decoy null",
            "",
            total,
            n_own / f,
            "",
            n_riv / f,
            "",
            n_all / f,
            "",
            n_shp / f,
        );
        // Two-sided sign test on the paired own-versus-rival outcome.
        let m = wins + losses;
        let c = |n: usize, k: usize| -> f64 {
            (0..k).fold(1.0f64, |acc, i| acc * (n - i) as f64 / (i + 1) as f64)
        };
        let tail: f64 = (wins.max(losses)..=m).map(|i| c(m, i)).sum();
        let p = (2.0 * tail / 2f64.powi(m as i32)).min(1.0);
        println!(
            "\n  own beats rival on {wins} of {m} markers, sign test p = {p:.3}"
        );
        println!(
            "\n  The column that decides it is own-channel against rival-channels. \
             Those are\n  the same probes, the same pool and the same scorer, \
             differing only in which\n  dimension the step is taken in. A gap there \
             means the relation owns the\n  dimension. No gap means the model \
             learned which words go together and the\n  assignment of relations to \
             channels carries nothing."
        );
    }
}
