//! Is `X as Y` the right unit of representation?
//! `cargo run --release --bin senses [corpus]`
//!
//! "X counts as Y" is not only about `cow counts as animal`. Its commonest
//! instance is the one every sentence performs: *greet* as verb, *cool* as
//! adjective, *mum* as noun — and the same surface word taking a different Y in
//! a different place. `cool` is an adjective in "cool water" and a verb in "cool
//! the engine", and those are two different things wearing one spelling.
//!
//! `bin/frames` showed this project cannot represent that. Two occurrences of the
//! same word pair land on the same harmonic mode **100.0% of the time**, because
//! `offset(a, b) = theta(b) - theta(a)` and each word has exactly one phasor. The
//! relation between two words is a constant of those two words, so no context can
//! vary it and Searle's third term has nowhere to live.
//!
//! The fix the argument implies: make `X as Y` the unit rather than `X`. A word
//! gets one position per role it plays instead of one position.
//!
//! # Where Y comes from
//!
//! Not from a tag set. `phinum::PosDictionary` maps each word to a single
//! `PartOfSpeech`, which is the same collapse one level up — it cannot say `cool`
//! is an adjective here and a verb there. Y is discovered from use instead, which
//! is what the CLU argument asks for: each occurrence of a word is described by
//! the phase state of its local window, those descriptions are clustered, and the
//! clusters are the word's roles. Nothing is named, and the number of roles per
//! word is bounded.
//!
//! # The measurement, and the control that decides it
//!
//! Splitting a word into `k` variants divides its evidence `k` ways and multiplies
//! its parameters by `k`. Both effects change the score on their own, so "split
//! beats unsplit" is not a result.
//!
//! * **unsplit** — one phasor per word. What the project ships.
//! * **sense-split** — variants discovered from context.
//! * **RANDOM-split** — the same number of variants per word, occurrences
//!   assigned by a hash instead of by context. Identical vocabulary growth,
//!   identical count dilution, no linguistic information. **This is the control.**
//!
//! Scored as held-out log-loss of the *next surface word*. The target stays the
//! surface word in every arm, so the label space is identical and the three
//! numbers are directly comparable — which they would not be if the split changed
//! what was being predicted.
//!
//! # What it measured
//!
//! WikiText-2 prose, 76,765 training and 9,596 held-out sentences, 3,291 words
//! split (seen 60+ times). Log-loss in nats per token, lower better:
//!
//! ```text
//!   k   unsplit    sense    random   sense cost   random cost   recovered
//!   2    7.4776   7.4808    7.7485       0.0032        0.2709       98.8%
//!   3    7.4776   7.4846    7.9032       0.0070        0.4256       98.4%
//!   4    7.4776   7.4893    8.0180       0.0117        0.5404       97.8%
//! ```
//!
//! **The roles are real, and nearly free.** Dividing a word's evidence `k` ways
//! costs 0.27 to 0.54 nats when the division carries no information. Dividing it
//! by discovered role costs 0.003 to 0.012 — between 1.2% and 2.2% of that. A
//! third of the occurrences predicting almost as well as all of them is only
//! possible if the variants are internally coherent, and a hash assignment with
//! identical vocabulary growth and identical count dilution cannot do it.
//!
//! **And they do not yet pay.** Sense splitting is slightly *worse* than not
//! splitting, at every `k`. Affordable is not the same as profitable.
//!
//! That is not surprising in this particular test, and it is worth saying why
//! rather than treating it as a verdict on the idea. Next-word prediction is
//! close to the worst place to look for a payoff: the immediate successor of a
//! token is largely fixed by its surface form, so knowing that `cool` is the
//! adjective rather than the verb buys little about what follows it. Where the
//! split should pay is exactly where this project has been failing — relations,
//! where `bin/frames` found two occurrences of a pair on the same mode 100% of
//! the time because the words have one position each. That number cannot be 100%
//! once a word has several, and whether it drops for a *reason* is the next
//! measurement, with the random split as its control again.

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiano::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::phasor::{fnv1a, mix64};
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;
use std::collections::HashMap;

/// Words seen fewer times than this are not split: a handful of occurrences
/// cannot support several roles, and splitting them only dilutes.
const SPLIT_FLOOR: u32 = 60;

/// Tokens either side of an occurrence that describe it.
const WINDOW: usize = 3;

/// Roles per word, overridable from the command line.
///
/// Bounded, in the spirit of the rest of the project — and small, because the
/// evidence per role falls as it rises and the split has to pay for itself.
static VARIANTS_CELL: std::sync::OnceLock<usize> = std::sync::OnceLock::new();

