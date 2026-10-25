//! Does the strand find a text's true structure? (docs/143.) Each trial
//! takes runs of paragraphs from `k` different chapters of *Emma*, joins
//! them, and asks the strand to divide the result, with the prior read from
//! the *other* chapters so nothing it is tested on was counted. Measured:
//! the units the information supports against `k`; the boundaries at the
//! supported level and at the true count against equal-length and random
//! cuts (Pk, boundary F1 within ±`tol` tokens); and the same text with its
//! tokens shuffled, which has no structure to find.

use super::score::{equal, f1, pk, Rng};
use phient::prose::strand::{Prior, Strand};
use phient::prose::tokens;
use std::fmt::Write;
use std::path::Path;

/// One trial's numbers.
pub struct Trial {
    pub k: usize,
    pub n: usize,
    pub supported: usize,
    pub shuffled: usize,
    pub pk_supported: f64,
    pub pk_at_k: f64,
    pub pk_equal: f64,
    pub pk_random: f64,
    pub f1_at_k: f64,
    pub f1_equal: f64,
    pub f1_random: f64,
    /// Of the strand's cuts that are not a true boundary (±tol), how many fall
    /// within 2 tokens of a paragraph start, and how many would by chance.
    pub extra: usize,
    pub extra_at_paragraph: usize,
    pub chance_at_paragraph: f64,
}

fn paragraphs(text: &str) -> Vec<String> {
    text.split("\n\n").map(|p| p.trim().to_string()).filter(|p| !p.is_empty()).collect()
}

fn starts(s: &Strand, units: usize) -> Vec<usize> {
    s.cut.units_at(units).iter().map(|i| s.cut.spans[*i].start).filter(|x| *x > 0).collect()
}

/// *Emma*'s chapters, each counted, and all of them counted together.
pub struct Emma {
    pub chapters: Vec<String>,
    per: Vec<Prior>,
    whole: Prior,
}

/// A joined text: its text, where its chapters begin, where its paragraphs begin, and the prior without its chapters.
pub struct Joined {
    pub text: String,
    pub truth: Vec<usize>,
    pub paras: Vec<usize>,
    pub prior: Prior,
}

impl Emma {
    pub fn read(dir: &Path) -> Self {
        let mut files: Vec<_> = std::fs::read_dir(dir).expect("chapters").flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "txt")).collect();
        files.sort();
        let chapters: Vec<String> = files.iter().map(|f| std::fs::read_to_string(f).unwrap_or_default()).collect();
        let mut whole = Prior::default();
        let per: Vec<Prior> = chapters.iter().map(|c| { let mut p = Prior::default(); p.add(c); whole.add(c); p }).collect();
        Emma { chapters, per, whole }
    }

    /// Runs of about `len` tokens from `k` different chapters, joined; the prior is *Emma* without them.
    pub fn draw(&self, rng: &mut Rng, k: usize, len: usize) -> Joined {
        let mut picked: Vec<usize> = Vec::new();
        while picked.len() < k {
            let c = rng.below(self.chapters.len());
            if !picked.contains(&c) {
                picked.push(c);
            }
        }
        let mut prior = self.whole.clone();
        for c in &picked {
            for (w, n) in &self.per[*c].counts {
                if let Some(x) = prior.counts.get_mut(w) {
                    *x -= n;
                }
            }
            prior.total -= self.per[*c].total;
        }
        prior.counts.retain(|_, n| *n > 0);
        let (mut text, mut truth, mut n, mut paras) = (String::new(), Vec::new(), 0usize, Vec::new());
        for (i, c) in picked.iter().enumerate() {
            let ps = paragraphs(&self.chapters[*c]);
            let mut j = rng.below(ps.len().saturating_sub(3).max(1));
            let mut got = 0;
            if i > 0 {
                truth.push(n);
            }
            while got < len && j < ps.len() {
                let t = tokens(&ps[j]).len();
                if got > 0 {
                    paras.push(n + got);
                }
                text.push_str(&ps[j]);
                text.push_str("\n\n");
                got += t;
                j += 1;
            }
            n += got;
        }
        Joined { text, truth, paras, prior }
    }
}

