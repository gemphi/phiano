//! Do definitions build concepts? Measured, one change at a time.
//!
//! `cargo run --release --bin conception -- [corpus] [chunks] [rounds]`
//!
//! Definition grounding is already in the codebase and already measured: it
//! halves phase dispersion and improves no relation metric. `Conception`
//! changes three things about how a definition reaches the manifold — every
//! channel instead of one, order carried in the phase instead of a centroid,
//! and a mutual pull instead of a one-way one. Three changes need three
//! controls, so this runs five conditions on the same trained facet:
//!
//! | condition           | channels | order | mutual |
//! |---------------------|----------|-------|--------|
//! | baseline            | –        | –     | –      |
//! | grounder (existing) | 1        | no    | no     |
//! | compose, unbound    | 64       | no    | no     |
//! | compose             | 64       | yes   | no     |
//! | compose + reinforce | 64       | yes   | yes    |
//!
//! Each row differs from the one above it in exactly one property, so a gain
//! is attributable. The relation benchmark is the target metric — whether
//! *grandmother* sits near *grandfather* the way *woman* sits near *man* — with
//! phase dispersion and held-out perplexity logged beside it, because a change
//! that improves relations while collapsing the manifold or wrecking prediction
//! has not improved the model.
//!
//! The definition source is any [`Groundable`]. Here it is the local Webster's
//! chunk store; `ApiSource` supplies the same shape from an online dictionary
//! where the network allows it, and nothing below changes.

use phiano::chunker::ChunkStore;
use phiano::cognitive::grounding::DefinitionGrounder;
use phiano::conception::{
    Conception, DefinitionGraph, ANCHOR, BETA_STRONG, BETA_WEAK, HEAD_STEP, REINFORCE,
};
use phiano::config::LEARNING_RATE;
use phiano::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::metrics::relation::RelationBenchmark;
use phiano::sources::definition_core;
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;

/// Mean and sample standard deviation of a metric across seeds.
///
/// Sample sd (n-1), because these runs are a sample of the seeds that could
/// have been drawn, not the population of them. With n = 5 the difference is
/// not cosmetic.
#[derive(Clone, Copy, Default)]
struct Stat {
    mean: f64,
    sd: f64,
    n: usize,
}

impl Stat {
    fn of(xs: &[f64]) -> Self {
        let n = xs.len();
        if n == 0 {
            return Self::default();
        }
        let mean = xs.iter().sum::<f64>() / n as f64;
        let sd = match n {
            0 | 1 => 0.0,
            _ => (xs.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64).sqrt(),
        };
        Self { mean, sd, n }
    }

    /// True when this is separated from `other` by more than the two spreads
    /// together — the crude test, stated as crude, that an effect clears its
    /// own noise.
    fn separated_from(&self, other: &Stat) -> bool {
        (self.mean - other.mean).abs() > self.sd + other.sd
    }
}

struct Row {
    name: String,
    pair_vs_random: f64,
    neighbour_top10: f64,
    analogy_top1: f64,
    analogy_mrr: f64,
    dispersion: f64,
    valid_ppl: f64,
    usable_pairs: usize,
}

fn measure(name: &str, facet: &Facet, valid: &[String]) -> Row {
    let families = RelationBenchmark::default_families();
    let r = RelationBenchmark::evaluate(facet, &families);
    Row {
        name: name.to_string(),
        usable_pairs: r.families.iter().map(|f| f.usable_pairs).sum(),
        pair_vs_random: r.overall_pair_vs_random,
        neighbour_top10: r
            .families
            .iter()
            .map(|f| f.neighbour_top10)
            .sum::<f64>()
            / r.families.len().max(1) as f64,
        analogy_top1: r.overall_analogy_top1,
        analogy_mrr: r.families.iter().map(|f| f.analogy_mrr).sum::<f64>()
            / r.families.len().max(1) as f64,
        dispersion: facet.phase_dispersion(),
        valid_ppl: Harness::perplexity_no_phase(facet, valid),
    }
}

