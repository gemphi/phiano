//! The counts readings of the relation board (docs/124, docs/126): the
//! same told rows read by `phiano::relate::Counts`, flat or under the
//! path's scopes, scored like the operator's readings.

use super::newfn::GIVEN;
use phiano::relate::{RelationModel, Triple, Weights};
use phiecs::quest::Row;
use phical::relate;
use std::collections::HashMap;

/// `phiano::relate::Counts` read over the same rows, scored like the rest.
pub fn counts_score(m: &RelationModel, told: &[Triple], test: &[Triple], pool: &[usize], by_kind: &HashMap<usize, Vec<usize>>, tau: f64) -> relate::Score {
    counts_read(m, told, test, pool, by_kind, tau, false, Weights::Breadth)
}

/// `levels`: the function's scopes, outermost first, each a level, then
/// its words and types (docs/126); else every condition flat (docs/124).
/// `weights`: how much a level trusts its own counts (docs/146–147).
#[allow(clippy::too_many_arguments)]
pub fn counts_read(m: &RelationModel, told: &[Triple], test: &[Triple], pool: &[usize], by_kind: &HashMap<usize, Vec<usize>>, tau: f64, levels: bool, weights: Weights) -> relate::Score {
    let row = |t: &Triple| Row { about: m.names[t.about].clone(), kind: m.names[t.kind].trim_matches(['⟨', '⟩']).to_string(), other: m.names[t.other].clone(), weight: 1.0 };
    let rows: Vec<Row> = told.iter().map(row).collect();
    let mut c = phiano::relate::Counts::new(&GIVEN);
    c.weights = weights;
    c.learn(&rows);
    // Each scope's parent, from the `within` rows between scopes.
    let parent: HashMap<&str, &str> = rows.iter().filter(|r| r.kind == "within" && r.about.starts_with(phiano::relate::SCOPE)).map(|r| (r.about.as_str(), r.other.as_str())).collect();
    let table: Vec<HashMap<usize, f64>> = test
        .iter()
        .map(|t| {
            let r = row(t);
            let values: Vec<String> = by_kind.get(&t.kind).into_iter().flatten().map(|&v| m.names[v].clone()).collect();
            let conds = c.conditions(&r.about);
            let s = match levels {
                false => c.read(&conds, &r.kind, &values, tau).0,
                true => {
                    let (scopes, own): (Vec<String>, Vec<String>) = conds.iter().cloned().partition(|x| x.starts_with(phiano::relate::SCOPE));
                    let depth = |s: &String| std::iter::successors(Some(s.as_str()), |x| parent.get(x).copied()).count();
                    let mut scopes = scopes;
                    scopes.sort_by_key(depth);
                    let mut lv: Vec<Vec<String>> = scopes.into_iter().map(|s| vec![s]).collect();
                    lv.push(own);
                    c.read_levels(&lv, &r.kind, &values, tau).0
                }
            };
            by_kind.get(&t.kind).into_iter().flatten().copied().zip(s).collect()
        })
        .collect();
    relate::score::tally(test, pool, by_kind, |i, v| table[i].get(&v).copied().unwrap_or(f64::MIN))
}

