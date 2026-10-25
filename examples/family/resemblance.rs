//! §§65–67, family resemblance, counted in Phi-4's answers. For the members
//! of a category: is there one relation they all share (a common essence),
//! or do they hang together by overlapping relations, like fibres in a
//! thread, none running the whole length?

use super::data::Table;
use std::collections::BTreeSet;

pub struct Family {
    /// Share of members holding the most widely held relation.
    pub longest_fibre: f64,
    pub mean_jaccard: f64,
    /// Share of members joined, through chains of shared relations, to the largest group.
    pub thread: f64,
}

pub fn family(t: &Table, members: &[String]) -> Family {
    let props: Vec<BTreeSet<String>> = members.iter().map(|m| t.properties(m)).collect();
    let mut all: BTreeSet<&String> = BTreeSet::new();
    props.iter().for_each(|p| all.extend(p.iter()));
    let n = members.len() as f64;
    let longest = all.iter().map(|f| props.iter().filter(|p| p.contains(*f)).count()).max().unwrap_or(0) as f64 / n;
    let (mut j, mut pairs) = (0.0f64, 0.0f64);
    let mut parent: Vec<usize> = (0..members.len()).collect();
    fn root(p: &mut Vec<usize>, i: usize) -> usize {
        if p[i] != i {
            let r = root(p, p[i]);
            p[i] = r;
        }
        p[i]
    }
    for a in 0..props.len() {
        for b in a + 1..props.len() {
            let inter = props[a].intersection(&props[b]).count() as f64;
            let uni = props[a].union(&props[b]).count() as f64;
            j += if uni > 0.0 { inter / uni } else { 0.0 };
            pairs += 1.0;
            if inter > 0.0 {
                let (ra, rb) = (root(&mut parent, a), root(&mut parent, b));
                parent[ra] = rb;
            }
        }
    }
    let mut size = vec![0usize; members.len()];
    for i in 0..members.len() {
        let r = root(&mut parent, i);
        size[r] += 1;
    }
    Family { longest_fibre: longest, mean_jaccard: j / pairs.max(1.0), thread: *size.iter().max().unwrap_or(&0) as f64 / n }
}
