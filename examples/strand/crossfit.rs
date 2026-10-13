//! Cross-fitting the strand's weights (docs/148). docs/145 §6 found that
//! reading a unit down the strand's tree costs more than reading it straight
//! under the whole. It guessed why: a span is chosen because its own tokens
//! repeat, and its weight is then counted from those same tokens. Here the
//! guess is tested. In each fold the training tokens are dealt into two
//! halves, A and B, alternately. A strand is built on A alone, and its
//! weights are counted twice: left out on A (the old way), and on B, tokens
//! the division never saw (`Strand::scopes_witnessed`). If the guess is
//! right, the tree read with B's weights is no longer worse than the flat
//! units. A third count, on the held-out words themselves (not a reading,
//! a diagnostic), says whether any weights could make the tree read as well.
//! The path reading (`strand::Path`) counts each unit's weights over its
//! whole path for that unit alone.

use phiplm::prose::strand::{Path, Prior, Scopes, Strand};

pub const READERS: [&str; 12] = [
    "whole, left out",
    "labels, left out",
    "labels, path, left out",
    "flat, left out",
    "tree, left out",
    "path, left out",
    "flat, cross-fit (half)",
    "tree, cross-fit (half)",
    "path, cross-fit (half)",
    "flat, weights on the read words (half)",
    "tree, weights on the read words (half)",
    "path, weights on the read words (half)",
];

/// One text in every fold: held-out words and nats summed by reader.
pub struct Fit {
    pub held: usize,
    pub nats: [f64; 12],
}

/// A flat scope: (start, end, the scope it sits under).
type Span = (usize, usize, Option<usize>);

/// The supported units and the flat scopes over them: the whole, then each unit under it.
fn flat(s: &Strand) -> (Vec<usize>, Vec<Span>) {
    let units = s.cut.units_at(s.informative_units());
    let mut spans = vec![(0, s.tokens.len(), None)];
    spans.extend(units.iter().map(|u| (s.cut.spans[*u].start, s.cut.spans[*u].end, Some(0))));
    (units, spans)
}

/// A strand's three readings of a word at `at`: flat, down its tree, along its path.
struct Read {
    strand: Strand,
    units: Vec<usize>,
    flat: Scopes,
    tree: Scopes,
    paths: Vec<Path>,
}

impl Read {
    /// `witness`: tokens to count the weights on, or `None` to leave each of the strand's own out.
    fn new(words: Vec<String>, prior: &Prior, witness: Option<&[(usize, String)]>) -> Self {
        let strand = Strand::from_tokens(words, prior.clone());
        let (units, spans) = flat(&strand);
        let ids: Vec<u32> = strand.tokens.iter().map(|t| strand.index[t]).collect();
        let (flat, tree, paths) = match witness {
            None => {
                let (tree, _, paths) = strand.unit_paths();
                (Scopes::fit(&ids, &spans, &|id| prior.p(&strand.vocab[id as usize])), tree, paths)
            }
            Some(w) => {
                let wt: Vec<(usize, Option<u32>, f64)> = w.iter().map(|(p, x)| (*p, strand.index.get(x).copied(), prior.p(x))).collect();
                let tree = strand.scopes_witnessed(w);
                let paths = units.iter().map(|u| {
                    let sp = &strand.cut.spans[*u];
                    let rows: Vec<(Option<u32>, f64)> = wt.iter().filter(|x| x.0 >= sp.start && x.0 < sp.end).map(|x| (x.1, x.2)).collect();
                    tree.path(*u, &rows, false)
                });
                (Scopes::fit_witnessed(&ids, &spans, &wt), tree.clone(), paths.collect())
            }
        };
        Read { strand, units, flat, tree, paths }
    }

    fn at(&self, at: usize, word: &str) -> [f64; 3] {
        let s = &self.strand;
        let j = self.units.partition_point(|u| s.cut.spans[*u].start <= at).saturating_sub(1);
        let (id, p) = (s.index.get(word).copied(), s.prior.p(word));
        [self.flat.read(1 + j, id, p), self.tree.read(self.units[j], id, p), self.tree.read_path(&self.paths[j], id, p)]
    }
}

/// Where position `i` falls among `axis` (sorted original positions): the token before it.
fn anchor(axis: &[usize], i: usize) -> usize {
    axis.partition_point(|p| *p < i).saturating_sub(1)
}

