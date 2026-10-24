#[allow(unused_imports)]
use super::*;

/// The chance level for head domination, given each head's fan-out and 64
/// modes to fall into, and how to read the table above.
pub(super) fn print_chance(pairs: &[(String, String)]) {
    let mut fan: HashMap<&str, usize> = HashMap::new();
    for (h, _) in pairs {
        *fan.entry(h.as_str()).or_insert(0) += 1;
    }
    let total: usize = fan.values().sum();
    let chance: f64 = fan
        .values()
        .map(|&f| {
            let f = f as f64;
            f * (1.0 - (1.0 - 1.0 / PHASE_CHANNELS as f64).powf(f - 1.0))
        })
        .sum::<f64>()
        / total.max(1) as f64;

    println!(
        "\n  head-dom chance level {:.1}%. Agreement is to the fixed harmonic basis,\n\
         \x20 so nothing is fitted and every pair is held out.\n\n\
         \x20 Read the last column. Centering within any partition removes variance,\n\
         \x20 so 'better than no C' is not evidence — a real C has to beat a fake C\n\
         \x20 of the same shape. And the two columns must move together: a transform\n\
         \x20 that merely destroys the offsets scores well on head-dom and nothing on\n\
         \x20 agreement, which is what the per-head row is there to show.",
        chance * 100.0
    );
}
