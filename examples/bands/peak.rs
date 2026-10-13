#[allow(unused_imports)]
use super::*;

/// The best band, and how far it stands above the others in standard
/// deviations.
///
/// A peak has to clear the spread of the other bands, or it is the argmax of
/// noise. Two flat profiles always put their maxima in different bands, and
/// reporting that as "the levels separate" is the same false positive this
/// project has now made three times.
pub(super) fn peak(v: &[f64]) -> (usize, f64) {
    let i = v
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
        .unwrap_or(0);
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    let sd = (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0).max(1.0))
        .sqrt();
    let z = match sd > 1e-12 {
        true => (v[i] - mean) / sd,
        false => 0.0,
    };
    (i, z)
}
