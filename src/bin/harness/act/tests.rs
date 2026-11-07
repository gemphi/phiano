use super::*;

#[test]
fn a_figure_below_the_declared_bar_is_a_miss_with_the_bar_on_record() {
    let m = percent("ladder.act.clean_soundness", Some(70.0), &Bars::default(), Some(CLEAN_SOUNDNESS_BAR));
    assert_eq!(m.bar, Some(Bar { value: CLEAN_SOUNDNESS_BAR, sense: Sense::AtLeast }));
    assert!(matches!(m.verdict, Judged::Miss { shortfall } if (shortfall + 6.8).abs() < 1e-9));
    let ok = percent("ladder.act.clean_coverage", Some(32.5), &Bars::default(), Some(CLEAN_COVERAGE_BAR));
    assert_eq!(ok.verdict, Judged::Pass);
}

#[test]
fn an_uncomputable_figure_is_a_named_absence_not_zero() {
    let m = percent("ladder.act.clean_soundness", None, &Bars::default(), Some(CLEAN_SOUNDNESS_BAR));
    assert_eq!(m.name, "ladder.act.clean_soundness");
    assert!(m.value.is_nan());
    assert_eq!(m.verdict, Judged::Unjudged);
    let empty = view("clean", &[], &Bars::default(), None);
    assert!(empty.iter().all(|m| m.value.is_nan()));
}

#[test]
fn no_comparable_previous_record_makes_the_regression_flag_absent() {
    let dir = std::env::temp_dir().join("harness_act_no_previous_record");
    let flag = regression(&[], &dir, &Bars::default());
    assert_eq!(flag.len(), 1);
    assert!(flag[0].value.is_nan());
}