/// Run `trials` trials over the chapters in `dir`; segments of about `len` tokens.
pub fn run(dir: &Path, trials: usize, seed: u64, len: usize, tol: usize, kmin: usize) -> (Vec<Trial>, String) {
    let emma = Emma::read(dir);
    let mut rng = Rng(seed);
    let mut out = Vec::new();
    for _ in 0..trials {
        let k = kmin + rng.below(9 - kmin.min(8));
        let Joined { text, truth, paras, prior } = emma.draw(&mut rng, k, len);
        let s = Strand::new(&text, prior.clone());
        let n = s.tokens.len();
        let supported = s.informative_units();
        let own: Vec<usize> = starts(&s, supported).into_iter().filter(|c| !truth.iter().any(|t: &usize| t.abs_diff(*c) <= tol)).collect();
        let near = |c: &usize| paras.iter().any(|p: &usize| p.abs_diff(*c) <= 2);
        let extra_at_paragraph = own.iter().filter(|c| near(c)).count();
        let chance_at_paragraph = own.len() as f64 * (paras.len() as f64 * 5.0 / n as f64).min(1.0);
        let at_k = starts(&s, k);
        let mut sh = s.tokens.clone();
        rng.shuffle(&mut sh);
        let shuffled = Strand::new(&sh.join(" "), prior).informative_units();
        let (eq, mut pr, mut fr) = (equal(n, k), 0.0, 0.0);
        for _ in 0..20 {
            let r = rng.cuts(n, k);
            pr += pk(n, &truth, &r) / 20.0;
            fr += f1(&truth, &r, tol) / 20.0;
        }
        out.push(Trial {
            k,
            n,
            supported,
            shuffled,
            pk_supported: pk(n, &truth, &starts(&s, supported)),
            pk_at_k: pk(n, &truth, &at_k),
            pk_equal: pk(n, &truth, &eq),
            pk_random: pr,
            f1_at_k: f1(&truth, &at_k, tol),
            f1_equal: f1(&truth, &eq, tol),
            f1_random: fr,
            extra: own.len(),
            extra_at_paragraph,
            chance_at_paragraph,
        });
    }
    let tsv = out.iter().fold(String::from("k\tn\tsupported\tshuffled\tpk_supported\tpk_at_k\tpk_equal\tpk_random\tf1_at_k\tf1_equal\tf1_random\n"), |mut acc, t| {
        writeln!(acc, "{}\t{}\t{}\t{}\t{:.4}\t{:.4}\t{:.4}\t{:.4}\t{:.4}\t{:.4}\t{:.4}", t.k, t.n, t.supported, t.shuffled, t.pk_supported, t.pk_at_k, t.pk_equal, t.pk_random, t.f1_at_k, t.f1_equal, t.f1_random).ok();
        acc
    });
    (out, tsv)
}

/// The summary a person reads.
pub fn report(t: &[Trial], tol: usize) -> String {
    let m = |f: &dyn Fn(&Trial) -> f64| t.iter().map(f).sum::<f64>() / t.len().max(1) as f64;
    let exact = t.iter().filter(|x| x.supported == x.k).count();
    let within1 = t.iter().filter(|x| x.supported.abs_diff(x.k) <= 1).count();
    let (mk, ms) = (m(&|x| x.k as f64), m(&|x| x.supported as f64));
    let cov = m(&|x| (x.k as f64 - mk) * (x.supported as f64 - ms));
    let (sk, ss) = (m(&|x| (x.k as f64 - mk).powi(2)).sqrt(), m(&|x| (x.supported as f64 - ms).powi(2)).sqrt());
    let mut s = String::new();
    writeln!(s, "{} trials · k = {}…8 chapters · {:.0} tokens a text on average", t.len(), t.iter().map(|x| x.k).min().unwrap_or(0), m(&|x| x.n as f64)).ok();
    writeln!(s, "units supported vs true k: exact {exact}/{n}, within one {within1}/{n}, mean |u − k| {:.2}, correlation {:.2}", m(&|x| x.supported.abs_diff(x.k) as f64), cov / (sk * ss).max(1e-9), n = t.len()).ok();
    writeln!(s, "shuffled (no structure): mean units {:.2}, one unit in {}/{}", m(&|x| x.shuffled as f64), t.iter().filter(|x| x.shuffled == 1).count(), t.len()).ok();
    writeln!(s, "Pk (lower is better): strand at its own count {:.3} · strand at true k {:.3} · equal parts {:.3} · random cuts {:.3}", m(&|x| x.pk_supported), m(&|x| x.pk_at_k), m(&|x| x.pk_equal), m(&|x| x.pk_random)).ok();
    let (e, ep, ec) = (t.iter().map(|x| x.extra).sum::<usize>(), t.iter().map(|x| x.extra_at_paragraph).sum::<usize>(), t.iter().map(|x| x.chance_at_paragraph).sum::<f64>());
    writeln!(s, "the strand's cuts that are not a chapter boundary: {e}; within 2 tokens of a paragraph start: {ep} ({:.0}%), by chance {ec:.0} ({:.0}%)", 100.0 * ep as f64 / e.max(1) as f64, 100.0 * ec / e.max(1) as f64).ok();
    writeln!(s, "boundary F1 within ±{tol} tokens, at true k: strand {:.3} · equal parts {:.3} · random cuts {:.3}", m(&|x| x.f1_at_k), m(&|x| x.f1_equal), m(&|x| x.f1_random)).ok();
    s
}
