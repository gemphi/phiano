#[allow(unused_imports)]
use super::*;

/// Bottom-up, exactly as the project ships it.
pub(super) fn train_base(train: &[String], seed: u64) -> Facet {
    let split = Harness::split(train.to_vec(), seed);
    let trainer = Trainer::new(LEARNING_RATE).with_seed(seed);
    Harness::train_ranking_only(&split, &trainer, 4)
}

/// The control arm plus one arm per downward pass.
///
/// The downward pass sees the training sentences only. Held-out sentences are
/// what everything is scored on, and a rule that had seen them would be
/// fitting the test.
pub(super) fn build_arms(base: &Facet, train: &[String], seed: u64) -> Vec<Arm> {
    let train_toks: Vec<Vec<String>> = train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let null_toks = shuffled(&train_toks, seed ^ 0x5DEECE66D);

    let mut arms: Vec<Arm> = vec![Arm {
        label: "bottom-up only".into(),
        facet: base.clone(),
        report: None,
    }];

    for (label, sents, up, contrast) in [
        ("descent bag", &train_toks, Up::Bag, true),
        ("descent bound", &train_toks, Up::Bound, true),
        ("descent bag, no contrast", &train_toks, Up::Bag, false),
        ("descent bag, SHUFFLED", &null_toks, Up::Bag, true),
    ] {
        let mut f = base.clone();
        let r = Descent::cycle(&mut f, sents, up, 4, 0.25, contrast);
        arms.push(Arm { label: label.into(), facet: f, report: Some(r) });
    }

    arms
}
