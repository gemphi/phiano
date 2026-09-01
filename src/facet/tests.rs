use super::*;

#[test]
fn test_empty_facet() {
    let f = Facet::new();
    assert_eq!(f.vocabulary_size(), 0);
    assert!(!f.contains_word("hello"));
    assert_eq!(f.average_amplitude(), 0.0);
    assert_eq!(f.dominant_band(), 1);
}

#[test]
fn test_add_and_query() {
    let mut f = Facet::new();
    f.lexicon.insert("hello".into(), SpectralPhasor::new(0.5, 1.0, 2));
    assert_eq!(f.vocabulary_size(), 1);
    assert!(f.contains_word("hello"));
    assert!(!f.contains_word("world"));
    assert_eq!(f.get_phasor("hello").unwrap().band_n, 2);
}

#[test]
fn test_average_amplitude() {
    let mut f = Facet::new();
    f.lexicon.insert("a".into(), SpectralPhasor::new(0.0, 1.0, 1));
    f.lexicon.insert("b".into(), SpectralPhasor::new(0.0, 3.0, 1));
    assert!((f.average_amplitude() - 2.0).abs() < 1e-10);
}

#[test]
fn test_dominant_band() {
    let mut f = Facet::new();
    f.lexicon.insert("a".into(), SpectralPhasor::new(0.0, 1.0, 1));
    f.lexicon.insert("b".into(), SpectralPhasor::new(0.0, 1.0, 3));
    f.lexicon.insert("c".into(), SpectralPhasor::new(0.0, 1.0, 3));
    assert_eq!(f.dominant_band(), 3);
}

#[test]
fn test_smoothed_bigram_never_zero_for_known_context() {
    let mut f = Facet::new();
    for _ in 0..3 {
        f.record_bigram("the", "cat");
    }
    // Raw MLE gives an unseen continuation exactly zero, which makes held-out
    // likelihood infinite; the discounted estimator leaves back-off mass.
    assert_eq!(f.bigram_probability("the", "zebra"), 0.0);
    let (p, backoff) = f.bigram_discounted("the", "zebra");
    assert_eq!(p, 0.0);
    assert!(backoff > 0.0, "unseen continuations must retain back-off mass");

    let (p_seen, _) = f.bigram_discounted("the", "cat");
    assert!(p_seen > 0.0 && p_seen < 1.0);
}

#[test]
fn test_prune_singletons_halves_a_sparse_table() {
    let mut f = Facet::new();
    for _ in 0..5 {
        f.record_bigram("the", "cat");
    }
    for i in 0..20 {
        f.record_bigram("the", &format!("rare{}", i));
    }
    let before = f.ngram_entries();
    let (bi, _tri) = f.prune_singletons();
    assert_eq!(bi, 20, "every singleton should go");
    assert!(f.ngram_entries() < before / 2);
    // The frequent transition survives.
    assert!(f.bigram_probability("the", "cat") > 0.0);
}
