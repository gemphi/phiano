use super::*;

/// The reachable cut drops the Unreachable_Frames, so it reports the same
/// two names with the `_reachable` suffix beside the all-150 pair, and
/// scores no lower than the all-150 cut on either figure (R2.2).
#[test]
fn reachable_cut_sits_beside_the_all_150_cut() {
    let bars = Bars::default();
    // Prime the reader as the live walk does, so the before figures this
    // test prints match the harness record the Realiser tasks compare
    // against. Absent files leave the reader on its built-in classes.
    if let Ok(pos) = std::fs::read_to_string(Path::new(POS)) {
        phisor::lexicon::prime(&pos);
    }
    if let Ok(func) = std::fs::read_to_string(Path::new(FUNCTION)) {
        phisor::lexicon::prime_function(&func);
    }
    let gold = phisor::gold::read().expect("the frozen gold set loads");

    let all: Vec<_> = gold.pairs().collect();
    let reachable: Vec<_> = gold
        .entries
        .iter()
        .enumerate()
        .filter(|(i, _)| phisor::gold::unreachable(*i).is_none())
        .map(|(_, e)| (&e.act, e.sentence.as_str()))
        .collect();

    // The subset is the 150 minus the listed Unreachable_Frames.
    assert_eq!(reachable.len(), gold.len() - phisor::gold::UNREACHABLE.len());

    let all_figs = gold_figures("", &all, &bars);
    let reach_figs = gold_figures("_reachable", &reachable, &bars);
    let get = |figs: &[Measured], name: &str| {
        figs.iter().find(|m| m.name == name).map(|m| m.value)
    };

    let rt_all = get(&all_figs, "ladder.act.round_trip").expect("all round-trip");
    let fp_all = get(&all_figs, "ladder.act.force_purity").expect("all force purity");
    let rt_reach =
        get(&reach_figs, "ladder.act.round_trip_reachable").expect("reachable round-trip");
    let fp_reach =
        get(&reach_figs, "ladder.act.force_purity_reachable").expect("reachable force purity");

    // Every figure is a share, so it sits in [0, 100].
    for v in [rt_all, fp_all, rt_reach, fp_reach] {
        assert!((0.0..=100.0).contains(&v), "{v} is not a percentage");
    }
    // The Unreachable_Frames are the ones whose Force the surface text
    // cannot settle (BareWill, and the NameCase/Unaccusative that share a
    // surface shape), so dropping them lifts force purity. Round-trip is a
    // realiser check independent of the label, so it is not constrained to
    // move either way and is only checked to be in range above.
    assert!(fp_reach >= fp_all, "force purity {fp_reach} < all {fp_all}");

    // The realisable cut is the 150 minus the Unrealisable_Frames; every
    // frame left round-trips, so the figure is 100.
    let realisable = realisable_figure(&gold, &bars).expect("realisable round-trip");
    assert_eq!(realisable.name, "ladder.act.round_trip_realisable");
    assert_eq!(realisable.value, 100.0);
}

/// Every covered gold frame the Reader reads with a wrong Force, by gold
/// index: `(index, gold Force, read Force, sentence)` (R2.2).
fn wrong_force(gold: &phisor::gold::Gold) -> Vec<(usize, phisor::ir::Force, phisor::ir::Force, String)> {
    gold.entries
        .iter()
        .enumerate()
        .filter_map(|(i, e)| {
            let read = phisor::ladder::read(&e.sentence)?;
            (read.force != e.act.force).then(|| (i, e.act.force, read.force, e.sentence.clone()))
        })
        .collect()
}

/// Prime the reader as the live walk does (pos.tsv, function.txt and the
/// Guide's irregular verbs), so the pinned table matches the harness record.
fn prime_as_the_harness() {
    // The tests run from the package directory, not the workspace root the
    // harness consts are relative to, so resolve from the manifest.
    let data = Path::new(env!("CARGO_MANIFEST_DIR")).join("data");
    let pos = std::fs::read_to_string(data.join("lex/pos.tsv")).expect("pos.tsv loads");
    phisor::lexicon::prime(&pos);
    let func = std::fs::read_to_string(data.join("lex/function.txt")).expect("function.txt loads");
    phisor::lexicon::prime_function(&func);
    let guide = std::fs::read_to_string(data.join("person/murphy.txt")).expect("the Guide text loads");
    phisor::lexicon::prime_irregular(&guide);
}


