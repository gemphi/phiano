#[allow(unused_imports)]
use super::*;

pub(super) fn show_sweep(label: &str, sweep: &[(f64, f64)]) {
    let alone = sweep.iter().find(|(g, _)| (*g - 1.0).abs() < 1e-9).map(|x| x.1).unwrap_or(f64::NAN);
    let (bg, best) = sweep
        .iter()
        .cloned()
        .fold((0.0, f64::INFINITY), |a, b| if b.1 < a.1 { b } else { a });
    let off = sweep.first().map(|x| x.1).unwrap_or(f64::NAN);
    println!(
        "  {:<40} alone (γ=1) {:>8.2}   best mix {:>8.2} at γ={:.1}   no phase {:>8.2}",
        label, alone, best, bg, off
    );
}