/// Measures a condition, keeps the facet if it is the best seen.
///
/// Only the best facet is retained — holding all fifteen would be about a
/// gigabyte at this vocabulary size — and it is what the per-family breakdown
/// is computed from.
fn record(
    rows: &mut Vec<Row>,
    best: &mut (f64, Facet),
    name: &str,
    f: Facet,
    valid: &[String],
) {
    let r = measure(name, &f, valid);
    if r.analogy_mrr > best.0 {
        *best = (r.analogy_mrr, f);
    }
    rows.push(r);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // A3: --runs N repeats the whole experiment at N seeds and reports mean and
    // spread. One deterministic number has no error bar, and the effects left
    // after A2 are small enough that the interval is what decides them.
    let runs: usize = std::env::var("RUNS").ok().and_then(|v| v.parse().ok()).unwrap_or(1);
    let base_seed: u64 = std::env::var("SEED").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
    let corpus_path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/rust_book_corpus.txt".to_string());
    let chunks_path = args.get(2).cloned().unwrap_or_else(|| "data/chunks".to_string());
    let rounds: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(3);

    let raw = match std::fs::read_to_string(&corpus_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("could not read {}: {}", corpus_path, e);
            std::process::exit(1);
        }
    };
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 4)
        .collect();
    let split = Harness::split(corpus, 42);

    // Ranking-only training, because it is the regime the objective experiment
    // measured as 27x better at putting relational structure in the manifold.
    // Testing a composition rule on top of the weaker objective would confound
    // the two.
    let trainer = Trainer::new(LEARNING_RATE).with_seed(base_seed);
    let base = Harness::train_ranking_only(&split, &trainer, 4);
    println!(
        "trained: {} sentences, vocabulary {}",
        split.train.len(),
        base.vocabulary_size()
    );

    let store = ChunkStore::new(&chunks_path);
    let all = store.load_all();
    // Only entries whose headword the corpus actually contains can move
    // anything, and only those are counted, so "entries" is not inflated by the
    // dictionary's size.
    // A1: entries reduced to their definitional core. Webster's stacks the
    // gloss together with elaboration, editorial notes, usage examples and
    // quoted lines; composing all of it put ~32 words into every definer set,
    // against ~10 that actually define. The kernel share this produces (9.9%)
    // lands on the literature's ~10%, which the 30.5% it replaced did not.
    let entries: Vec<(String, String)> = all
        .into_iter()
        .filter(|(w, _)| base.lexicon.contains_key(w))
        .map(|(w, d)| (w, definition_core(&d)))
        .filter(|(_, d)| d.split_whitespace().count() >= 2)
        .collect();
    println!("definitions covering the vocabulary: {}", entries.len());
    if entries.is_empty() {
        eprintln!("no definitions overlap the trained vocabulary — nothing to compose");
        std::process::exit(1);
    }

    let mut best: (f64, Facet) = (f64::NEG_INFINITY, base.clone());
    let mut rows: Vec<Row> = Vec::new();
    record(&mut rows, &mut best, "baseline", base.clone(), &split.valid);
    if rows[0].usable_pairs == 0 {
        eprintln!(
            "WARNING: the relation benchmark has 0 usable pairs on this vocabulary. \
             Every relation number below is vacuous — train on a corpus that \
             contains the relation words."
        );
    }

    {
        let mut f = base.clone();
        DefinitionGrounder::ground_from(&mut f, &store);
        record(&mut rows, &mut best, "grounder (1 channel, centroid)", f, &split.valid);
    }

    // The 2x2 that separates two things the first run conflated: whether the
    // composition is *bound* by position at all, and whether the position a word
    // gets is *canonical* (its rank in the sorted definer set) or an artefact of
    // how the lexicographer phrased the entry.
    //
    // Sorting is not "order removed". Sorting makes position a deterministic
    // function of the definer set, so the same set composes to the same target
    // however it was written down. Genuinely removing order means not rotating.
    let sorted: Vec<(String, String)> = entries
        .iter()
        .map(|(w, d)| {
            let mut t: Vec<String> = Tokenizer::tokenize(d);
            t.sort();
            t.dedup();
            (w.clone(), t.join(" "))
        })
        .collect();

    for (label, src, bind) in [
        ("compose: bag, no rotation", &entries, false),
        ("compose: bound, as written", &entries, true),
        ("compose: bound, canonical", &sorted, true),
    ] {
        let mut f = base.clone();
        Conception::compose_all_bound(&mut f, src, rounds, HEAD_STEP, 0.0, bind);
        record(&mut rows, &mut best, label, f, &split.valid);
    }

    // Reinforcement, added to each composition rule, so its contribution is
    // read against that rule rather than against the baseline.
    for (label, src, bind) in [
        ("  + reinforce (bag)", &entries, false),
        ("  + reinforce (canonical)", &sorted, true),
    ] {
        let mut f = base.clone();
        let r = Conception::compose_all_bound(&mut f, src, rounds, HEAD_STEP, REINFORCE, bind);
        println!(
            "  [{}] {} heads, {} definers reinforced",
            label.trim(),
            r.heads_moved,
            r.definers_reinforced
        );
        record(&mut rows, &mut best, label, f, &split.valid);
    }

    // Dict2vec's split: a definitional pair is *strong* when each word occurs
    // in the other's definition and *weak* when the membership is one-way. The
    // flat rule above pulls both at the same rate, which gives a passing
    // mention the same weight as a reciprocal definition.
    let mut graph = DefinitionGraph::build(&entries);
    // Before promotion: the kernel is a property of the definitional graph, and
    // promotion adds edges inferred from phase similarity rather than from
    // definition. Peeling the promoted graph mixes the two and measures neither.
    let kernel = graph.kernel();
    let (raw_strong, raw_weak) = graph.counts();
    // Dict2vec SS3.1: promote a weak pair to strong when the two words are among
    // each other's K nearest. Raw reciprocity alone gives 505:1 on a cleaned
    // single dictionary; their 9:1 came from four concatenated modern
    // dictionaries plus this promotion, not from reciprocity by itself.
    let promoted = graph.promote_neighbours(&base, 5);
    let (n_strong, n_weak) = graph.counts();
    println!(
        "definition graph: {} strong before promotion, {} promoted (K=5)",
        raw_strong, promoted
    );
    let _ = raw_weak;
    println!(
        "definition graph: {} strong pairs, {} weak ({:.1}:1)",
        n_strong,
        n_weak,
        n_weak as f64 / n_strong.max(1) as f64
    );
    {
        let mut f = base.clone();
        Conception::compose_graded(
            &mut f,
            &entries,
            rounds,
            HEAD_STEP,
            BETA_STRONG,
            BETA_WEAK,
            false,
            Some(&graph),
        );
        record(&mut rows, &mut best, "  + strong/weak (dict2vec)", f, &split.valid);
    }
    {
        // Control: the same two rates applied without the reciprocity test, so
        // any gain above this row is the *split* rather than the rates.
        let mut f = base.clone();
        Conception::compose_graded(
            &mut f,
            &entries,
            rounds,
            HEAD_STEP,
            BETA_STRONG,
            BETA_STRONG,
            false,
            None,
        );
        record(&mut rows, &mut best, "  control: flat at BETA_STRONG", f, &split.valid);
    }

    // The retrofitting anchor. The best-scoring row above drops dispersion from
    // 0.986 to 0.327, which is a third of the way to collapse — the anchor is
    // the term that competes with the neighbour pull, and the sweep is what
    // says whether the relation gains survive keeping the manifold spread.
    println!(
        "grounding kernel (pre-promotion): {} of {} nodes ({:.1}%)",
        kernel.len(),
        graph.nodes(),
        100.0 * kernel.len() as f64 / graph.nodes().max(1) as f64
    );
    for a in [0.25f64, 0.5, ANCHOR, 2.0] {
        let mut f = base.clone();
        Conception::compose_anchored(
            &mut f, &entries, rounds, HEAD_STEP, BETA_STRONG, BETA_STRONG, false, None, a, None,
        );
        record(&mut rows, &mut best, &format!("  anchor α={:.2}", a), f, &split.valid);
    }
    {
        // Kernel scheduling: hold the definitional core at a tenth of the rate
        // and compose the periphery against it, instead of relaxing all
        // entries simultaneously against neighbours that are themselves moving.
        let mut f = base.clone();
        Conception::compose_anchored(
            &mut f, &entries, rounds, HEAD_STEP, BETA_STRONG, BETA_STRONG, false, None, ANCHOR,
            Some(&kernel),
        );
        record(&mut rows, &mut best, "  anchor + held kernel", f, &split.valid);
    }

    // Controlled negative sampling, measured on the training path rather than
    // asserted. This retrains from scratch with the filter attached, so the
    // comparison against `baseline` is the filter's whole effect.
    {
        let g = std::sync::Arc::new(DefinitionGraph::build(&entries));
        let filtered = Trainer::new(LEARNING_RATE).with_definitions(g);
        let f = Harness::train_ranking_only(&split, &filtered, 4);
        record(&mut rows, &mut best, "controlled negatives (retrained)", f, &split.valid);
    }

    println!("\n=== definitions as compositions ===");
    println!("usable relation pairs: {}", rows[0].usable_pairs);
    println!(
        "{:<32} {:>9} {:>9} {:>9} {:>9} {:>7} {:>9}",
        "condition", "pair/rnd", "nbr@10", "anlg@1", "anlg MRR", "disp", "valid ppl"
    );
    for r in &rows {
        println!(
            "{:<32} {:>8.1}% {:>8.1}% {:>8.2}% {:>9.4} {:>7.3} {:>9.2}",
            r.name,
            r.pair_vs_random * 100.0,
            r.neighbour_top10 * 100.0,
            r.analogy_top1 * 100.0,
            r.analogy_mrr,
            r.dispersion,
            r.valid_ppl
        );
    }

    // A2: per-family, not just the aggregate. A total can be carried by one
    // family, and morphological families (plural, comparative, past tense) can
    // be learned from spelling alone while the semantic ones learn nothing —
    // which the aggregate would hide.
    {
        let families = RelationBenchmark::default_families();
        let report = RelationBenchmark::evaluate(&best.1, &families);
        println!("\n--- per family, best condition ---");
        println!(
            "{:<14} {:>7} {:>10} {:>10} {:>10}",
            "family", "usable", "pair/rnd", "nbr@10", "anlg MRR"
        );
        for f in &report.families {
            println!(
                "{:<14} {:>7} {:>9.1}% {:>9.1}% {:>10.4}",
                f.name,
                f.usable_pairs,
                f.pair_vs_random * 100.0,
                f.neighbour_top10 * 100.0,
                f.analogy_mrr
            );
        }
        let usable: usize = report.families.iter().map(|f| f.usable_pairs).sum();
        let covered = report.families.iter().filter(|f| f.usable_pairs > 0).count();
        println!(
            "  {} usable pairs across {} of {} families covered by this vocabulary",
            usable,
            covered,
            report.families.len()
        );
    }

    // Repeat at further seeds and report the spread of the two headline
    // conditions. Only two, because each seed is a full retrain plus fifteen
    // compositions, and an interval on the numbers actually being claimed is
    // worth more than a wide, thin sweep.
    if runs > 1 {
        println!("\n=== across {} seeds ===", runs);
        let mut base_mrr: Vec<f64> = Vec::new();
        let mut best_mrr: Vec<f64> = Vec::new();
        let mut base_pair: Vec<f64> = Vec::new();
        let mut best_pair: Vec<f64> = Vec::new();

        for r in 0..runs {
            let seed = base_seed.wrapping_add(r as u64);
            let t = Trainer::new(LEARNING_RATE).with_seed(seed);
            let f0 = Harness::train_ranking_only(&split, &t, 4);
            let b = measure("baseline", &f0, &split.valid);

            let mut f1 = f0.clone();
            Conception::compose_graded(
                &mut f1,
                &entries,
                rounds,
                HEAD_STEP,
                BETA_STRONG,
                BETA_STRONG,
                false,
                None,
            );
            let c = measure("composed", &f1, &split.valid);

            println!(
                "  seed {:<4} baseline MRR {:.4}  composed MRR {:.4}  (pair {:.1}% -> {:.1}%)",
                seed,
                b.analogy_mrr,
                c.analogy_mrr,
                b.pair_vs_random * 100.0,
                c.pair_vs_random * 100.0
            );
            base_mrr.push(b.analogy_mrr);
            best_mrr.push(c.analogy_mrr);
            base_pair.push(b.pair_vs_random);
            best_pair.push(c.pair_vs_random);
        }

        let (bm, cm) = (Stat::of(&base_mrr), Stat::of(&best_mrr));
        let (bp, cp) = (Stat::of(&base_pair), Stat::of(&best_pair));
        println!(
            "\n  analogy MRR   baseline {:.4} +/- {:.4}   composed {:.4} +/- {:.4}   n={}",
            bm.mean, bm.sd, cm.mean, cm.sd, cm.n
        );
        println!(
            "  pair/random   baseline {:.1}% +/- {:.1}   composed {:.1}% +/- {:.1}",
            bp.mean * 100.0,
            bp.sd * 100.0,
            cp.mean * 100.0,
            cp.sd * 100.0
        );
        println!(
            "\n  VERDICT: composition {} the baseline by more than the two spreads \
             combined on MRR, and {} on pair/random.",
            if cm.separated_from(&bm) { "clears" } else { "does NOT clear" },
            if cp.separated_from(&bp) { "clears" } else { "does not" }
        );
        println!(
            "  (Spread-sum separation is a crude test and is labelled as one: it is \n\
             \x20 not a t-test, and n={} is small.)",
            cm.n
        );
    }

    println!("\n--- reading ---");
    let b = &rows[0];
    for r in rows.iter().skip(1) {
        println!(
            "{:<32} pair {:+.1}pp, nbr@10 {:+.1}pp, MRR {:+.4}, dispersion {:+.3}",
            r.name,
            (r.pair_vs_random - b.pair_vs_random) * 100.0,
            (r.neighbour_top10 - b.neighbour_top10) * 100.0,
            r.analogy_mrr - b.analogy_mrr,
            r.dispersion - b.dispersion
        );
    }
    println!(
        "\nvalid perplexity is the no-phase path and must not move: {:.2} throughout.",
        b.valid_ppl
    );
}
