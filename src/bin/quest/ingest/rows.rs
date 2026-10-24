//! The quest table, and the documents it becomes.
#[allow(unused_imports)]
use super::*;

/// The one row type of the workspace, read as phiecs reads it.
pub use phiecs::Row;

/// Read the table; a header row is skipped because its weight does not parse.
pub fn read_rows(path: &str) -> Vec<Row> {
    match std::fs::read_to_string(path) {
        Ok(t) => phiecs::parse_tsv(&t),
        Err(e) => {
            eprintln!("cannot read {path}: {e}");
            std::process::exit(1);
        }
    }
}

/// How many times a row is said to the trainer: its weight on a scale of
/// [`WEIGHT_REPEATS`], never fewer than once. A claim the oracle held at 0.9
/// is heard four times as often as one it held at 0.2.
pub const WEIGHT_REPEATS: f64 = 4.0;

pub fn repeats(weight: f64) -> usize {
    ((weight * WEIGHT_REPEATS).round() as usize).max(1)
}

/// Every row as the three-token document `about kind other`, repeated by
/// weight. The kind word stands *between* the pair, which is exactly what
/// [`phiano::relational::RelationVocab`] reads as a marker: the quest's kinds
/// become marker channels without any mapping written here.
pub fn documents(rows: &[Row]) -> Vec<Vec<String>> {
    let mut docs = Vec::new();
    for r in rows {
        let doc = vec![r.about.clone(), r.kind.clone(), r.other.clone()];
        for _ in 0..repeats(r.weight) {
            docs.push(doc.clone());
        }
    }
    docs
}

/// The kinds in the table, in first-seen order.
pub fn kinds(rows: &[Row]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for r in rows {
        if !out.contains(&r.kind) {
            out.push(r.kind.clone());
        }
    }
    out
}

/// Every word that is the `other` of some row — the pool a recovered target
/// is ranked against. Sorted, so the ranking is reproducible.
pub fn target_pool(rows: &[Row]) -> Vec<String> {
    let mut pool: Vec<String> = rows.iter().map(|r| r.other.clone()).collect();
    pool.sort();
    pool.dedup();
    pool
}

/// The words the oracle ever named under `kind` — the pool the quest's own
/// question ranks against: "kind | other" where `other` is one of the things
/// anything stands in that relation to. Sorted.
pub fn kind_pool(rows: &[Row], kind: &str) -> Vec<String> {
    let mut pool: Vec<String> = rows.iter().filter(|r| r.kind == kind).map(|r| r.other.clone()).collect();
    pool.sort();
    pool.dedup();
    pool
}
