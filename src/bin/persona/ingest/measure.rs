#[allow(unused_imports)]
use super::*;

/// Attribution on held-out paragraphs: four consecutive held-out sentences,
/// the persona with the highest likelihood wins. Fingerprints are extracted
/// once against `facet` rather than per paragraph.
pub(crate) fn attribution(world: &World, facet: &Facet, authors: &[Author], paragraphs: usize) -> (f64, Vec<(String, usize, usize, f64)>) {
    let fps = world.fingerprints(facet);
    let mut rows = Vec::new();
    let (mut ok_all, mut n_all) = (0usize, 0usize);
    for a in authors {
        let (mut ok, mut n, mut conf) = (0usize, 0usize, 0.0);
        for chunk in a.split.test.chunks(4).take(paragraphs) {
            let text = chunk.join(". ");
            let Some(m) = world.match_text_with(facet, &text, &fps) else { continue };
            n += 1;
            if m.best_name == a.name {
                ok += 1;
            }
            conf += m.confidence;
        }
        ok_all += ok;
        n_all += n;
        rows.push((a.name.clone(), ok, n, conf / n.max(1) as f64));
    }
    (100.0 * ok_all as f64 / n_all.max(1) as f64, rows)
}
