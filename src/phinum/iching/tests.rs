//! Unit tests for I Ching hexagrams and trigram spin.

use super::*;

#[test]
fn test_trigram_bits_and_symbols() {
    let heaven = Trigram::Heaven;
    assert_eq!(heaven.bits(), 0b111);
    assert_eq!(heaven.symbol(), "☰");
    assert_eq!(Trigram::from_bits(0b111), Trigram::Heaven);

    let earth = Trigram::Earth;
    assert_eq!(earth.bits(), 0b000);
    assert_eq!(earth.symbol(), "☷");
    assert_eq!(Trigram::from_bits(0b000), Trigram::Earth);
}

#[test]
fn test_hexagram_creation_and_spin() {
    let hex_0 = Hexagram::from_id(0);
    assert_eq!(hex_0.lower, Trigram::Earth);
    assert_eq!(hex_0.upper, Trigram::Earth);

    let hex_63 = Hexagram::from_id(63);
    assert_eq!(hex_63.lower, Trigram::Heaven);
    assert_eq!(hex_63.upper, Trigram::Heaven);

    let spun = hex_0.spin(std::f64::consts::PI);
    assert_eq!(spun.id, 32);

    let mutated = hex_0.changing_lines(0b000001);
    assert_eq!(mutated.id, 1);
}

#[test]
fn test_syntax_key_to_hexagram() {
    let key = crate::phinum::SyntaxParser::parse("i want to hug you");
    let hex = Hexagram::from_syntax_key(&key);
    assert!(hex.id < 64);
    assert!(!hex.archetype_name().is_empty());
}
