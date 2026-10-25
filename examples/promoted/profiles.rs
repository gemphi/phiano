#[allow(unused_imports)]
use super::*;

/// What a word's polygon looks like: how many channels carry evidence for it,
/// and which ones are loudest.
pub(super) fn report_profiles(promoted: &Facet, pool: &[String]) {
    println!("\n  === evidence profiles (the polygon) ===");
    for w in ["the", "of", "a", "is", "and"] {
        if let Some(p) = promoted.lexicon.get(w) {
            print_polygon(promoted, w, p);
        }
    }
    let mut sample: Vec<&String> = pool.iter().filter(|w| w.len() > 4).take(6).collect();
    sample.sort();
    for w in sample {
        print_polygon(promoted, w, &promoted.lexicon[w]);
    }
}

fn print_polygon(promoted: &Facet, w: &str, p: &phiano::phasor::SpectralPhasor) {
    let mut top: Vec<(usize, u32)> =
        (0..PHASE_CHANNELS).map(|k| (k, p.evidence_count(k))).collect();
    top.sort_by_key(|x| std::cmp::Reverse(x.1));
    let sides = top.iter().filter(|(_, c)| *c > 0).count();
    println!(
        "  {:<8} {:>2} sides; loudest: {}",
        w,
        sides,
        top.iter()
            .take(5)
            .map(|(k, c)| format!("{}={}", promoted.relation_name(*k), c))
            .collect::<Vec<_>>()
            .join(", ")
    );
}
