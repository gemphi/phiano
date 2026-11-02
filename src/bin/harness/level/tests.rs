//! The level table's tests: the two properties the task names and the unit
//! facts that pin the table's shape.
//!
//! **Property 57: Only the ten level names are accepted** — for any string,
//! `accept` answers Ok exactly when the string is one of the ten declared
//! names, a rejection names the given value and all ten accepted names, and no
//! run record is left on disk.
//!
//! **Property 60: A level compiles its crate and nothing above it** — for all
//! ten levels, the crates compiled for a level are the transitive closure of
//! that crate's dependencies in the permitted-edge graph together with the
//! crate itself, no crate above it is compiled, and the count equals the set's
//! size. Exhaustive over the ten, then over 256 generated graphs.

use super::*;
use phistd::gen::Seeded;

/// The run's declared seed, matching the structural target's.
const SEED: u64 = 0x5747_4f4c_4341_494c;

// --- Property 57 -----------------------------------------------------------

/// A string the generator might offer: one of the ten, a case variant, a
/// padded name, a prefix, a suffix, or an unrelated word. The expected
/// acceptance is known by construction, not recomputed by a second copy of
/// the rule.
fn offered(g: &mut Seeded) -> (String, bool) {
    let base = *g.pick(&LEVELS);
    match g.below(8) {
        0 => (base.to_string(), true),            // exactly one of the ten
        1 => (base.to_uppercase(), false),        // a case variant
        2 => (format!("  {base}  "), false),      // padded
        3 => (format!("  {base}"), false),        // leading space
        4 => (base[..base.len() - 1].to_string(), false), // a prefix
        5 => (format!("{base}x"), false),         // a suffix
        6 => (String::new(), false),              // empty
        _ => (format!("ph{}", g.below(1000)), false), // unrelated
    }
}

/// **Validates: Requirements 1.8**
#[test]
fn property_57_only_the_ten_level_names_are_accepted() {
    // The directory the Harness would write a record into; the property
    // asserts no file lands here on a rejection. It is created fresh per run.
    let dir = std::env::temp_dir().join(format!("phiano_harness_57_{SEED:x}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp dir");
    let before = std::fs::read_dir(&dir).unwrap().count();

    let mut saw_accept = false;
    let mut saw_reject = false;
    for case in 0..256u64 {
        let mut g = Seeded::for_case(SEED, case as usize);
        let (given, should_accept) = offered(&mut g);
        let got = accept(&given);
        assert_eq!(
            got.is_ok(),
            should_accept,
            "case {case}: {given:?} acceptance disagreed with the ten-name list"
        );
        assert_eq!(is_level(&given), should_accept, "case {case}: is_level disagreed for {given:?}");
        match got {
            Ok(name) => {
                saw_accept = true;
                assert_eq!(name, given, "an accepted name is returned verbatim");
            }
            Err(HarnessError::UnknownLevel { given: g2, accepted }) => {
                saw_reject = true;
                assert_eq!(g2, given, "a rejection names the value given");
                assert_eq!(accepted, LEVELS, "a rejection names all ten accepted names");
                let said = HarnessError::UnknownLevel { given: g2, accepted }.to_string();
                for name in LEVELS {
                    assert!(said.contains(name), "case {case}: the message omits {name}");
                }
            }
        }
        // No run, so no record file, on either branch.
        assert_eq!(
            std::fs::read_dir(&dir).unwrap().count(),
            before,
            "case {case}: accepting or rejecting {given:?} left a file on disk"
        );
    }
    assert!(saw_accept && saw_reject, "the generator drew neither an accept nor a reject case");
    let _ = std::fs::remove_dir_all(&dir);
}

// --- Property 60 -----------------------------------------------------------

/// An independent transitive closure over `graph`: `level` and everything it
/// reaches, as an unordered set. The oracle for Property 60, written as a
/// plain reachability fixpoint so it shares no code with `closure_in`.
fn reach(graph: &[(&'static str, &'static [&'static str])], level: &'static str) -> Vec<&'static str> {
    let mut set: Vec<&'static str> = vec![level];
    let mut changed = true;
    while changed {
        changed = false;
        let frontier: Vec<&'static str> = set.clone();
        for c in frontier {
            if let Some((_, deps)) = graph.iter().find(|(name, _)| *name == c) {
                for &d in *deps {
                    if !set.contains(&d) {
                        set.push(d);
                        changed = true;
                    }
                }
            }
        }
    }
    set.sort_unstable();
    set
}

/// The real ten-crate graph `compiled_for` reads, exposed for the test.
fn real_graph() -> &'static [(&'static str, &'static [&'static str])] {
    super::PERMITTED
}