fn variants() -> usize {
    *VARIANTS_CELL.get_or_init(|| 3)
}

/// Laplace smoothing on the bigram tables.
const ALPHA: f64 = 0.1;

/// The phase state of an occurrence's neighbourhood, excluding the word itself.
fn window_state(facet: &Facet, toks: &[String], at: usize) -> Vec<f64> {
    let lo = at.saturating_sub(WINDOW);
    let hi = (at + WINDOW + 1).min(toks.len());
    let mut acc = vec![(0.0f64, 0.0f64); PHASE_CHANNELS];
    for (i, w) in toks.iter().enumerate().take(hi).skip(lo) {
        if i == at {
            continue;
        }
        if let Some(p) = facet.lexicon.get(w) {
            for (k, a) in acc.iter_mut().enumerate() {
                let t = p.theta(k);
                a.0 += t.cos();
                a.1 += t.sin();
            }
        }
    }
    acc.iter().map(|(x, y)| y.atan2(*x)).collect()
}

fn agree(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).cos()).sum::<f64>() / PHASE_CHANNELS as f64
}

/// Per-word role centroids, learned from the training half only.
struct Roles {
    centroids: HashMap<String, Vec<Vec<f64>>>,
}

impl Roles {
    /// Clusters each frequent word's occurrences by their window state.
    ///
    /// Deterministic throughout: farthest-point seeding, fixed rounds, words
    /// visited in sorted order.
    fn learn(facet: &Facet, docs: &[Vec<String>]) -> Self {
        let mut by_word: HashMap<&str, Vec<Vec<f64>>> = HashMap::new();
        for toks in docs {
            for (i, w) in toks.iter().enumerate() {
                let frequent = facet
                    .lexicon
                    .get(w)
                    .is_some_and(|p| p.count >= SPLIT_FLOOR);
                if frequent {
                    by_word
                        .entry(w.as_str())
                        .or_default()
                        .push(window_state(facet, toks, i));
                }
            }
        }

        let mut words: Vec<&str> = by_word.keys().copied().collect();
        words.sort_unstable();

        let mut centroids = HashMap::new();
        for w in words {
            let states = &by_word[w];
            if states.len() < variants() * 8 {
                continue;
            }
            // Farthest-point seeds, then a few rounds of assignment and circular
            // means. Same procedure as everywhere else in this project.
            let mut seeds: Vec<Vec<f64>> = vec![states[0].clone()];
            while seeds.len() < variants() {
                let next = states
                    .iter()
                    .min_by(|a, b| {
                        let sim = |s: &Vec<f64>| {
                            seeds.iter().map(|c| agree(s, c)).fold(f64::NEG_INFINITY, f64::max)
                        };
                        sim(a).partial_cmp(&sim(b)).unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .cloned();
                match next {
                    Some(n) => seeds.push(n),
                    None => break,
                }
            }
            for _ in 0..6 {
                let mut acc = vec![vec![(0.0f64, 0.0f64); PHASE_CHANNELS]; seeds.len()];
                for s in states {
                    let c = seeds
                        .iter()
                        .enumerate()
                        .max_by(|a, b| {
                            agree(s, a.1).partial_cmp(&agree(s, b.1)).unwrap_or(std::cmp::Ordering::Equal)
                        })
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    for k in 0..PHASE_CHANNELS {
                        acc[c][k].0 += s[k].cos();
                        acc[c][k].1 += s[k].sin();
                    }
                }
                for (c, a) in acc.iter().enumerate() {
                    for k in 0..PHASE_CHANNELS {
                        if a[k].0.hypot(a[k].1) > 1e-12 {
                            seeds[c][k] = a[k].1.atan2(a[k].0);
                        }
                    }
                }
            }
            centroids.insert(w.to_string(), seeds);
        }
        Self { centroids }
    }

    /// Which role this occurrence plays, or `None` when the word is not split.
    fn assign(&self, facet: &Facet, toks: &[String], at: usize) -> Option<usize> {
        let cs = self.centroids.get(&toks[at])?;
        let s = window_state(facet, toks, at);
        cs.iter()
            .enumerate()
            .max_by(|a, b| agree(&s, a.1).partial_cmp(&agree(&s, b.1)).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
    }

    fn split_words(&self) -> usize {
        self.centroids.len()
    }
}

/// How a token is keyed in the bigram table's *condition* side.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Scheme {
    Unsplit,
    Sense,
    Random,
}

fn key(scheme: Scheme, roles: &Roles, facet: &Facet, toks: &[String], at: usize) -> String {
    let w = &toks[at];
    match scheme {
        Scheme::Unsplit => w.clone(),
        Scheme::Sense => match roles.assign(facet, toks, at) {
            Some(v) => format!("{w}#{v}"),
            None => w.clone(),
        },
        // Same words split, same number of variants, assignment by hash of the
        // position rather than by context. The point of comparison.
        Scheme::Random => match roles.centroids.contains_key(w) {
            true => {
                let h = mix64(fnv1a(&format!("{w}:{at}")) ^ (at as u64));
                format!("{w}#{}", h as usize % variants())
            }
            false => w.clone(),
        },
    }
}

/// Held-out log-loss of the next *surface* word, in nats.
fn next_word_loss(
    scheme: Scheme,
    roles: &Roles,
    facet: &Facet,
    train: &[Vec<String>],
    held: &[Vec<String>],
    vocab: usize,
) -> f64 {
    let mut joint: HashMap<(String, String), f64> = HashMap::new();
    let mut cond: HashMap<String, f64> = HashMap::new();
    for toks in train {
        for i in 0..toks.len().saturating_sub(1) {
            let k = key(scheme, roles, facet, toks, i);
            *joint.entry((k.clone(), toks[i + 1].clone())).or_insert(0.0) += 1.0;
            *cond.entry(k).or_insert(0.0) += 1.0;
        }
    }

    let v = vocab as f64;
    let (mut ll, mut n) = (0.0f64, 0.0f64);
    for toks in held {
        for i in 0..toks.len().saturating_sub(1) {
            let k = key(scheme, roles, facet, toks, i);
            let num = joint.get(&(k.clone(), toks[i + 1].clone())).copied().unwrap_or(0.0) + ALPHA;
            let den = cond.get(&k).copied().unwrap_or(0.0) + ALPHA * v;
            ll -= (num / den).ln();
            n += 1.0;
        }
    }
    ll / n.max(1.0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/prose_corpus.txt".to_string());
    let _ = VARIANTS_CELL.set(args.get(2).and_then(|s| s.parse().ok()).unwrap_or(3));

    let raw = match std::fs::read_to_string(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("could not read {path}: {e}\nRun `bin/prepare_prose` first.");
            std::process::exit(1);
        }
    };
    let corpus: Vec<String> = raw
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| Tokenizer::tokenize(l).len() >= 5)
        .collect();

    let split = Harness::split(corpus, 42);
    let facet = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);

