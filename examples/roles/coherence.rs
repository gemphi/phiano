#[allow(unused_imports)]
use super::*;

/// Can offset coherence be trained, and does it survive the collapse guard?
///
/// Discovery found relation types at 1.7x chance but with cluster coherence of
/// 0.27, against >0.9 on relations planted by construction. Nothing in the
/// objective optimises that. This asks whether it can be optimised at all, and
/// what it costs.
///
/// The alignment groups are the extracted relations — all genus pairs form one
/// group, all form pairs another — so the supervision comes from the dictionary
/// rather than from the benchmark. The benchmark families are then used only to
/// score, and their labels never enter the update.
pub(super) fn coherence_experiment(facet: &Facet, rules: &RuleSet) {
    use phient::train::roles::{Coherence, RoleDiscovery};

    // Groups from the extractor: one per role.
    let mut groups: Vec<Vec<(String, String)>> = Vec::new();
    for role in Role::ALL {
        let g: Vec<(String, String)> = rules
            .all()
            .iter()
            .filter(|r| r.role == role)
            .filter(|r| facet.lexicon.contains_key(&r.head) && facet.lexicon.contains_key(&r.filler))
            .map(|r| (r.head.clone(), r.filler.clone()))
            .take(4000)
            .collect();
        if g.len() >= 2 {
            groups.push(g);
        }
    }
    if groups.is_empty() {
        println!("\n=== coherence training ===\n  no usable relation groups");
        return;
    }

    // Held-out scoring: the benchmark families, never used in the update.
    let families = RelationBenchmark::default_families();
    let mut probe: Vec<(String, String)> = Vec::new();
    let mut labels: HashMap<(String, String), String> = HashMap::new();
    for f in &families {
        for p in &f.pairs {
            if facet.lexicon.contains_key(&p.a) && facet.lexicon.contains_key(&p.b) {
                probe.push((p.a.clone(), p.b.clone()));
                labels.insert((p.a.clone(), p.b.clone()), f.name.clone());
            }
        }
    }

    println!("\n=== coherence training ===");
    println!(
        "  {} groups from the extractor ({} pairs), scored on {} held-out benchmark pairs",
        groups.len(),
        groups.iter().map(|g| g.len()).sum::<usize>(),
        probe.len()
    );

    let base_coh: f64 = groups.iter().map(|g| Coherence::measure(facet, g)).sum::<f64>()
        / groups.len() as f64;
    let base_clusters = RoleDiscovery::discover(facet, &probe, 10, 25);
    let (base_purity, chance) = RoleDiscovery::purity(&base_clusters, &labels);
    let base_cluster_coh: f64 = base_clusters.iter().map(|c| c.coherence).sum::<f64>()
        / base_clusters.len().max(1) as f64;

    println!(
        "\n  {:<24} {:>10} {:>12} {:>10} {:>10} {:>7}",
        "condition", "train coh", "cluster coh", "purity", "dispersion", "kept"
    );
    println!(
        "  {:<24} {:>10.3} {:>12.3} {:>9.1}% {:>10.3} {:>7}",
        "baseline",
        base_coh,
        base_cluster_coh,
        base_purity * 100.0,
        facet.phase_dispersion(),
        "-"
    );

    for (rate, rounds) in [(0.1f64, 3usize), (0.3, 3), (0.3, 10), (0.6, 10)] {
        let mut f = facet.clone();
        let (coh, kept) = Coherence::align_groups(&mut f, &groups, rate, rounds, 0.40);
        let clusters = RoleDiscovery::discover(&f, &probe, 10, 25);
        let (purity, _) = RoleDiscovery::purity(&clusters, &labels);
        let cluster_coh: f64 = clusters.iter().map(|c| c.coherence).sum::<f64>()
            / clusters.len().max(1) as f64;

        println!(
            "  {:<24} {:>10.3} {:>12.3} {:>9.1}% {:>10.3} {:>7}",
            format!("rate {:.1}, {} rounds", rate, rounds),
            coh,
            cluster_coh,
            purity * 100.0,
            f.phase_dispersion(),
            match kept {
                true => "yes",
                false => "REJECTED",
            }
        );
    }

    println!(
        "\n  chance purity is {:.1}%. Training coherence is the objective; cluster\n\
         \x20 coherence and purity are held out, so a rise in the first without a rise\n\
         \x20 in the other two would mean the objective is fitting its own groups and\n\
         \x20 nothing else. A REJECTED row is the guard firing: coherence is perfect on\n\
         \x20 a collapsed manifold, so this objective has a degenerate optimum and the\n\
         \x20 dispersion floor is what stands between it and them.",
        chance * 100.0
    );
}
