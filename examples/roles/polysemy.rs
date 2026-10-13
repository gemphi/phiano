#[allow(unused_imports)]
use super::*;

/// How much does one-phasor-per-word cost?
///
/// `Facet::lexicon` is `HashMap<String, SpectralPhasor>`: exactly one point per
/// word. Webster's gives *cat* eight numbered senses — the animal, a strong
/// sailing vessel, a double tripod, the constellation — and `definition_core`
/// concatenates three of them into one gloss which composes to one centroid.
/// Every measurement in this project has been taken on a representation that
/// cannot hold polysemy.
///
/// That is a candidate explanation for why prepositional relations sit at 1.7x
/// noise rather than 10x: `of(brim, hat)` and `of(pneumonia, lungs)` are already
/// different relations sharing a preposition, and on top of that *brim* and
/// *lungs* are each an average over senses. Two sources of blur, and only the
/// first has been named.
///
/// The test needs no retraining and no sense-tagged corpus. If sense collapse is
/// a noise source, then relations whose **head is monosemous** must show more
/// coherence over its own null than relations whose head has many senses. The
/// senses are counted from the dictionary; the phasors are the ones already
/// trained. Nothing here is fitted, so nothing here can be fitting.
pub(super) fn polysemy_experiment(facet: &Facet, chunks_path: &str) {
    let senses = load_senses(chunks_path);
    let lex = lexical_rules(facet, &senses.glosses);

    let ratios = sense_bands(facet, &lex, &senses.counts);
    let fratios = frequency_bands(facet, &lex);

    polysemy_verdict(&ratios, &fratios);
}
