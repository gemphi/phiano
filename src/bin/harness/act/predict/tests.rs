use super::*;

fn scores(top1: Option<f64>, ppl: Option<f64>) -> Scores {
    Scores {
        total: 10,
        predicted: 4,
        abstained: 6,
        abstract_hits: 2,
        surface_hits: 1,
        abstract_top1: top1,
        surface_top1: top1,
        abstract_perplexity: ppl,
        surface_perplexity: ppl,
    }
}

#[test]
fn gate_opens_only_when_learned_beats_both_controls() {
    let learned = scores(Some(0.6), Some(5.0));
    let weak = scores(Some(0.2), Some(9.0));
    assert!(decide(&learned, &weak, &weak).enabled);
    // Better perplexity but worse top-1 than one control: stays off.
    let sharp = scores(Some(0.9), Some(9.0));
    let g = decide(&learned, &weak, &sharp);
    assert!(!g.enabled && g.reason.contains("frequency"), "{g:?}");
}

#[test]
fn an_uncomputable_figure_keeps_the_layer_off_and_is_named() {
    let blind = scores(None, Some(5.0));
    let weak = scores(Some(0.2), Some(9.0));
    let g = decide(&blind, &weak, &weak);
    assert!(!g.enabled && g.reason.contains("uncomputable"), "{g:?}");
    let (records, _) = gate_records(&blind, &weak, &weak, &Bars::default());
    assert!(records[0].value.is_nan() && records[1].value.is_nan());
    assert_eq!(records[2].value, 0.0);
}

#[test]
fn a_control_that_never_answers_is_judged_on_perplexity_alone() {
    let learned = scores(Some(0.6), Some(5.0));
    let silent = scores(None, Some(9.0));
    assert!(decide(&learned, &silent, &silent).enabled);
    let better = scores(None, Some(4.0));
    assert!(!decide(&learned, &better, &silent).enabled);
}

#[test]
fn missing_scores_are_eight_named_absences() {
    let r = arm_records("learned", None, &Bars::default());
    assert_eq!(r.len(), 8);
    assert!(r.iter().all(|m| m.value.is_nan() && m.name.starts_with("ladder.act.")));
    assert!(r.iter().any(|m| m.name == "ladder.act.surface_learned_top1"));
}

#[test]
fn the_gate_never_changes_the_default() {
    assert!(!Config::default().enabled);
}

/// Run the real Guide split once and print the figures and the gate verdict.
/// Run from the workspace root: `cargo test -p phiano --bin harness real_guide -- --ignored --nocapture`.
#[test]
#[ignore = "reads the real Guide; slow"]
fn real_guide_figures() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let text = std::fs::read_to_string(root.join("crates/phiano/data/person/murphy.txt")).unwrap();
    let key = std::fs::read_to_string(root.join("crates/phiano/data/person/murphy_key.txt")).unwrap();
    phisor::lexicon::prime_irregular(&text);
    let split = Split::new(phisor::guide::read(&text, &key));
    for m in measure(&split, &Bars::default()) {
        eprintln!("{} = {}", m.name, m.value);
    }
}
