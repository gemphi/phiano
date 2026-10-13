#[allow(unused_imports)]
use super::*;

/// Percentile of a sorted duration list, nearest-rank.
pub(super) fn pct(sorted: &[Duration], p: f64) -> Duration {
    if sorted.is_empty() {
        return Duration::ZERO;
    }
    let idx = ((p / 100.0) * sorted.len() as f64).ceil() as usize;
    sorted[idx.saturating_sub(1).min(sorted.len() - 1)]
}

/// Microseconds. Two of these paths are a phase write and a table insert, which
/// land below a millisecond's resolution; printing them as "0.000 ms" reads as a
/// broken measurement rather than as the point.
pub(super) fn us(d: Duration) -> f64 {
    d.as_secs_f64() * 1_000_000.0
}

impl Path {
    pub(super) fn report(&self) {
        let mut s = self.samples.clone();
        s.sort();
        println!(
            "  {:<10} n={:<4} p50 {:>10.2} us   p99 {:>10.2} us   max {:>10.2} us   {}",
            self.name,
            s.len(),
            us(pct(&s, 50.0)),
            us(pct(&s, 99.0)),
            us(*s.last().unwrap_or(&Duration::ZERO)),
            self.note
        );
    }
}
