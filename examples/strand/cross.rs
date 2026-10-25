//! Real text with real, clear shifts (docs/144): runs of paragraphs from
//! documents of different kinds — *Emma*, our own docs, the talk, Rust
//! source — joined so that no two neighbours are of the same kind. The prior
//! is *Emma* without any chapter used. Where the shifts are this clear, the
//! count should come near the truth.

use super::score::{f1, pk, Rng};
use phient::prose::strand::{Prior, Strand};
use phient::prose::tokens;
use std::path::Path;

fn files(dir: &Path, ext: &str, out: &mut Vec<std::path::PathBuf>) {
    let Ok(es) = std::fs::read_dir(dir) else { return };
    for e in es.flatten() {
        let p = e.path();
        if p.is_dir() && !p.ends_with("target") && !p.ends_with("papers") {
            files(&p, ext, out);
        } else if p.extension().is_some_and(|x| x == ext) {
            out.push(p);
        }
    }
}

/// `(kind, chapter index if Emma, paragraphs)` for every document.
fn pool(root: &Path) -> Vec<(usize, Option<usize>, Vec<String>)> {
    let paras = |t: &str| t.split("\n\n").map(|p| p.trim().to_string()).filter(|p| tokens(p).len() >= 4).collect::<Vec<_>>();
    let mut out = Vec::new();
    let sources = [("crates/phiano/data/text/emma", "txt"), ("crates/phiano/docs", "md"), ("crates/phiano/data/talks", "txt"), ("crates/phient/src", "rs")];
    for (kind, (dir, ext)) in sources.iter().enumerate() {
        let mut fs = Vec::new();
        files(&root.join(dir), ext, &mut fs);
        // The talks folder also holds the strand's own outputs: only the transcript is a talk.
        fs.retain(|f| kind != 2 || f.file_name().is_some_and(|n| n.to_string_lossy().starts_with("world_models_close")));
        fs.sort();
        for (i, f) in fs.iter().enumerate() {
            let t = std::fs::read_to_string(f).unwrap_or_default();
            let mut ps = paras(&t);
            if kind == 2 {
                // The talk is one paragraph: its sentences, in fours, are its paragraphs.
                let ss: Vec<&str> = t.split_inclusive(['.', '?']).collect();
                ps = ss.chunks(4).map(|c| c.concat()).collect();
            }
            if ps.len() >= 3 {
                out.push((kind, (kind == 0).then_some(i), ps));
            }
        }
    }
    out
}

/// Every document of the four kinds, and *Emma*'s chapters for the prior.
pub struct Pool {
    pub docs: Vec<(usize, Option<usize>, Vec<String>)>,
    chapters: Vec<String>,
}

impl Pool {
    pub fn read(root: &Path) -> Self {
        let docs = pool(root);
        let chapters = docs.iter().filter(|d| d.0 == 0).map(|d| d.2.join("\n\n")).collect();
        Pool { docs, chapters }
    }

    /// One join of `k` (2–8) runs, no two neighbours of a kind: `(text, truth, prior, kinds)`.
    pub fn draw(&self, rng: &mut Rng, len: usize) -> (String, Vec<usize>, Prior, Vec<usize>) {
        let k = 2 + rng.below(7);
        let (mut text, mut truth, mut n, mut kinds, mut used_ch) = (String::new(), Vec::new(), 0usize, Vec::new(), Vec::new());
        while kinds.len() < k {
            let d = &self.docs[rng.below(self.docs.len())];
            if kinds.last() == Some(&d.0) {
                continue;
            }
            if let Some(c) = d.1 {
                used_ch.push(c);
            }
            if !kinds.is_empty() {
                truth.push(n);
            }
            kinds.push(d.0);
            let (mut j, mut got) = (rng.below(d.2.len().saturating_sub(2).max(1)), 0);
            while got < len && j < d.2.len() {
                got += tokens(&d.2[j]).len();
                text.push_str(&d.2[j]);
                text.push_str("\n\n");
                j += 1;
            }
            n += got;
        }
        let mut prior = Prior::default();
        for (i, c) in self.chapters.iter().enumerate() {
            if !used_ch.contains(&i) {
                prior.add(c);
            }
        }
        (text, truth, prior, kinds)
    }
}

/// Trials over the four kinds; returns the summary.
pub fn run(root: &Path, trials: usize, seed: u64, len: usize, tol: usize) -> String {
    let pool = Pool::read(root);
    let mut rng = Rng(seed);
    let (mut rows, mut lines) = (Vec::new(), String::from("k\tn\tsupported\tpk_at_k\tf1_at_k\tkinds\n"));
    for _ in 0..trials {
        let (text, truth, prior, kinds) = pool.draw(&mut rng, len);
        let k = kinds.len();
        let s = Strand::new(&text, prior);
        let n = s.tokens.len();
        let at_k: Vec<usize> = s.cut.units_at(k).iter().map(|i| s.cut.spans[*i].start).filter(|x| *x > 0).collect();
        let u = s.informative_units();
        let (p, f) = (pk(n, &truth, &at_k), f1(&truth, &at_k, tol));
        lines.push_str(&format!("{k}\t{n}\t{u}\t{p:.4}\t{f:.4}\t{kinds:?}\n"));
        rows.push((k, u, p, f));
    }
    let m = rows.len() as f64;
    let exact = rows.iter().filter(|r| r.1 == r.0).count();
    let within1 = rows.iter().filter(|r| r.1.abs_diff(r.0) <= 1).count();
    let over = rows.iter().filter(|r| r.1 > r.0).count();
    let under = rows.iter().filter(|r| r.1 < r.0).count();
    format!(
        "{} trials · four kinds (Emma · our docs · the talk · Rust) · {} documents\nunits vs k: exact {exact}/{t}, within one {within1}/{t}, over {over}, under {under}, mean excess {:+.2}\nat true k: Pk {:.3}, boundary F1 ±{tol} {:.3}\n",
        rows.len(),
        pool.docs.len(),
        rows.iter().map(|r| r.1 as f64 - r.0 as f64).sum::<f64>() / m,
        rows.iter().map(|r| r.2).sum::<f64>() / m,
        rows.iter().map(|r| r.3).sum::<f64>() / m,
        t = rows.len()
    ) + &lines
}