pub fn text(tokens: &[String], truth: &[usize], prior: &Prior, folds: usize) -> Fit {
    let mut f = Fit { held: 0, nats: [0.0; 12] };
    for fold in 0..folds {
        let train: Vec<usize> = (0..tokens.len()).filter(|i| i % folds != fold).collect();
        let (a, b): (Vec<usize>, Vec<usize>) = (train.iter().step_by(2).copied().collect(), train.iter().skip(1).step_by(2).copied().collect());
        let held: Vec<usize> = (0..tokens.len()).filter(|i| i % folds == fold).collect();
        let words = |axis: &[usize]| axis.iter().map(|i| tokens[*i].clone()).collect::<Vec<_>>();
        let witness = |of: &[usize], on: &[usize]| of.iter().map(|i| (anchor(on, *i), tokens[*i].clone())).collect::<Vec<_>>();
        let all = Read::new(words(&train), prior, None);
        let a_by_b = Read::new(words(&a), prior, Some(&witness(&b, &a)));
        let a_by_read = Read::new(words(&a), prior, Some(&witness(&held, &a)));
        // The whole, and the true segments under it, on the same four fifths.
        let ids: Vec<u32> = all.strand.tokens.iter().map(|x| all.strand.index[x]).collect();
        let n = ids.len();
        let mut cuts: Vec<usize> = truth.iter().map(|t| train.partition_point(|p| p < t)).filter(|c| *c > 0 && *c < n).collect();
        cuts.dedup();
        let bounds: Vec<usize> = std::iter::once(0).chain(cuts.iter().copied()).chain(std::iter::once(n)).collect();
        let mut spans = vec![(0, n, None)];
        spans.extend(bounds.windows(2).map(|w| (w[0], w[1], Some(0))));
        let pr = |id: u32| prior.p(&all.strand.vocab[id as usize]);
        let (whole, labels) = (Scopes::fit(&ids, &spans[..1], &pr), Scopes::fit(&ids, &spans, &pr));
        let label_paths: Vec<Path> = (1..spans.len()).map(|j| labels.path(j, &ids[spans[j].0..spans[j].1].iter().map(|id| (Some(*id), pr(*id))).collect::<Vec<_>>(), true)).collect();
        for &i in &held {
            let w = &tokens[i];
            let at = anchor(&train, i);
            let (id, pw) = (all.strand.index.get(w).copied(), prior.p(w));
            let seg = cuts.partition_point(|c| *c <= at);
            let p: Vec<f64> = [vec![whole.read(0, id, pw), labels.read(1 + seg, id, pw), labels.read_path(&label_paths[seg], id, pw)], all.at(at, w).to_vec(), a_by_b.at(anchor(&a, i), w).to_vec(), a_by_read.at(anchor(&a, i), w).to_vec()].concat();
            for (n, q) in f.nats.iter_mut().zip(p) {
                *n -= q.ln();
            }
            f.held += 1;
        }
    }
    f
}

/// Nats a held-out word by reader, and how often the tree beats the flat units.
pub fn summary(name: &str, fs: &[Fit]) -> String {
    let held: usize = fs.iter().map(|f| f.held).sum();
    let per = |j: usize| fs.iter().map(|f| f.nats[j]).sum::<f64>() / held.max(1) as f64;
    let wins = |t: usize, l: usize| fs.iter().filter(|f| f.nats[t] < f.nats[l]).count();
    let mut s = format!("{name}: {} texts · {held} words read\n", fs.len());
    for (j, r) in READERS.iter().enumerate() {
        s += &format!("  {r:<34} {:.4}\n", per(j));
    }
    s += &format!("  left out, against the whole: labels {:+.4} · labels as a path {:+.4} · flat {:+.4} · tree {:+.4} · path {:+.4}; path beats labels as a path in {}/{n} texts\n", per(1) - per(0), per(2) - per(0), per(3) - per(0), per(4) - per(0), per(5) - per(0), wins(5, 2), n = fs.len());
    for (i, how) in ["left out", "cross-fit (half)", "weights on the read words (half)"].iter().enumerate() {
        let j = 3 + 3 * i;
        s += &format!("  {how}: tree − flat {:+.4}, path − flat {:+.4}; tree beats flat in {}/{n}, path in {}/{n} texts\n", per(j + 1) - per(j), per(j + 2) - per(j), wins(j + 1, j), wins(j + 2, j), n = fs.len());
    }
    s
}
