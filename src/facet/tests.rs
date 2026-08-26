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
