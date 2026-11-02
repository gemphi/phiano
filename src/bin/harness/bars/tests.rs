//! The Bars reader's tests: Property 6 and the unit facts that pin the
//! reader's three outcomes and the shape of `phinet/docs/bars.md`.
//!
//! **Property 6: The Bar verdict has exactly three outcomes and its shortfall
//! is signed by the Bar's sense** — for any measured value and any declared
//! Bars input, the verdict is `Unjudged` exactly when the name is absent or
//! the input could not be read; otherwise `Pass` exactly when the Bar is met
//! under its declared sense; and a `Miss` records a shortfall equal to the
//! signed distance from the Bar in the number's own unit. Values are placed
//! at, just below and just above each Bar.

use super::*;
use phistd::gen::Seeded;

/// The run's declared seed, matching the harness's other structural targets.
const SEED: u64 = 0x5241_425f_5048_494e;

// --- Property 6 ------------------------------------------------------------

/// One generated case: a Bar value, a sense, a placement of the measured value
/// relative to the Bar, and whether the name is present in the input. The
/// expected verdict is known by construction here, not recomputed by a second
/// copy of `judge`.
struct Case {
    bar_value: f64,
    sense: Sense,
    value: f64,
    /// Where `value` sits: -1 just below the Bar, 0 at it, +1 just above.
    placement: i8,
    present: bool,
}

fn draw_case(g: &mut Seeded) -> Case {
    // A Bar value in a human range, kept well away from the float edges so
    // "just below" and "just above" stay distinct from the Bar itself.
    let bar_value = (g.below(10_000) as f64) / 100.0; // 0.00 ..= 99.99
    let sense = if g.below(2) == 0 { Sense::AtLeast } else { Sense::AtMost };
    let placement = [-1i8, 0, 1][g.below(3) as usize];
    let delta = 0.01; // one step in the two-decimal unit the Bars use
    let value = match placement {
        -1 => bar_value - delta,
        1 => bar_value + delta,
        _ => bar_value,
    };
    let present = g.below(2) == 0;
    Case { bar_value, sense, value, placement, present }
}

/// The oracle: the verdict computed directly from the sense and the placement,
/// sharing no code with `Bars::judge`.
fn expected(c: &Case) -> Judged {
    if !c.present {
        return Judged::Unjudged;
    }
    let met = match (c.sense, c.placement) {
        // At the Bar is always met: both senses are inclusive.
        (_, 0) => true,
        (Sense::AtLeast, 1) => true,
        (Sense::AtLeast, -1) => false,
        (Sense::AtMost, -1) => true,
        (Sense::AtMost, 1) => false,
        _ => unreachable!("placement is only -1, 0 or 1"),
    };
    if met {
        Judged::Pass
    } else {
        Judged::Miss { shortfall: c.value - c.bar_value }
    }
}

/// **Validates: Requirements 2.2, 2.6, 2.7, 2.8**
#[test]
fn property_6_the_bar_verdict_has_three_outcomes_signed_by_sense() {
    let mut saw_pass = false;
    let mut saw_miss = false;
    let mut saw_unjudged = false;
    for case in 0..256u64 {
        let mut g = Seeded::for_case(SEED, case as usize);
        let c = draw_case(&mut g);

        // Build a one-Bar input; the judged name is present or absent per the
        // case, so the absent branch exercises R2.6.
        let mut map = BTreeMap::new();
        map.insert("n".to_string(), Bar { value: c.bar_value, sense: c.sense });
        let bars = Bars::from_map(map);
        let name = if c.present { "n" } else { "absent" };

        let got = bars.judge(name, c.value);
        let want = expected(&c);

        match (got, want) {
            (Judged::Miss { shortfall: a }, Judged::Miss { shortfall: b }) => {
                assert!(
                    (a - b).abs() < 1e-9,
                    "case {case}: shortfall {a} disagreed with the signed distance {b}"
                );
                // The shortfall's sign follows the sense: an AtLeast miss is
                // from below (negative), an AtMost miss is from above
                // (positive).
                match c.sense {
                    Sense::AtLeast => assert!(a < 0.0, "case {case}: an AtLeast miss is signed below"),
                    Sense::AtMost => assert!(a > 0.0, "case {case}: an AtMost miss is signed above"),
                }
                saw_miss = true;
            }
            (Judged::Pass, Judged::Pass) => saw_pass = true,
            (Judged::Unjudged, Judged::Unjudged) => saw_unjudged = true,
            (g, w) => panic!("case {case}: verdict {g:?} disagreed with the oracle {w:?}"),
        }

        // The verdict is one of exactly three outcomes — enforced by the match
        // above being total over `Judged`.
    }
    assert!(saw_pass, "the generator drew no pass case");
    assert!(saw_miss, "the generator drew no miss case");
    assert!(saw_unjudged, "the generator drew no unjudged case");
}

