//! Held-out positions split by what the counts know (docs/137): where the
//! word that came was told after the last word or the last pair, and where
//! it was not — the stratum the counts can only answer with the prior, and
//! the one the operator (docs/137) and the kin reading (docs/138) are for.
//! Then the readings together: the levels moving from the operator's or the
//! kin's distribution, and the field cast with the operator's word functions.

use super::operator;
use phiano::prose::{heldout, Knowing};
use phiano::relate::{Field, RelationModel};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default)]
struct Tally {
    n: usize,
    top1: [usize; 4],
    rr: [f64; 4],
}

impl Tally {
    fn add(&mut self, ranks: [Option<usize>; 4]) {
        self.n += 1;
        for (i, r) in ranks.iter().enumerate() {
            if let Some(r) = r {
                self.top1[i] += (*r == 0) as usize;
                self.rr[i] += 1.0 / (*r + 1) as f64;
            }
        }
    }

    fn row(&self, name: &str) -> String {
        let n = self.n.max(1) as f64;
        let cell = |i: usize| format!("{:.1}% · {:.3}", 100.0 * self.top1[i] as f64 / n, self.rr[i] / n);
        format!("| {name} | {} | {} | {} | {} | {} |\n", self.n, cell(0), cell(1), cell(2), cell(3))
    }
}

fn rank_of(ranked: &[(String, f64)], truth: &str) -> Option<usize> {
    ranked.iter().position(|x| x.0 == truth)
}

fn ranked(values: &[String], scores: Vec<f64>) -> Vec<(String, f64)> {
    let mut r: Vec<(String, f64)> = values.iter().cloned().zip(scores).collect();
    r.sort_by(|a, b| b.1.total_cmp(&a.1));
    r
}

/// The two strata, read by the counts (flat), the operator (the last word,
/// when there is one), the last word's kin, and the prior alone: `top-1 ·
/// MRR` each.
pub fn strata(k: &mut Knowing, mut m: Option<&mut RelationModel>) -> String {
    let values = k.counts.values_and_prior("next");
    let (values, prior) = (&values.0, &values.1);
    let mut by_prior: Vec<(String, f64)> = values.iter().cloned().zip(prior.iter().cloned()).collect();
    by_prior.sort_by(|a, b| b.1.total_cmp(&a.1));
    let (mut known, mut unknown) = (Tally::default(), Tally::default());
    k.flat = true;
    for (_, words) in k.held.clone() {
        for i in 2..words.len() {
            let (last, truth) = (&words[i - 1], &words[i]);
            let pair = format!("pair:{} {}", words[i - 2], last);
            let told = k.counts.count(Some(last), "next", truth) > 0.0 || k.counts.count(Some(&pair), "next", truth) > 0.0;
            let counts = rank_of(&k.next(&words[..i]).1, truth);
            let op = m.as_deref_mut().and_then(|m| m.scores(last, "next", values)).and_then(|c| rank_of(&ranked(values, c), truth));
            let kin = k.counts.kin_base(last, "next", values).and_then(|b| rank_of(&ranked(values, b), truth));
            let p = rank_of(&by_prior, truth);
            match told {
                true => known.add([counts, op, kin, p]),
                false => unknown.add([counts, op, kin, p]),
            }
        }
    }
    k.flat = false;
    let mut md = String::from("| the word that came was… | positions | counts (flat) | operator (last word) | kin of the last word | prior alone |\n|---|---|---|---|---|---|\n");
    md += &known.row("told after the last word or pair");
    md += &unknown.row("never told after either");
    md
}

/// The levels moving from the kin reading, flat and in the tree.
pub fn with_kin(k: &mut Knowing) -> String {
    let mut md = String::new();
    k.kin = true;
    k.flat = true;
    md += &heldout::score(k).row("flat, moving from the last word's kin");
    md += "\n";
    k.flat = false;
    md += &heldout::score(k).row("tree by counts, moving from the last word's kin");
    md += "\n";
    k.kin = false;
    md
}

/// The readings together with the operator, each a row of the held-out table.
pub fn together(k: &mut Knowing, m: Rc<RefCell<RelationModel>>, reader: operator::Reader) -> String {
    let mut md = String::new();
    k.base = Some(reader.base(m.clone()));
    k.flat = true;
    md += &heldout::score(k).row("flat, moving from the operator's distribution");
    md += "\n";
    k.flat = false;
    md += &heldout::score(k).row("tree by counts, moving from the operator's distribution");
    md += "\n";
    k.base = None;
    let words: HashMap<String, phistd::Function> = {
        let mm = m.borrow();
        let table = mm.joint.functions();
        mm.names.iter().cloned().zip(table).filter(|(n, _)| !n.starts_with('⟨')).collect()
    };
    let space = m.borrow().joint.fno.config.space;
    k.field = Some(Field::with_words(&k.counts, &k.tree, space, words));
    md += &heldout::score(k).row(&format!("tree, cast by phasor with the operator's word functions (R = {space})"));
    k.field = None;
    md
}
