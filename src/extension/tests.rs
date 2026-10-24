use super::*;
use crate::phasor::Rounding;

#[test]
fn channels_are_declared_past_the_base_and_named_once() {
    let mut e = Extension::default();
    let a = e.declare("is-a#0");
    let b = e.declare("scope:fn");
    assert_eq!((a, b), (BASE, BASE + 1));
    assert_eq!(e.declare("is-a#0"), a);
    assert_eq!(e.name(b), Some("scope:fn"));
    assert_eq!(e.channel("scope:fn"), Some(b));
    // After a round trip the index is rebuilt on demand.
    let back: Extension = serde_json::from_str(&serde_json::to_string(&e).unwrap()).unwrap();
    assert_eq!(back.channel("is-a#0"), Some(a));
}

#[test]
fn without_slots_a_kind_routes_to_itself() {
    let mut e = Extension::default();
    assert_eq!(e.route(9, "is-a", "mammal"), 9);
    assert_eq!(e.route_existing(9, "is-a", "mammal"), Some(9));
    assert_eq!(e.channels(), 0);
}

#[test]
fn slots_keep_one_words_answers_off_another_word() {
    // cat has fur; dog has fur and bark. On one angle per kind cat reads
    // bark; on slots it cannot, unless bark hashes onto fur's slot.
    let mut e = Extension::default();
    e.set_slots(9, 16);
    let fur = e.route(9, "has", "fur");
    let bark = e.route(9, "has", "bark");
    assert_ne!(fur, bark, "fur and bark share a slot at n = 16; pick other words");
    for _ in 0..200 {
        for (w, a, ch) in [("cat", "fur", fur), ("dog", "fur", fur), ("dog", "bark", bark)] {
            e.couple(w, a, ch, 0.06, 1.0, Rounding::Dither);
        }
    }
    assert!(e.on("cat", "fur", fur).unwrap() > 0.95);
    assert!(e.on("dog", "bark", bark).unwrap() > 0.95);
    assert_eq!(e.on("cat", "bark", bark), None, "cat was never told anything on bark's slot");
    assert_eq!(e.angles(), 2 * 5, "slots: cat 1, dog 2, fur 1, bark 1 — two angles each");
}
