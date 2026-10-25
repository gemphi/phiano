//! The signature as a condition (docs/118). Movement sees only the last K
//! tokens; once the body is a few tokens long, the signature has left the
//! window. Here every word of the signature is a condition on the whole
//! body: `w` said in the signature ⇒ `v` said in the body, counted over
//! Phi-4's answers in training. A name relates to a method (`sum` ⇒ `sum`,
//! `max` ⇒ `max`), `Option` to `Some` and `None`, a parameter to itself.
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions (c): here c is "in the signature", and the movement is
//! ln P(v in body | w in signature) − ln P(v in body).

use std::collections::{HashMap, HashSet};

pub struct Sig {
    rel: HashMap<u32, HashMap<u32, f64>>,
    told: HashMap<u32, f64>,
    body: Vec<f64>,
    files: f64,
    /// prev token ⇒ (body tokens after it, of which a word of the signature):
    /// the relation "said in the signature ⇒ said again here".
    copy: HashMap<u32, (f64, f64)>,
    copy_all: (f64, f64),
}

/// A file's signature (up to its first `{`) and its body, as id sets.
fn split(toks: &[u32], open: u32) -> (HashSet<u32>, HashSet<u32>) {
    let i = toks.iter().position(|&t| t == open).unwrap_or(toks.len());
    (toks[..i].iter().copied().collect(), toks[(i + 1).min(toks.len())..].iter().copied().collect())
}

impl Sig {
    pub fn new(files: &[Vec<u32>], v: usize, open: u32, ident: &dyn Fn(u32) -> bool) -> Self {
        let mut s = Sig { rel: HashMap::new(), told: HashMap::new(), body: vec![0.0; v], files: files.len() as f64, copy: HashMap::new(), copy_all: (0.0, 0.0) };
        for f in files {
            let (sig, body) = split(f, open);
            let start = f.iter().position(|&t| t == open).map_or(f.len(), |i| i + 1);
            for i in start.max(1)..f.len() {
                let hit = (sig.contains(&f[i]) && ident(f[i]) && f[i] != 0) as usize as f64;
                let e = s.copy.entry(f[i - 1]).or_default();
                e.0 += 1.0;
                e.1 += hit;
                s.copy_all.0 += 1.0;
                s.copy_all.1 += hit;
            }
            body.iter().for_each(|&b| s.body[b as usize] += 1.0);
            for &w in sig.iter().filter(|&&w| w != 0) {
                *s.told.entry(w).or_default() += 1.0;
                let m = s.rel.entry(w).or_default();
                body.iter().for_each(|&b| *m.entry(b).or_default() += 1.0);
            }
        }
        s
    }

    /// The movement each body token gets from the words of `sig`.
    pub fn boost(&self, sig: &[u32]) -> Vec<f64> {
        let p: Vec<f64> = self.body.iter().map(|c| (c + 0.5) / (self.files + 1.0)).collect();
        let mut out = vec![0.0; p.len()];
        let words: HashSet<u32> = sig.iter().copied().filter(|&w| w != 0).collect();
        for w in words {
            let (Some(m), Some(&n)) = (self.rel.get(&w), self.told.get(&w)) else { continue };
            for (&v, &c) in m {
                out[v as usize] += ((c + p[v as usize]) / (n + 1.0) / p[v as usize]).ln();
            }
        }
        out
    }

    /// How likely the next body token repeats a word of the signature, after `prev`.
    pub fn kappa(&self, prev: u32) -> f64 {
        let g = (self.copy_all.1 + 1.0) / (self.copy_all.0 + 2.0);
        let (n, k) = self.copy.get(&prev).copied().unwrap_or((0.0, 0.0));
        (k + 2.0 * g) / (n + 2.0)
    }
}