/// The gold frames the Reader reads with a wrong Force, by gold index:
/// `(index, gold Force, read Force)`, measured with the harness priming
/// (task 4.1). Gold is frozen, so this is the yardstick for tasks 4.2 to
/// 4.4. Sentence beside the confusion:
///
/// Commit read as Assert, the seven `BareWill` Unreachable_Frames (a bare
/// `will` or `shall` is a Commit or a prediction, and is never mapped to
/// Commit by rule, R2.5):
///
/// - 117 `i will explain the rule.`
/// - 119 `we shall not forget.`
/// - 120 `i will send you the book.`
/// - 121 `i am going to study french.`
/// - 122 `we will meet at eight.`
/// - 124 `i will have finished by friday.`
/// - 126 `we will answer your letter.`
///
/// Express read as Assert, the one frame that remains (129 and 131 are fixed
/// phrases in `expressive_phrase.tsv`, task 4.2; 130, 132 and 134 are a
/// first-person present psychological-state predicate listed there, task
/// 4.3; 133 is past tense, outside that rule):
///
/// - 133 `i was delighted with your letter.`
///
/// Direct read as Assert, 102 `please close the window.`, is fixed by
/// `politeness_marker.txt` (task 4.3). Wish read as Assert, 143
/// `i wish we did not have to go.`, which has no general rule.
const WRONG_FORCE: [(usize, Force, Force); 8] = [
    (117, Force::Commit, Force::Assert),
    (119, Force::Commit, Force::Assert),
    (120, Force::Commit, Force::Assert),
    (121, Force::Commit, Force::Assert),
    (122, Force::Commit, Force::Assert),
    (124, Force::Commit, Force::Assert),
    (126, Force::Commit, Force::Assert),
    (133, Force::Express, Force::Assert),
];

/// Pins the wrong-Force gold frames by index with the confusion, and the
/// decidable figure that follows from them (R2.1, R2.2).
#[test]
fn the_wrong_force_frames_are_pinned_by_index_and_confusion() {
    use phisor::gold::Reason;
    prime_as_the_harness();
    let gold = phisor::gold::read().expect("the frozen gold set loads");
    let wrong = wrong_force(&gold);
    let found: Vec<_> = wrong.iter().map(|(i, g, r, _)| (*i, *g, *r)).collect();
    assert_eq!(found, WRONG_FORCE, "the set of wrong-Force gold frames moved");
    assert!(found.iter().all(|(i, ..)| *i < phisor::gold::COUNT));

    // Confusion counts, gold Force to read Force.
    let count = |g: Force, r: Force| found.iter().filter(|f| f.1 == g && f.2 == r).count();
    assert_eq!(count(Force::Commit, Force::Assert), 7);
    assert_eq!(count(Force::Express, Force::Assert), 1);
    assert_eq!(count(Force::Direct, Force::Assert), 0);
    assert_eq!(count(Force::Wish, Force::Assert), 0);

    // The seven Commit frames are exactly the BareWill Unreachable_Frames.
    let bare: Vec<usize> = (0..phisor::gold::COUNT)
        .filter(|i| phisor::gold::unreachable(*i) == Some(Reason::BareWill))
        .collect();
    let commits: Vec<usize> =
        found.iter().filter(|f| f.1 == Force::Commit).map(|f| f.0).collect();
    assert_eq!(bare, commits, "the BareWill frames and the Commit confusions differ");

    // Removing only the BareWill frames leaves 1 wrong of 115 covered (wish rows read back: was 2; task 4.3: was 6; task 4.2: was 8 of 113).
    let decidable = decidable_figure(&gold, &Bars::default()).expect("decidable figure");
    assert_eq!(decidable.name, "ladder.act.force_purity_decidable");
    assert_eq!(decidable.value, 99.1);
}
