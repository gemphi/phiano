//! Does role binding beat the bag on real data?
//!
//! `cargo run --release --bin roles -- [chunks] [corpus]`
//!
//! Positional binding failed three times (definitions, LM context, sentence
//! composition), and the diagnosis was that position is a **nonce** role: the
//! same concept lands at a different angle in every context, so superposed
//! copies cancel. A role rotation is shared — every `genus` binds at the same
//! angle everywhere — so the relation becomes a direction rather than noise.
//!
//! That is a claim about recoverability, and this measures it on the dictionary
//! rather than on a fixture.
//!
//! # The task
//!
//! Take the relation benchmark's **hypernym** family — `(dog, animal)`,
//! `(oak, tree)`, `(salmon, fish)` — which is exactly the genus relation.
//! Compose each specific term from the rules extracted from its own Webster's
//! gloss, then ask the composition what its genus is. Correct means the true
//! hypernym comes back.
//!
//! Two scorers on identical pairs:
//!
//! * **role query** — `identity ⊖ genus`, then nearest word.
//! * **bag** — the same fillers superposed with *no* role rotation, then
//!   nearest word. The control: everything role binding has, minus the roles.
//!
//! If they tie, the rotation is decoration. Only a gap is evidence.

use phiano::chunker::ChunkStore;
use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiano::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::metrics::relation::RelationBenchmark;
use phiano::phasor::SpectralPhasor;
use phiano::roles::{Role, RuleSet, Roles};
use phiano::sources::definition_core;
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;
use std::collections::HashMap;

/// Function words. A gloss opens with them constantly and none is ever a genus.
///
/// The first version of this extractor filtered adjectives but not these, and
/// then picked the candidate with the highest definitional in-degree on the
/// theory that genus terms are general. Frequency-as-generality is exactly
/// backwards once function words are in the pool: *the* has the highest
/// in-degree in any dictionary, so every entry came back with
/// `genus(dog, the)`. The role-versus-bag comparison still ran, and still
/// showed a gap — on garbage input, which makes the number worthless.
fn is_function_word(w: &str) -> bool {
    matches!(
        w,
        "a" | "an" | "the" | "of" | "or" | "and" | "to" | "in" | "is" | "are" | "as" | "that"
            | "which" | "with" | "for" | "by" | "on" | "at" | "from" | "it" | "its" | "any"
            | "one" | "used" | "being" | "having" | "also" | "was" | "were" | "be" | "been"
            | "this" | "these" | "those" | "not" | "but" | "so" | "than" | "then" | "into"
            | "upon" | "over" | "under" | "out" | "up" | "down" | "off" | "who" | "whom"
            | "what" | "when" | "where" | "while" | "all" | "some" | "each" | "every"
            | "more" | "most" | "less" | "least" | "very" | "much" | "may" | "can" | "will"
            | "would" | "should" | "must" | "has" | "have" | "had" | "does" | "did" | "do"
    )
}

/// Adjectives and quantifiers that open a gloss without being its genus.
fn is_modifier(w: &str) -> bool {
    matches!(
        w,
        "small" | "large" | "great" | "little" | "young" | "old" | "common" | "certain"
            | "various" | "many" | "several" | "other" | "same" | "such" | "first" | "last"
            | "high" | "low" | "long" | "short" | "new" | "good" | "bad" | "hard" | "soft"
            | "domestic" | "wild" | "male" | "female" | "single" | "double" | "whole"
    )
}

/// Extracts constitutive rules from a Webster's gloss.
///
/// The genus is the load-bearing one, and Webster's makes it recoverable
/// because its entries are genus-differentia: *cat: **an animal** of various
/// species*, *car: a small **vehicle** moved on wheels*. The kind comes first,
/// after the article and any adjectives.
///
/// So the rule is simply the **first surviving content word**: drop function
/// words, drop modifiers, take what is left. Frequency is deliberately not used
/// to rank candidates — see [`is_function_word`] for what that cost.
///
/// `in_degree` is still consulted, but only as a *ceiling*: a candidate used in
/// more than a fifth of all glosses is a function word this list has missed,
/// and taking it as a genus would place half the dictionary under one node.
fn extract(
    gloss: &str,
    in_degree: &HashMap<String, usize>,
    total_glosses: usize,
    head: &str,
) -> Vec<(Role, String)> {
    let toks: Vec<String> = Tokenizer::tokenize(gloss);
    let mut out = Vec::new();
    let ceiling = (total_glosses / 5).max(1);

    // ---- genus: first content word ----
    let genus = toks
        .iter()
        .take(6)
        .find(|w| {
            w.len() > 2
                && !is_function_word(w)
                && !is_modifier(w)
                && *w != head
                && in_degree.get(*w).copied().unwrap_or(0) < ceiling
        })
        .cloned();
    if let Some(g) = genus {
        out.push((Role::Genus, g));
    }

    // ---- function: "used to X", "used for X", "enables X", "serves to X" ----
    for (i, w) in toks.iter().enumerate() {
        let trigger = matches!(w.as_str(), "used" | "serves" | "enables" | "employed");
        if !trigger {
            continue;
        }
        if let Some(f) = toks
            .iter()
            .skip(i + 1)
            .take(4)
            .find(|x| x.len() > 3 && !is_modifier(x) && !is_function_word(x))
        {
            out.push((Role::Function, f.clone()));
            break;
        }
    }

    // ---- form: "made of X", "consisting of X", "composed of X" ----
    for (i, w) in toks.iter().enumerate() {
        if !matches!(w.as_str(), "made" | "consisting" | "composed" | "formed") {
            continue;
        }
        if let Some(f) = toks
            .iter()
            .skip(i + 1)
            .take(4)
            .find(|x| x.len() > 2 && !is_modifier(x) && !is_function_word(x))
        {
            out.push((Role::Form, f.clone()));
            break;
        }
    }

    out
}