/// Generate an acyclic graph over the ten names: each crate may depend only on
/// crates earlier in `LEVELS`, so the graph is a topological order and can
/// hold no cycle, as the permitted graph is. Returned as a leaked slice so it
/// has the `'static` lifetime `closure_in` wants.
fn generated_graph(g: &mut Seeded) -> &'static [(&'static str, &'static [&'static str])] {
    let mut rows: Vec<(&'static str, &'static [&'static str])> = Vec::new();
    for (i, &name) in LEVELS.iter().enumerate() {
        let mut deps: Vec<&'static str> = Vec::new();
        for &lower in &LEVELS[..i] {
            if g.below(2) == 0 {
                deps.push(lower);
            }
        }
        let deps: &'static [&'static str] = Box::leak(deps.into_boxed_slice());
        rows.push((name, deps));
    }
    Box::leak(rows.into_boxed_slice())
}

/// One graph's check: for every crate, the closure equals the oracle set, no
/// crate above it is included, and the ordering is a topological order.
fn check_graph(graph: &[(&'static str, &'static [&'static str])]) {
    for (i, &level) in LEVELS.iter().enumerate() {
        let got = closure_in(graph, level);
        let mut got_sorted = got.clone();
        got_sorted.sort_unstable();
        got_sorted.dedup();
        assert_eq!(got_sorted.len(), got.len(), "{level}: the closure holds a crate twice");

        let oracle = reach(graph, level);
        assert_eq!(got_sorted, oracle, "{level}: the closure disagreed with the reachability oracle");

        assert!(got.contains(&level), "{level}: its own crate is absent from the closure");
        // Nothing strictly above the level in `LEVELS` is compiled.
        for &higher in &LEVELS[i + 1..] {
            assert!(!got.contains(&higher), "{level}: {higher} is above it yet was compiled");
        }
        // Dependency order: each crate precedes any that depends on it, so a
        // crate's position in `graph` never decreases along the result.
        let mut last = None;
        for c in &got {
            let pos = graph.iter().position(|(n, _)| n == c).unwrap();
            if let Some(p) = last {
                assert!(pos > p, "{level}: {c} is out of dependency order");
            }
            last = Some(pos);
        }
        // The reported count equals the set's size.
        assert_eq!(got.len(), oracle.len(), "{level}: the reported count is not the set size");
    }
}

/// **Validates: Requirements 11.11**
#[test]
fn property_60_a_level_compiles_its_crate_and_nothing_above_it() {
    // Exhaustive over the ten, against the real graph.
    check_graph(real_graph());

    // A further 256 generated acyclic graphs over the same ten names.
    for case in 0..256u64 {
        let mut g = Seeded::for_case(SEED ^ 0x60, case as usize);
        let graph = generated_graph(&mut g);
        check_graph(graph);
    }
}

// --- Unit facts about the table --------------------------------------------

#[test]
fn the_ten_names_are_in_dependency_order() {
    assert_eq!(LEVELS.len(), 10);
    assert_eq!(LEVELS[0], "phistd");
    assert_eq!(LEVELS[9], "phiano");
    // Every permitted edge is downward: a crate depends only on earlier ones.
    for (i, &name) in LEVELS.iter().enumerate() {
        for &dep in compiled_for(name).iter().filter(|&&d| d != name) {
            let j = LEVELS.iter().position(|&l| l == dep).unwrap();
            assert!(j < i, "{name} depends on {dep}, which is not below it");
        }
    }
}

#[test]
fn phistd_compiles_alone_and_phiano_pulls_the_stack() {
    assert_eq!(compiled_for("phistd"), vec!["phistd"]);
    let all = compiled_for("phiano");
    assert_eq!(all.len(), 10, "phiano closes over all ten crates");
    assert_eq!(*all.last().unwrap(), "phiano", "the level itself is last");
    for name in LEVELS {
        assert!(all.contains(&name), "phiano's closure omits {name}");
    }
}

#[test]
fn an_unaccepted_name_compiles_nothing() {
    assert!(compiled_for("nope").is_empty());
    assert!(compiled_for("PHISTD").is_empty(), "the match is case-exact");
    assert!(compiled_for(" phistd ").is_empty(), "the match rejects padding");
}

#[test]
fn the_level_graph_matches_the_structural_table() {
    // The structural PERMITTED table, copied here as the oracle. If the two
    // drift, this fails before the walk trusts a stale closure.
    let structural: &[(&str, &[&str])] = &[
        ("phistd", &[]),
        ("phifno", &["phistd"]),
        ("phiton", &["phistd"]),
        ("phical", &["phistd", "phifno"]),
        ("phiecs", &["phistd", "phiton", "phical"]),
        ("phiclm", &["phiecs"]),
        ("phient", &["phistd", "phical", "phiton", "phiecs", "phiclm"]),
        ("philog", &["phient", "phiecs", "phiton"]),
        ("phisor", &["phiecs", "phiton", "phical", "phistd", "phiclm"]),
        (
            "phiano",
            &["phistd", "phiton", "phical", "phifno", "phient", "philog", "phiecs", "phiclm", "phisor"],
        ),
    ];
    for (name, deps) in structural {
        let ours = super::PERMITTED.iter().find(|(c, _)| c == name).map(|(_, d)| *d);
        assert_eq!(ours, Some(*deps), "the {name} row disagrees with the structural table");
    }
    assert_eq!(super::PERMITTED.len(), structural.len(), "the two tables hold different crate counts");
}
