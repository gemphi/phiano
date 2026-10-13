#[allow(unused_imports)]
use super::*;

/// Every measured condition, in order, plus the best-scoring facet kept for the
/// per-family breakdown.
pub(super) struct Outcome {
    pub(super) rows: Vec<Row>,
    pub(super) best: (f64, Facet),
}

/// Run the whole ladder of conditions on the baseline facet, one property
/// changed at a time, recording a row for each.
pub(super) fn run_conditions(s: &Setup) -> Outcome {
    let (base, entries, rounds, valid) = (&s.base, &s.entries, s.rounds, &s.split.valid);
    let mut best: (f64, Facet) = (f64::NEG_INFINITY, base.clone());
    let mut rows: Vec<Row> = Vec::new();
    record(&mut rows, &mut best, "baseline", base.clone(), valid);
    if rows[0].usable_pairs == 0 {
        eprintln!(
            "WARNING: the relation benchmark has 0 usable pairs on this vocabulary. \
             Every relation number below is vacuous — train on a corpus that \
             contains the relation words."
        );
    }

    {
        let mut f = base.clone();
        DefinitionGrounder::ground_from(&mut f, &s.store);
        record(&mut rows, &mut best, "grounder (1 channel, centroid)", f, valid);
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
        ("compose: bag, no rotation", entries, false),
        ("compose: bound, as written", entries, true),
        ("compose: bound, canonical", &sorted, true),
    ] {
        let mut f = base.clone();
        Conception::compose_all_bound(&mut f, src, rounds, HEAD_STEP, 0.0, bind);
        record(&mut rows, &mut best, label, f, valid);
    }

    // Reinforcement, added to each composition rule, so its contribution is
    // read against that rule rather than against the baseline.
    for (label, src, bind) in [
        ("  + reinforce (bag)", entries, false),
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
        record(&mut rows, &mut best, label, f, valid);
    }

    // Dict2vec's split: a definitional pair is *strong* when each word occurs
    // in the other's definition and *weak* when the membership is one-way. The
    // flat rule above pulls both at the same rate, which gives a passing
    // mention the same weight as a reciprocal definition.
    let mut graph = DefinitionGraph::build(entries);
    // Before promotion: the kernel is a property of the definitional graph, and
    // promotion adds edges inferred from phase similarity rather than from
    // definition. Peeling the promoted graph mixes the two and measures neither.
    let kernel = graph.kernel();
    let (raw_strong, raw_weak) = graph.counts();
    // Dict2vec SS3.1: promote a weak pair to strong when the two words are among
    // each other's K nearest. Raw reciprocity alone gives 505:1 on a cleaned
    // single dictionary; their 9:1 came from four concatenated modern
    // dictionaries plus this promotion, not from reciprocity by itself.
    let promoted = graph.promote_neighbours(base, 5);
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
            entries,
            rounds,
            HEAD_STEP,
            BETA_STRONG,
            BETA_WEAK,
            false,
            Some(&graph),
        );
        record(&mut rows, &mut best, "  + strong/weak (dict2vec)", f, valid);
    }
    {
        // Control: the same two rates applied without the reciprocity test, so
        // any gain above this row is the *split* rather than the rates.
        let mut f = base.clone();
        Conception::compose_graded(
            &mut f,
            entries,
            rounds,
            HEAD_STEP,
            BETA_STRONG,
            BETA_STRONG,
            false,
            None,
        );
        record(&mut rows, &mut best, "  control: flat at BETA_STRONG", f, valid);
    }

    anchor_conditions(s, &graph, &kernel, &mut rows, &mut best);
    controlled_negatives(s, &mut rows, &mut best);

    Outcome { rows, best }
}

/// Controlled negative sampling, measured on the training path rather than
/// asserted. This retrains from scratch with the filter attached, so the
/// comparison against `baseline` is the filter's whole effect.
fn controlled_negatives(s: &Setup, rows: &mut Vec<Row>, best: &mut (f64, Facet)) {
    let g = std::sync::Arc::new(DefinitionGraph::build(&s.entries));
    let filtered = Trainer::new(LEARNING_RATE).with_definitions(g);
    let f = Harness::train_ranking_only(&s.split, &filtered, 4);
    record(rows, best, "controlled negatives (retrained)", f, &s.split.valid);
}
