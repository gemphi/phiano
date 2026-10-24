#[allow(unused_imports)]
use super::*;

/// Do prepositions give the multi-relation structure the mechanism needs?
///
/// The enum extractor averaged 1.06 relations per head, 94% of them one
/// over-broad `genus` bucket, and both role binding (§8) and coherence training
/// (§10b) came out flat as a result. English marks its relation types with
/// prepositions, so this asks the language for the inventory instead of
/// declaring one.
///
/// Two numbers decide it. **Register per head** — the mechanism needs several
/// per word or it has nothing to disambiguate. And **per-role coherence** — if
/// `for` pairs agree on an offset better than the mixed `genus` bucket did, the
/// prepositions are naming real relations rather than slicing noise.
pub(super) fn lexical_experiment(facet: &Facet, glosses: &[(String, String)]) {
    use phiano::roles::{Coherence, LexicalRules};

    let known: std::collections::HashSet<&str> =
        facet.lexicon.keys().map(|s| s.as_str()).collect();
    let is_content = |w: &str| {
        w.len() > 2 && !is_function_word(w) && !is_modifier(w) && known.contains(w)
    };

    let mut lex = LexicalRules::new();
    for (head, gloss) in glosses {
        if facet.lexicon.contains_key(head) {
            lex.extract(head, gloss, is_content);
        }
    }

    println!("\n=== relations named by the language ===");
    println!(
        "  {} triples, {:.2} per head, {} heads with 3 or more",
        lex.len(),
        lex.mean_per_head(),
        lex.heads_with_at_least(3)
    );
    println!(
        "  (the enum extractor: 1.06 per head, 94% in one genus bucket)\n"
    );

    // The control that decides it: the same number of pairs, drawn at random
    // from the same vocabulary. The circular mean of N random offsets has
    // agreement about 1/sqrt(N) by chance alone, so a small coherence on a small
    // group means nothing without this column beside it.
    let vocab: Vec<&String> = facet.lexicon.keys().collect();
    let shuffled_coherence = |n: usize, salt: u64| -> f64 {
        let mut r = 0x9E3779B97F4A7C15u64 ^ salt;
        let mut pairs = Vec::with_capacity(n);
        for _ in 0..n {
            r ^= r << 13;
            r ^= r >> 7;
            r ^= r << 17;
            let a = vocab[(r % vocab.len() as u64) as usize].clone();
            r ^= r << 13;
            r ^= r >> 7;
            r ^= r << 17;
            let b = vocab[(r % vocab.len() as u64) as usize].clone();
            if a != b {
                pairs.push((a, b));
            }
        }
        Coherence::measure(facet, &pairs)
    };

    println!(
        "  {:<10} {:>9} {:>11} {:>11} {:>8}",
        "role", "pairs", "coherence", "shuffled", "ratio"
    );
    let mut total_pairs = 0usize;
    let mut weighted = 0.0f64;
    let mut any_real = false;
    for (i, (role, n)) in lex.roles().into_iter().take(12).enumerate() {
        let pairs = lex.pairs_for_role(&role);
        let sample: Vec<(String, String)> = pairs.into_iter().take(4000).collect();
        let coh = Coherence::measure(facet, &sample);
        let null = shuffled_coherence(sample.len(), i as u64);
        let ratio = coh / null.max(1e-9);
        if ratio > 1.5 {
            any_real = true;
        }
        total_pairs += n;
        weighted += coh * n as f64;
        println!(
            "  {:<10} {:>9} {:>11.3} {:>11.3} {:>8.2}x",
            role, n, coh, null, ratio
        );
    }
    let mean = weighted / total_pairs.max(1) as f64;
    println!(
        "\n  VERDICT: {}",
        match any_real {
            true => "at least one preposition beats its shuffled control — the \
                     relation is in the manifold",
            false => "no preposition beats its shuffled control. Coherence at \
                      this scale is what random pairs give, so the manifold does \
                      not hold these relations at all.",
        }
    );

    println!(
        "\n  weighted mean coherence {:.3}, against 0.026 for the single genus\n\
         \x20 bucket the enum extractor produced.",
        mean
    );
    println!(
        "  Coherence here is over UNTRAINED phases: it measures whether the\n\
         \x20 relation exists in the data, not whether the model has learned it.\n\
         \x20 Near zero across every role would mean prepositions do not name\n\
         \x20 relations the manifold can hold, and no amount of training fixes that."
    );

    // Now train on these groups and check the held-out numbers that stayed flat
    // last time.
    let groups: Vec<Vec<(String, String)>> = lex
        .roles()
        .into_iter()
        .take(12)
        .map(|(r, _)| lex.pairs_for_role(&r).into_iter().take(4000).collect())
        .filter(|g: &Vec<(String, String)>| g.len() >= 2)
        .collect();

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

    use phiano::roles::RoleDiscovery;
    let base = RoleDiscovery::discover(facet, &probe, 10, 25);
    let (base_purity, chance) = RoleDiscovery::purity(&base, &labels);
    let base_coh: f64 =
        base.iter().map(|c| c.coherence).sum::<f64>() / base.len().max(1) as f64;

    println!("\n  --- training on prepositional groups ---");
    println!(
        "  {:<22} {:>10} {:>12} {:>9} {:>11} {:>7}",
        "condition", "train coh", "cluster coh", "purity", "dispersion", "kept"
    );
    println!(
        "  {:<22} {:>10.3} {:>12.3} {:>8.1}% {:>11.3} {:>7}",
        "baseline",
        mean,
        base_coh,
        base_purity * 100.0,
        facet.phase_dispersion(),
        "-"
    );

    for (rate, rounds) in [(0.3f64, 3usize), (0.6, 10)] {
        let mut f = facet.clone();
        let (coh, kept) = Coherence::align_groups(&mut f, &groups, rate, rounds, 0.40);
        let cl = RoleDiscovery::discover(&f, &probe, 10, 25);
        let (purity, _) = RoleDiscovery::purity(&cl, &labels);
        let ccoh: f64 = cl.iter().map(|c| c.coherence).sum::<f64>() / cl.len().max(1) as f64;
        println!(
            "  {:<22} {:>10.3} {:>12.3} {:>8.1}% {:>11.3} {:>7}",
            format!("rate {:.1}, {} rounds", rate, rounds),
            coh,
            ccoh,
            purity * 100.0,
            f.phase_dispersion(),
            match kept { true => "yes", false => "REJECTED" }
        );
    }
    println!("  chance purity {:.1}%.", chance * 100.0);
}