/// An unreadable input makes every name `Unjudged`, never a pass (R2.8), even
/// a name that would otherwise carry a Bar.
#[test]
fn property_6_an_unreadable_input_judges_nothing() {
    let missing = std::env::temp_dir().join(format!("phiano_bars_absent_{SEED:x}.md"));
    let _ = std::fs::remove_file(&missing);
    let bars = Bars::read(&missing);
    assert!(bars.unreadable.is_some(), "a missing file reports its path");
    assert!(bars.unreadable.as_ref().unwrap().contains("phiano_bars_absent"), "the path is named");
    for case in 0..256u64 {
        let mut g = Seeded::for_case(SEED ^ 0x8, case as usize);
        let value = (g.below(10_000) as f64) / 100.0;
        assert_eq!(bars.judge("n", value), Judged::Unjudged, "case {case}: unreadable yet judged");
    }
    assert_eq!(bars.bar("n"), None, "an unreadable input declares no Bar");
}

// --- Unit facts about the reader -------------------------------------------

/// A small in-memory `bars.md` written to a temp file, parsed back.
fn read_fixture(body: &str) -> Bars {
    let path = std::env::temp_dir().join(format!("phiano_bars_fixture_{:x}.md", body.len() as u64));
    std::fs::write(&path, body).expect("write fixture");
    let bars = Bars::read(&path);
    let _ = std::fs::remove_file(&path);
    bars
}

#[test]
fn a_well_formed_table_parses_every_data_row() {
    let body = "\
# heading prose, skipped

| name | value | sense | unit | control |
|---|---|---|---|---|
| `a.purity` | 56.7 | AtLeast | Percentage | — |
| `b.perplexity` | 84.7 | AtMost | Perplexity | trigram |
not a table row, skipped
| `c.count` | 64 | AtLeast | Count | — |
";
    let bars = read_fixture(body);
    assert!(bars.unreadable.is_none());
    assert_eq!(bars.len(), 3, "three data rows were parsed; the header and prose were skipped");
    assert_eq!(bars.bar("a.purity"), Some(Bar { value: 56.7, sense: Sense::AtLeast }));
    assert_eq!(bars.bar("b.perplexity"), Some(Bar { value: 84.7, sense: Sense::AtMost }));
    assert_eq!(bars.bar("c.count"), Some(Bar { value: 64.0, sense: Sense::AtLeast }));
    assert_eq!(bars.bar("missing"), None, "an absent name declares no Bar");
}

#[test]
fn a_malformed_row_is_skipped_not_fatal() {
    let body = "\
| `ok` | 1.0 | AtLeast | Count | |
| `bad.value` | not-a-number | AtLeast | Count | |
| `bad.sense` | 1.0 | sideways | Count | |
| `bad.unit` | 1.0 | AtLeast | furlongs | |
| not-quoted | 1.0 | AtLeast | Count | |
";
    let bars = read_fixture(body);
    assert_eq!(bars.len(), 1, "only the well-formed row parsed");
    assert_eq!(bars.bar("ok"), Some(Bar { value: 1.0, sense: Sense::AtLeast }));
    assert_eq!(bars.judge("bad.value", 1.0), Judged::Unjudged, "a skipped row judges nothing");
}

#[test]
fn an_atmost_bar_passes_at_and_below_and_misses_above() {
    let mut map = BTreeMap::new();
    map.insert("p".to_string(), Bar { value: 100.0, sense: Sense::AtMost });
    let bars = Bars::from_map(map);
    assert_eq!(bars.judge("p", 84.7), Judged::Pass, "below an AtMost Bar passes");
    assert_eq!(bars.judge("p", 100.0), Judged::Pass, "at an AtMost Bar passes");
    match bars.judge("p", 500.0) {
        Judged::Miss { shortfall } => assert!((shortfall - 400.0).abs() < 1e-9),
        other => panic!("a perplexity of 500 should miss, got {other:?}"),
    }
}

#[test]
fn the_shipped_bars_file_declares_the_eighteen_named_bars() {
    // The real declared-Bars input ships beside the run records. It must parse
    // and hold exactly the eighteen Bars the requirements name, with the two
    // AtMost Bars facing the right way.
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("docs")
        .join("bars.md");
    let bars = Bars::read(&path);
    assert!(bars.unreadable.is_none(), "the shipped bars.md must be readable: {:?}", bars.unreadable);
    assert_eq!(bars.len(), 18, "bars.md declares the eighteen named Bars");

    // The two Bars met by falling below, named by the requirements.
    assert_eq!(bars.bar("lowering.unlowered").map(|b| b.sense), Some(Sense::AtMost));
    assert_eq!(bars.bar("curriculum.perplexity").map(|b| b.sense), Some(Sense::AtMost));
    // A sample of the floors.
    assert_eq!(bars.bar("lowering.agreeing"), Some(Bar { value: 64.0, sense: Sense::AtLeast }));
    assert_eq!(bars.bar("keyboard.operator_count"), Some(Bar { value: 64.0, sense: Sense::AtLeast }));
    assert_eq!(bars.bar("curriculum.top1"), Some(Bar { value: 24.8, sense: Sense::AtLeast }));

    // A perplexity of 500 misses its AtMost Bar — the reason the sense is
    // declared rather than inferred from the unit.
    assert!(matches!(bars.judge("curriculum.perplexity", 500.0), Judged::Miss { .. }));
    // Every declared Bar carries one of the two senses and judges a value.
    for name in [
        "lowering.unlowered",
        "curriculum.perplexity",
        "ladder.function_purity.learned",
        "truth.recall.degenerate",
    ] {
        assert!(bars.bar(name).is_some(), "the shipped bars.md omits {name}");
    }
}