/// Bag control: the same fillers, superposed with no role rotation.
fn bag_identity(facet: &Facet, fillers: &[String]) -> Option<SpectralPhasor> {
    let mut acc = vec![(0.0f64, 0.0f64); PHASE_CHANNELS];
    let mut used = 0usize;
    for f in fillers {
        let p = match facet.lexicon.get(f) {
            Some(p) => p,
            None => continue,
        };
        used += 1;
        for (k, a) in acc.iter_mut().enumerate() {
            let t = p.theta(k);
            a.0 += t.cos();
            a.1 += t.sin();
        }
    }
    if used == 0 {
        return None;
    }
    let mut out = SpectralPhasor::seeded("bag", 1.0, 1);
    for (k, (x, y)) in acc.iter().enumerate() {
        if x.hypot(*y) > 1e-12 {
            out.set_theta(k, y.atan2(*x));
        }
    }
    out.sync_phase();
    Some(out)
}

fn nearest(facet: &Facet, q: &SpectralPhasor, exclude: &[&str], k: usize) -> Vec<String> {
    let mut scored: Vec<(&str, f64)> = facet
        .lexicon
        .iter()
        .filter(|(w, _)| !exclude.contains(&w.as_str()))
        .map(|(w, p)| (w.as_str(), q.resonance(p)))
        .collect();
    scored.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.0.cmp(b.0))
    });
    scored.into_iter().take(k).map(|(w, _)| w.to_string()).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let chunks = args.get(1).cloned().unwrap_or_else(|| "data/chunks".to_string());
    let corpus_path = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| "data/dictionary_corpus.txt".to_string());

    let raw = std::fs::read_to_string(&corpus_path).unwrap_or_default();
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 4)
        .collect();
    if corpus.is_empty() {
        eprintln!("empty corpus at {}", corpus_path);
        std::process::exit(1);
    }
    let split = Harness::split(corpus, 42);
    let facet = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    println!("facet: {} words", facet.vocabulary_size());

    // Glosses, reduced to their definitional core.
    let glosses: Vec<(String, String)> = ChunkStore::new(&chunks)
        .load_all()
        .into_iter()
        .map(|(w, d)| (w, definition_core(&d)))
        .filter(|(_, d)| d.split_whitespace().count() >= 2)
        .collect();

    // Definitional in-degree: how many entries use this word to define
    // something. General terms score high, which is what makes the genus
    // recoverable without a parser.
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for (_, g) in &glosses {
        for t in Tokenizer::tokenize(g) {
            *in_degree.entry(t).or_insert(0) += 1;
        }
    }

    let mut rules = RuleSet::new();
    let mut fillers_of: HashMap<String, Vec<String>> = HashMap::new();
    for (head, gloss) in &glosses {
        for (role, filler) in extract(gloss, &in_degree, glosses.len(), head) {
            rules.add(head, role, &filler);
            fillers_of.entry(head.clone()).or_default().push(filler);
        }
    }
    println!("extracted {} rules from {} glosses", rules.len(), glosses.len());
    for (r, n) in rules.role_counts() {
        println!("  {:<10} {}", r.name(), n);
    }

    // The hypernym family is the genus relation, so it is the family this can
    // be scored on without inventing a probe set.
    let families = RelationBenchmark::default_families();
    let hyper = families
        .iter()
        .find(|f| f.name == "hypernym")
        .expect("hypernym family present");

    let mut usable = 0usize;
    let (mut role_top1, mut role_top5) = (0usize, 0usize);
    let (mut bag_top1, mut bag_top5) = (0usize, 0usize);
    let mut examples: Vec<String> = Vec::new();

    for pair in &hyper.pairs {
        let (specific, general) = (&pair.a, &pair.b);
        if !facet.lexicon.contains_key(specific) || !facet.lexicon.contains_key(general) {
            continue;
        }
        let fillers = match fillers_of.get(specific) {
            Some(f) if !f.is_empty() => f.clone(),
            _ => continue,
        };
        usable += 1;

        let id = match Roles::identity(&facet, &rules, specific) {
            Some(i) => i,
            None => continue,
        };
        let by_role: Vec<String> = Roles::query(&facet, &id, Role::Genus, &[specific], 5)
            .into_iter()
            .map(|(w, _)| w)
            .collect();

        let bag = match bag_identity(&facet, &fillers) {
            Some(b) => b,
            None => continue,
        };
        let by_bag = nearest(&facet, &bag, &[specific], 5);

        if by_role.first().is_some_and(|w| w == general) {
            role_top1 += 1;
        }
        if by_role.iter().any(|w| w == general) {
            role_top5 += 1;
        }
        if by_bag.first().is_some_and(|w| w == general) {
            bag_top1 += 1;
        }
        if by_bag.iter().any(|w| w == general) {
            bag_top5 += 1;
        }

        if examples.len() < 8 {
            examples.push(format!(
                "  {:<10} → genus? role: {:<28} bag: {:<28} (true: {})",
                specific,
                by_role.iter().take(3).cloned().collect::<Vec<_>>().join(", "),
                by_bag.iter().take(3).cloned().collect::<Vec<_>>().join(", "),
                general
            ));
        }
    }

    let n = usable.max(1) as f64;
    println!("\n=== genus recovery on {} hypernym pairs ===", usable);
    println!("  {:<14} {:>9} {:>9}", "scorer", "top-1", "top-5");
    println!(
        "  {:<14} {:>8.1}% {:>8.1}%",
        "role query",
        role_top1 as f64 / n * 100.0,
        role_top5 as f64 / n * 100.0
    );
    println!(
        "  {:<14} {:>8.1}% {:>8.1}%",
        "bag (control)",
        bag_top1 as f64 / n * 100.0,
        bag_top5 as f64 / n * 100.0
    );
    println!(
        "  {:<14} {:>8.4}% {:>8.4}%",
        "chance",
        100.0 / facet.vocabulary_size() as f64,
        500.0 / facet.vocabulary_size() as f64
    );

    println!("\n--- what came back ---");
    for e in &examples {
        println!("{}", e);
    }

    println!(
        "\n  VERDICT: role binding {} the bag on top-5 ({:+.1} pp).",
        match role_top5 > bag_top5 {
            true => "BEATS",
            false => "does NOT beat",
        },
        (role_top5 as f64 - bag_top5 as f64) / n * 100.0
    );
    println!(
        "  A tie would mean the rotation is decoration: the bag has the same\n\
         \x20 fillers and only lacks the roles, so only a gap is evidence that\n\
         \x20 typing the relation carries information."
    );

    // The CLU question: can the types be discovered rather than declared?
    discovery_experiment(&facet);

    // And the blocker discovery exposed: can coherence be trained?
    coherence_experiment(&facet, &rules);

    // The unblocking attempt: let the language name the relations.
    lexical_experiment(&facet, &glosses);

    // Hierarchy, as a demonstration rather than a metric.
    println!("\n--- genus chains ---");
    for w in ["cow", "dog", "oak", "money", "salmon"] {
        let path = Roles::ascend(&rules, w, Role::Genus, 5);
        if path.len() > 1 {
            println!("  {}", path.join(" → "));
        }
    }
}

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
fn discovery_experiment(facet: &Facet) {
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
    println!("  {:>3} {:>9} {:>10}  {}", "k", "purity", "vs chance", "verdict");

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
            sorted.sort_by(|a, b| b.members.len().cmp(&a.members.len()));
            for c in sorted.iter().take(6) {
                let mut counts: HashMap<&str, usize> = HashMap::new();
                for m in &c.members {
                    if let Some(l) = labels.get(m) {
                        *counts.entry(l.as_str()).or_insert(0) += 1;
                    }
                }
                let mut top: Vec<_> = counts.into_iter().collect();
                top.sort_by(|a, b| b.1.cmp(&a.1));
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
fn coherence_experiment(facet: &Facet, rules: &RuleSet) {
    use phiano::roles::{Coherence, RoleDiscovery};

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


/// Do prepositions give the multi-relation structure the mechanism needs?
///
/// The enum extractor averaged 1.06 relations per head, 94% of them one
/// over-broad `genus` bucket, and both role binding (§8) and coherence training
/// (§10b) came out flat as a result. English marks its relation types with
/// prepositions, so this asks the language for the inventory instead of
/// declaring one.
///
/// Two numbers decide it. **Relations per head** — the mechanism needs several
/// per word or it has nothing to disambiguate. And **per-role coherence** — if
/// `for` pairs agree on an offset better than the mixed `genus` bucket did, the
/// prepositions are naming real relations rather than slicing noise.
fn lexical_experiment(facet: &Facet, glosses: &[(String, String)]) {
    use phiano::roles::{Coherence, LexicalRules};

    let known: std::collections::HashSet<&str> =
        facet.lexicon.keys().map(|s| s.as_str()).collect();
    let is_content = |w: &str| {
        w.len() > 2 && !is_function_word(w) && !is_modifier(w) && known.contains(w)
    };

    let mut lex = LexicalRules::new();
    for (head, gloss) in glosses {
        if facet.lexicon.contains_key(head) {
            lex.extract(head, gloss, &is_content);
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
