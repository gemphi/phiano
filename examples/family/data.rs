//! Phi-4's relation table as words and their relations: every row
//! `about · kind · other` is one relation of `about`, written `kind:other`.

use phiecs::quest::Row;
use std::collections::{BTreeMap, BTreeSet};

pub struct Table {
    pub rows: Vec<Row>,
    /// Word ⇒ its relations (`kind:other`), is-a included.
    pub rel: BTreeMap<String, BTreeSet<String>>,
}

pub fn feature(r: &Row) -> String {
    format!("{}:{}", r.kind, r.other)
}

impl Table {
    pub fn read(path: &str) -> Self {
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
        let rows = phiecs::quest::parse_tsv(&text);
        let mut rel: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for r in &rows {
            rel.entry(r.about.clone()).or_default().insert(feature(r));
        }
        Table { rows, rel }
    }

    /// A word's relations other than is-a: what it is like, not what it is filed under.
    pub fn properties(&self, w: &str) -> BTreeSet<String> {
        self.rel.get(w).map(|s| s.iter().filter(|f| !f.starts_with("is-a:")).cloned().collect()).unwrap_or_default()
    }

    /// Categories: every `p` that at least `min` words are told to be (is-a),
    /// counting only words with at least three properties.
    pub fn categories(&self, min: usize) -> Vec<(String, Vec<String>)> {
        let mut by: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for r in self.rows.iter().filter(|r| r.kind == "is-a") {
            let v = by.entry(r.other.clone()).or_default();
            if !v.contains(&r.about) && self.properties(&r.about).len() >= 3 {
                v.push(r.about.clone());
            }
        }
        by.into_iter().filter(|(_, v)| v.len() >= min).collect()
    }
}
