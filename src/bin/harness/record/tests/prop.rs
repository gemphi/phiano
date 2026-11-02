//! Property 4 and Property 5.

use super::{count_verdicts, draw_measured, same, SEED};
use crate::record::{cap, parse, page, Measured};
use phistd::gen::Seeded;
use std::collections::BTreeMap;

/// The four declared caps of Property 4: a failure message and a compiler
/// error at 4,000, an Oracle's standard-error text at 2,000, a named-case list
/// at 20.
const CAPS: [usize; 4] = [4_000, 4_000, 2_000, 20];

/// **Property 4: A recorded field is capped and is a prefix of the original**
///
/// *For any* text and any declared cap, the recorded field's length is at most
/// the cap and the recorded field is a prefix of the original.
///
/// **Validates: Requirements 1.3, 1.9, 2.5, 3.8, 10.6**
#[test]
fn property_4_a_recorded_field_is_capped_and_is_a_prefix() {
    for case in 0..256u64 {
        let mut g = Seeded::for_case(SEED, case as usize);
        // A text of length 0 to 20,000, mixing ASCII with a multi-byte char so
        // the char-boundary cut is exercised.
        let len = g.below(20_001) as usize;
        let text: String = (0..len)
            .map(|i| if g.below(8) == 0 { 'é' } else { (b'a' + (i % 26) as u8) as char })
            .collect();
        let the_cap = CAPS[g.below(CAPS.len() as u64) as usize];

        let recorded = cap(&text, the_cap);

        assert!(
            recorded.chars().count() <= the_cap,
            "case {case}: recorded {} chars exceeds the cap {the_cap}",
            recorded.chars().count()
        );
        assert!(
            text.starts_with(&recorded),
            "case {case}: the recorded field is not a prefix of the original"
        );
        if text.chars().count() <= the_cap {
            assert_eq!(recorded, text, "case {case}: a short field was altered");
        }
    }
}

/// **Property 5: The record is complete and its verdict counts partition it**
///
/// *For any* set of measured numbers, every number appears exactly once in the
/// rendered page carrying its name, value, unit, Bar, control name and value,
/// and verdict; every unit is one of the six declared; and the counts of
/// passes, Misses and unjudged entries sum to the total.
///
/// **Validates: Requirements 2.1, 2.3**
#[test]
fn property_5_the_record_is_complete_and_its_counts_partition() {
    for case in 0..256u64 {
        let mut g = Seeded::for_case(SEED ^ 0x5, case as usize);
        let count = g.below(65) as usize; // 0 to 64 numbers
        let numbers: Vec<Measured> = (0..count).map(|i| draw_measured(&mut g, i)).collect();

        let body = page::render("- `workspace` abc123 clean", &numbers, &BTreeMap::new());
        let read = parse::parse(&body);

        assert_eq!(read.len(), numbers.len(), "case {case}: a number was lost or doubled");
        for (want, got) in numbers.iter().zip(read.iter()) {
            assert_eq!(got.name, want.name, "case {case}: name");
            assert!(same(got.value, want.value), "case {case}: value {} != {}", got.value, want.value);
            assert_eq!(got.unit, want.unit, "case {case}: unit");
            assert_eq!(got.bar, want.bar, "case {case}: bar");
            assert_eq!(got.control, want.control, "case {case}: control");
            assert_eq!(got.verdict, want.verdict, "case {case}: verdict");
        }

        let (p, mi, u) = count_verdicts(&numbers);
        assert_eq!(p + mi + u, numbers.len(), "case {case}: the three verdicts do not partition");
        let header = format!("recorded {}; passes {p}; misses {mi}; unjudged {u}", numbers.len());
        assert!(body.contains(&header), "case {case}: the Totals header disagrees with the partition");
    }
}
