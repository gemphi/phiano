#[allow(unused_imports)]
use super::*;

pub(super) fn variants() -> usize {
    *VARIANTS_CELL.get_or_init(|| 3)
}

/// The phase state of an occurrence's neighbourhood, excluding the word itself.
pub(super) fn window_state(facet: &Facet, toks: &[String], at: usize) -> Vec<f64> {
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

pub(super) fn agree(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).cos()).sum::<f64>() / PHASE_CHANNELS as f64
}

impl Roles {
    /// Clusters each frequent word's occurrences by their window state.
    ///
    /// Deterministic throughout: farthest-point seeding, fixed rounds, words
    /// visited in sorted order.
    pub(super) fn learn(facet: &Facet, docs: &[Vec<String>]) -> Self {
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
    pub(super) fn assign(&self, facet: &Facet, toks: &[String], at: usize) -> Option<usize> {
        let cs = self.centroids.get(&toks[at])?;
        let s = window_state(facet, toks, at);
        cs.iter()
            .enumerate()
            .max_by(|a, b| agree(&s, a.1).partial_cmp(&agree(&s, b.1)).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
    }

    pub(super) fn split_words(&self) -> usize {
        self.centroids.len()
    }
}

pub(super) fn key(scheme: Scheme, roles: &Roles, facet: &Facet, toks: &[String], at: usize) -> String {
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
pub(super) fn next_word_loss(
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