    let train: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let held: Vec<Vec<String>> = split.valid.iter().map(|s| Tokenizer::tokenize(s)).collect();

    // Roles are learned on the training half only. Learning them on everything
    // would let the held-out sentences choose their own conditioning.
    let roles = Roles::learn(&facet, &train);
    let vocab = facet.vocabulary_size();

    println!(
        "prose: {} train / {} held-out sentences, {} words\n\
         \x20 {} words split into {} roles each (seen {}+ times)",
        train.len(),
        held.len(),
        vocab,
        roles.split_words(),
        variants(),
        SPLIT_FLOOR
    );

    println!(
        "\n  {:<16} {:>14} {:>14}",
        "conditioning", "log-loss", "vs unsplit"
    );
    let base = next_word_loss(Scheme::Unsplit, &roles, &facet, &train, &held, vocab);
    println!("  {:<16} {:>14.4} {:>14}", "unsplit (X)", base, "-");

    for (label, scheme) in [
        ("sense (X as Y)", Scheme::Sense),
        ("RANDOM split", Scheme::Random),
    ] {
        let ll = next_word_loss(scheme, &roles, &facet, &train, &held, vocab);
        println!("  {:<16} {:>14.4} {:>+14.4}", label, ll, base - ll);
    }

    println!(
        "\n  Log-loss in nats per token, predicting the next surface word; lower is\n\
         \x20 better and 'vs unsplit' is positive when splitting helped. The target is\n\
         \x20 the surface word in every arm, so the label space is identical and the\n\
         \x20 three numbers compare directly.\n\n\
         \x20 The RANDOM row is the one that decides it. Splitting a word into {} and\n\
         \x20 dividing its evidence three ways changes the score by itself, so sense\n\
         \x20 splitting has to beat a split that carries no information — not merely\n\
         \x20 beat no split at all.",
        variants()
    );
}
