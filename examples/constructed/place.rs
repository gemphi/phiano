#[allow(unused_imports)]
use super::*;

/// Spreads a scalar position across the channels as its harmonics.
///
/// `theta_k = 2*pi * p * m_k` with `m_k` the k-th prime: one number becomes a
/// standing wave, which is how a scalar can occupy 64 channels without 63 of them
/// being copies. Magnitude is the same on every channel — it is a property of the
/// word, not of the channel.
pub(super) fn place(p: f64, amplitude: f64) -> Point {
    (0..PHASE_CHANNELS)
        .map(|k| {
            let m = CHANNEL_PRIMES[k % CHANNEL_PRIMES.len()] as f64;
            let t = (TWO_PI * p * m).rem_euclid(TWO_PI);
            (amplitude * t.cos(), amplitude * t.sin())
        })
        .collect()
}

/// Normalised complex inner product: magnitude counts.
pub(super) fn complex_similarity(a: &Point, b: &Point) -> f64 {
    let (mut dot, mut na, mut nb) = (0.0f64, 0.0f64, 0.0f64);
    for ((ax, ay), (bx, by)) in a.iter().zip(b.iter()) {
        dot += ax * bx + ay * by;
        na += ax * ax + ay * ay;
        nb += bx * bx + by * by;
    }
    match na > 1e-12 && nb > 1e-12 {
        true => dot / (na.sqrt() * nb.sqrt()),
        false => 0.0,
    }
}

/// Phase-only: exactly what `SpectralPhasor::resonance` does today.
pub(super) fn phase_similarity(a: &Point, b: &Point) -> f64 {
    let mut s = 0.0;
    for ((ax, ay), (bx, by)) in a.iter().zip(b.iter()) {
        s += (ay.atan2(*ax) - by.atan2(*bx)).cos();
    }
    s / PHASE_CHANNELS as f64
}

/// Fraction of probe pairs that out-resonate a random partner. Chance is 0.5.
pub(super) fn pair_vs_random<F: Fn(&Point, &Point) -> f64>(
    points: &HashMap<String, Point>,
    pool: &[&String],
    families: &[phient::metrics::relation::RelationFamily],
    sim: F,
    only: Option<&str>,
) -> (f64, usize) {
    let (mut wins, mut n) = (0.0f64, 0usize);
    for fam in families {
        if only.is_some_and(|f| f != fam.name) {
            continue;
        }
        for (i, p) in fam.pairs.iter().enumerate() {
            let (pa, pb) = match (points.get(&p.a), points.get(&p.b)) {
                (Some(a), Some(b)) => (a, b),
                _ => continue,
            };
            let target = sim(pa, pb);
            let (mut hits, mut draws) = (0usize, 0usize);
            for d in 0..DRAWS {
                let r = (fnv1a(&p.a) ^ ((i as u64) << 32) ^ (d as u64).wrapping_mul(0x9E37_79B9))
                    as usize;
                let cand = pool[r % pool.len().max(1)];
                if cand == &p.a || cand == &p.b {
                    continue;
                }
                draws += 1;
                if target > sim(pa, &points[cand]) {
                    hits += 1;
                }
            }
            if draws > 0 {
                wins += hits as f64 / draws as f64;
                n += 1;
            }
        }
    }
    match n {
        0 => (0.5, 0),
        _ => (wins / n as f64, n),
    }
}

/// Builds constructed positions from a banding and an ordering.
pub(super) fn construct(
    vocab: &[(String, u32)],
    band_of: &dyn Fn(&str) -> u64,
    alphabetical: bool,
    shuffle_bands: bool,
) -> HashMap<String, Point> {
    let mut bands: HashMap<u64, Vec<&(String, u32)>> = HashMap::new();
    for w in vocab {
        let b = match shuffle_bands {
            true => mix64(fnv1a(&format!("band:{}", w.0))),
            false => band_of(&w.0),
        };
        bands.entry(b).or_default().push(w);
    }

    let mut keys: Vec<u64> = bands.keys().copied().collect();
    keys.sort_unstable();
    let n_bands = keys.len().max(1) as f64;

    let max_count = vocab.iter().map(|w| w.1).max().unwrap_or(1).max(1) as f64;
    let mut out = HashMap::new();

    for (bi, b) in keys.iter().enumerate() {
        let members = bands.get_mut(b).expect("band present");
        match alphabetical {
            // The proposal: order within the band by the characters themselves.
            true => members.sort_by(|a, c| a.0.cmp(&c.0)),
            // Control: same band, order carrying no information.
            false => members.sort_by_key(|m| mix64(fnv1a(&format!("ord:{}", m.0)))),
        }
        let size = members.len().max(1) as f64;
        for (ri, m) in members.iter().enumerate() {
            let p = (bi as f64 + ri as f64 / size) / n_bands;
            // Magnitude is log frequency, so the complex number carries the
            // word's weight as well as its place.
            let a = (1.0 + m.1 as f64).ln() / (1.0 + max_count).ln();
            out.insert(m.0.clone(), place(p, a.max(1e-3)));
        }
    }
    out
}
