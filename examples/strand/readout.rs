//! What the reading test (docs/145) prints: per suite and rule, nats per
//! held-out word by reader, how often the strand wins, and where the best
//! level falls against the level the strand supports; and the curve of
//! nats per word over the sixty-four levels.

use super::read::{Reading, READERS, RULES};
use phistd::codon::CODONS;
use std::fmt::Write;

/// The summary of a suite under both rules.
pub fn summary(name: &str, rs: &[Reading]) -> String {
    let held: usize = rs.iter().map(|r| r.held).sum();
    let m = rs.len().max(1) as f64;
    let mut out = String::new();
    writeln!(out, "{name}: {} texts · {:.0} tokens a text · {held} words read · {:.1} units supported", rs.len(), held as f64 / m, rs.iter().map(|r| r.supported as f64 / r.folds as f64).sum::<f64>() / m).ok();
    for (rule, rname) in RULES.iter().enumerate() {
        let per = |j: usize| rs.iter().map(|r| r.nats[rule][j]).sum::<f64>() / held.max(1) as f64;
        let line: Vec<String> = READERS.iter().enumerate().map(|(j, n)| format!("{n} {:.4} ({:+.4})", per(j), per(j) - per(1))).collect();
        writeln!(out, "  {rname}: {}", line.join(" · ")).ok();
        let wins = |a: usize, b: usize| rs.iter().filter(|r| r.nats[rule][a] < r.nats[rule][b]).count();
        writeln!(out, "    strand beats: labels {}/{n} · equal parts {}/{n} · whole {}/{n} · strand flat {}/{n}; strand flat beats labels {}/{n} · equal parts {}/{n}; labels beat whole {}/{n}", wins(6, 2), wins(6, 3), wins(6, 1), wins(6, 4), wins(4, 2), wins(4, 3), wins(2, 1), n = rs.len()).ok();
        let (mut finer, mut same, mut coarser, mut regret, mut best_units) = (0, 0, 0, 0.0, 0.0);
        for r in rs {
            let best = (0..CODONS).min_by(|a, b| r.levels[rule][*a].total_cmp(&r.levels[rule][*b])).unwrap_or(CODONS - 1);
            let supported = r.supported as f64 / r.folds as f64;
            let level = (0..CODONS).find(|k| r.units[*k] as f64 <= supported).unwrap_or(CODONS - 1);
            match best.cmp(&level) {
                std::cmp::Ordering::Less => finer += 1,
                std::cmp::Ordering::Equal => same += 1,
                std::cmp::Ordering::Greater => coarser += 1,
            }
            regret += (r.nats[rule][6] - r.levels[rule][best]) / r.held as f64;
            best_units += r.units[best] as f64;
        }
        writeln!(out, "    best level: finer than supported {finer} · same {same} · coarser {coarser}; units at best {:.1}; strand's regret {:.4} nats a word", best_units / m, regret / m).ok();
    }
    out
}

/// Nats a word at each level under both rules, averaged over the texts, with the mean units there.
pub fn curve(rs: &[Reading]) -> String {
    let mut out = String::from("k\tunits\twitten_bell\tcounted\n");
    let held = rs.iter().map(|r| r.held).sum::<usize>().max(1) as f64;
    for k in (0..CODONS).rev() {
        let u = rs.iter().map(|r| r.units[k] as f64).sum::<f64>() / rs.len().max(1) as f64;
        let at = |rule: usize| rs.iter().map(|r| r.levels[rule][k]).sum::<f64>() / held;
        writeln!(out, "{k}\t{u:.1}\t{:.5}\t{:.5}", at(0), at(1)).ok();
    }
    out
}

/// One line a text: its words read and nats per word by rule and reader.
pub fn texts(rs: &[Reading]) -> String {
    let mut out = String::from("held\tsupported");
    for rule in ["wb", "counted"] {
        for n in READERS {
            write!(out, "\t{rule}_{}", n.replace(' ', "_")).ok();
        }
    }
    out.push('\n');
    for r in rs {
        write!(out, "{}\t{:.1}", r.held, r.supported as f64 / r.folds as f64).ok();
        for rule in 0..2 {
            for j in 0..READERS.len() {
                write!(out, "\t{:.5}", r.nats[rule][j] / r.held as f64).ok();
            }
        }
        out.push('\n');
    }
    out
}
