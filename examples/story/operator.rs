//! The operator beside the counts on held-out text (docs/136, docs/137):
//! phinet's Joint FNO trained on the told paragraphs' `word next word`
//! rows (`RelationModel`), read from a position's last word by cosine over
//! every word that ever came next. Trained once and saved (`--model`), it
//! is the slow speed for the same table the counts fill at once; as a
//! distribution it is the base the counts can move from (`Knowing::base`).

#[path = "reader.rs"]
mod reader;
pub use reader::Reader;

use phiano::prose::for_each_paragraph;
use phiano::relate::RelationModel;
use phiecs::quest::Row;
use phistd::Adam;
use phical::{FnoConfig, Tie};
use std::time::Instant;

/// The told `word next word` rows under `root` (paragraphs `held` names
/// left out) and the held-out paragraphs' words.
pub fn rows(root: &std::path::Path, held: &dyn Fn(&str) -> bool) -> (Vec<Row>, Vec<Vec<String>>) {
    rows_ahead(root, held, 1)
}

/// As `rows`, with the words up to `ahead` places on as kinds of their own:
/// `next`, then `next2`, `next3` … (the offset channels, docs/151).
pub fn rows_ahead(root: &std::path::Path, held: &dyn Fn(&str) -> bool, ahead: usize) -> (Vec<Row>, Vec<Vec<String>>) {
    let (mut told, mut heldout): (Vec<Row>, Vec<Vec<String>>) = (Vec::new(), Vec::new());
    for_each_paragraph(root, &mut |path, rows| {
        let words: Vec<String> = rows.iter().filter(|r| r.kind == "has-word").map(|r| r.other.clone()).collect();
        match held(&path) {
            true => heldout.push(words),
            false => {
                for d in 1..=ahead {
                    let kind = if d == 1 { "next".to_string() } else { format!("next{d}") };
                    told.extend(words.iter().zip(words.iter().skip(d)).map(|(a, b)| Row { about: a.clone(), kind: kind.clone(), other: b.clone(), weight: 1.0 }));
                }
            }
        }
    });
    (told, heldout)
}

/// The model saved at `path` when there is one, else trained `epochs`
/// epochs at R = `space` on `told` and saved there. Returns it with a line
/// on where it came from.
pub fn model(told: &[Row], space: usize, epochs: u64, path: Option<&str>) -> (RelationModel, String) {
    if let Some(m) = path.and_then(|p| RelationModel::load(p).ok()) {
        let note = format!("operator loaded from {} (R = {})", path.unwrap_or(""), m_space(&m));
        return (m, note);
    }
    let cfg = FnoConfig { space, width: 16, layers: 2, modes: (space / 4).min(16), margin: 0.3, lr: 5e-3, tie: Tie::Circle };
    let mut m = RelationModel::new(told, None, cfg, 7);
    let triples: Vec<_> = told.iter().filter_map(|r| m.triple(r)).collect();
    let examples: Vec<_> = triples.iter().map(phical::relate::example).collect();
    let (pool, _) = phical::relate::pools(&triples);
    let (mut ao, mut at) = (Adam::new(cfg.lr), Adam::new(cfg.lr * 0.5));
    let t = Instant::now();
    let mut loss = 0.0;
    for e in 0..epochs {
        loss = m.joint.train_epoch(&examples, &pool, 32, &mut ao, &mut at, e);
    }
    let note = format!("operator trained {epochs} epochs at R = {space} on {} rows in {:.0} s (last hinge loss {loss:.3})", told.len(), t.elapsed().as_secs_f64());
    if let Some(p) = path {
        if let Err(e) = m.save(p) {
            eprintln!("could not save {p}: {e}");
        }
    }
    (m, note)
}

fn m_space(m: &RelationModel) -> usize {
    m.joint.fno.config.space
}

/// Cosines as a distribution at temperature `t`.
pub fn soften(c: &[f64], t: f64) -> Vec<f64> {
    let top = c.iter().cloned().fold(f64::MIN, f64::max);
    let e: Vec<f64> = c.iter().map(|x| ((x - top) / t).exp()).collect();
    let z: f64 = e.iter().sum();
    e.into_iter().map(|x| x / z).collect()
}
