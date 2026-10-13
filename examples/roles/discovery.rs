#[allow(unused_imports)]
use super::*;

/// Do relation types fall out of use, without being named?
///
/// `Role::ALL` is six variants someone chose and `extract` is a list of regexes
/// hardcoding the phrasings someone thought of. CLU's argument against exactly
/// that is why this exists: a type should be defined by what its instances share
/// under the operations, not by a label handed in from outside.
///
/// The relation benchmark has 305 pairs across 10 families whose labels the
/// clusterer never sees. If the discovered clusters line up with those labels
/// above chance, the types came from the data.
pub(super) fn discovery_experiment(facet: &Facet) {
    use phiano::roles::RoleDiscovery;

    let families = RelationBenchmark::default_families();
    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut labels: HashMap<(String, String), String> = HashMap::new();
    for f in &families {
        for p in &f.pairs {
            if facet.lexicon.contains_key(&p.a) && facet.lexicon.contains_key(&p.b) {
                pairs.push((p.a.clone(), p.b.clone()));
                labels.insert((p.a.clone(), p.b.clone()), f.name.clone());
            }
        }
    }

    println!(
        "\n=== discovered relation types ===\n  {} labelled pairs across {} families, labels withheld",
        pairs.len(),
        families.len()
    );
    println!("  {:>3} {:>9} {:>10}  verdict", "k", "purity", "vs chance");

    for k in [2usize, 5, 10, 15] {
        let clusters = RoleDiscovery::discover(facet, &pairs, k, 25);
        if clusters.is_empty() {
            continue;
        }
        let (purity, chance) = RoleDiscovery::purity(&clusters, &labels);
        println!(
            "  {:>3} {:>8.1}% {:>9.1}%  {}",
            k,
            purity * 100.0,
            chance * 100.0,
            match purity > chance * 1.25 {
                true => "types recovered from use",
                false => "no better than the majority label",
            }
        );

        if k == 10 {
            println!("\n  --- what the clusters contain, at k=10 ---");
            let mut sorted: Vec<_> = clusters.iter().filter(|c| c.members.len() >= 3).collect();
            sorted.sort_by_key(|c| std::cmp::Reverse(c.members.len()));
            for c in sorted.iter().take(6) {
                let mut counts: HashMap<&str, usize> = HashMap::new();
                for m in &c.members {
                    if let Some(l) = labels.get(m) {
                        *counts.entry(l.as_str()).or_insert(0) += 1;
                    }
                }
                let mut top: Vec<_> = counts.into_iter().collect();
                top.sort_by_key(|x| std::cmp::Reverse(x.1));
                let makeup: Vec<String> =
                    top.iter().take(3).map(|(l, n)| format!("{} {}", n, l)).collect();
                let example = c
                    .members
                    .first()
                    .map(|(a, b)| format!("{}→{}", a, b))
                    .unwrap_or_default();
                println!(
                    "    {:>3} pairs, coherence {:.2}  [{}]  e.g. {}",
                    c.members.len(),
                    c.coherence,
                    makeup.join(", "),
                    example
                );
            }
            println!();
        }
    }

    println!(
        "  Purity must be read against chance, which is the largest family's share:\n\
         \x20 a single cluster holding everything scores exactly chance. Only a gap\n\
         \x20 means the offsets carry relation identity."
    );
}
