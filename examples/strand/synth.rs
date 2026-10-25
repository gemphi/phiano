//! Text whose only structure is known (docs/144). Each of `k` segments is
//! drawn word by word, independently, from its own distribution: English
//! (Emma's unigram) mixed with weight `lambda` into a topic of 30 words of
//! its own. Inside a segment there is nothing to find, so a criterion that
//! is calibrated supports `k` units when the topics are strong enough to
//! see and fewer when they are not — never more. Sentence marks fall every
//! 10–20 words, independent of the topics, and every segment ends on one, as
//! a chapter does.

use super::score::{f1, Rng};
use phient::prose::strand::{Prior, Strand};

/// One synthetic text: `(words, where segments 1… begin)`.
pub fn draw(vocab: &[(String, f64)], rng: &mut Rng, lambda: f64, len: usize) -> (Vec<String>, Vec<usize>) {
    let k = 1 + rng.below(8);
    let cum: Vec<f64> = vocab.iter().scan(0.0, |a, (_, p)| { *a += p; Some(*a) }).collect();
    let total = *cum.last().unwrap_or(&1.0);
    let draw = |rng: &mut Rng| {
        let u = (rng.next() >> 11) as f64 / (1u64 << 53) as f64 * total;
        cum.partition_point(|c| *c < u).min(vocab.len() - 1)
    };
    // Topics: 30 words each from the middle of the vocabulary (ranks 200–5,000).
    let (lo, hi) = (200.min(vocab.len() - 1), 5000.min(vocab.len()));
    let (mut words, mut truth, mut until_mark) = (Vec::new(), Vec::new(), 10 + rng.below(11));
    for j in 0..k {
        if j > 0 {
            truth.push(words.len());
        }
        let topic: Vec<usize> = (0..30).map(|_| lo + rng.below(hi - lo)).collect();
        let n = len / 2 + rng.below(len);
        for i in 0..n {
            let from_topic = ((rng.next() >> 11) as f64 / (1u64 << 53) as f64) < lambda;
            let w = if from_topic { topic[rng.below(topic.len())] } else { draw(rng) };
            words.push(vocab[w].0.clone());
            until_mark -= 1;
            if until_mark == 0 || i + 1 == n {
                words.push(".".into());
                until_mark = 10 + rng.below(11);
            }
        }
    }
    (words, truth)
}

/// One synthetic trial: `(k, units supported, boundary F1 at k within ±tol)`.
pub fn trial(prior: &Prior, vocab: &[(String, f64)], rng: &mut Rng, lambda: f64, len: usize, tol: usize) -> (usize, usize, f64) {
    let (words, truth) = draw(vocab, rng, lambda, len);
    let k = truth.len() + 1;
    let s = Strand::new(&words.join(" "), prior.clone());
    let at_k: Vec<usize> = s.cut.units_at(k).iter().map(|i| s.cut.spans[*i].start).filter(|x| *x > 0).collect();
    (k, s.informative_units(), f1(&truth, &at_k, tol))
}

/// The vocabulary of a prior as `(word, probability)`, words only, most common first.
pub fn vocabulary(prior: &Prior) -> Vec<(String, f64)> {
    let mut v: Vec<(String, f64)> = prior.counts.iter().filter(|(w, _)| w.chars().any(char::is_alphanumeric)).map(|(w, c)| (w.clone(), *c as f64)).collect();
    v.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    v
}

/// A sweep over topic strengths: per `lambda`, trials, mean |u − k|, over- and under-counts, exact, F1.
pub fn sweep(prior: &Prior, trials: usize, seed: u64, len: usize, tol: usize, lambdas: &[f64]) -> String {
    let vocab = vocabulary(prior);
    let mut rng = Rng(seed);
    let mut out = String::from("lambda\ttrials\texact\tover\tunder\tmean_abs\tmean_excess\tf1_at_k\n");
    for &l in lambdas {
        let r: Vec<(usize, usize, f64)> = (0..trials).map(|_| trial(prior, &vocab, &mut rng, l, len, tol)).collect();
        let n = r.len() as f64;
        let exact = r.iter().filter(|x| x.1 == x.0).count();
        let over = r.iter().filter(|x| x.1 > x.0).count();
        let under = r.iter().filter(|x| x.1 < x.0).count();
        let abs = r.iter().map(|x| x.1.abs_diff(x.0) as f64).sum::<f64>() / n;
        let excess = r.iter().map(|x| x.1 as f64 - x.0 as f64).sum::<f64>() / n;
        let f = r.iter().filter(|x| x.0 > 1).map(|x| x.2).sum::<f64>() / r.iter().filter(|x| x.0 > 1).count().max(1) as f64;
        out.push_str(&format!("{l}\t{trials}\t{exact}\t{over}\t{under}\t{abs:.2}\t{excess:+.2}\t{f:.3}\n"));
    }
    out
}
