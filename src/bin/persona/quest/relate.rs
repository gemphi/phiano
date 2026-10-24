//! One persona's relation model: trained on four fifths of its rows, read
//! on the rest against the prior, saved as `.relate`. The kinds the persona
//! file declares under `inherit` take their is-a parents' readings, in
//! training and in reading (docs/97 §1, docs/99).

use phiano::relate::{kind_token, RelationModel, Triple};
use phical::relate::{self, inherit_epoch, prior, score, score_inherit, Inherit};
use phistd::Adam;
use phical::{FnoConfig, Tie};
use std::path::Path;

fn held_out(r: &phiecs::quest::Row) -> bool {
    let h = phistd::mix64(phiano::phasor::fnv1a(&format!("{}\t{}\t{}", r.about, r.kind, r.other)));
    ((h >> 11) as f64 / (1u64 << 53) as f64) < 0.2
}

pub fn train(persona: &phiecs::persona::Persona, rows: &[phiecs::quest::Row], out: &Path) {
    let name = persona.name.as_str();
    // The persona's `relations` is the size knob (docs/100): 64 by default,
    // any value past it wired the same way.
    let r = persona.relations.max(4) as usize;
    let cfg = FnoConfig { space: r, width: 16, layers: 2, modes: (r / 4).min(16), margin: 0.3, lr: 5e-3, tie: Tie::Circle };
    let mut m = RelationModel::new(rows, None, cfg, 7);
    let (test, told): (Vec<_>, Vec<_>) = rows.iter().partition(|r| held_out(r));
    let told: Vec<Triple> = told.iter().filter_map(|r| m.triple(r)).collect();
    let test: Vec<Triple> = test.iter().filter_map(|r| m.triple(r)).collect();
    let all: Vec<Triple> = told.iter().chain(&test).copied().collect();
    let (pool, by_kind) = relate::pools(&all);
    let examples: Vec<_> = told.iter().map(relate::example).collect();
    let inh = m.index.get(&kind_token("is-a")).filter(|_| !persona.inherit.is_empty()).map(|&isa| {
        let kinds = persona.inherit.iter().filter_map(|k| m.index.get(&kind_token(k)).copied()).collect();
        Inherit::from_told(&told, isa, kinds, 1.0)
    });
    let (mut a, mut b) = (Adam::new(cfg.lr), Adam::new(cfg.lr * 0.5));
    for e in 0..60 {
        match &inh {
            Some(i) => {
                inherit_epoch(&mut m.joint, &told, i, &pool, &mut a, &mut b, e);
            }
            None => {
                m.joint.train_epoch(&examples, &pool, 32, &mut a, &mut b, e);
            }
        }
    }
    let p = prior(&told, &test, &pool, &by_kind);
    let s = match &inh {
        Some(i) => score_inherit(&mut m.joint, &test, i, &pool, &by_kind),
        None => score(&mut m.joint, &test, &pool, &by_kind),
    };
    println!(
        "  {name}: relation model, {r} relations a word, on {} rows{} · held out {} · MRR {:.3} (prior {:.3}) · within kind {:.3} (prior {:.3})",
        told.len(),
        if inh.is_some() { format!(", inheriting {}", persona.inherit.join(", ")) } else { String::new() },
        test.len(),
        s.mrr,
        p.mrr,
        s.within,
        p.within
    );
    if let Err(e) = m.save(&out.to_string_lossy()) {
        eprintln!("  could not save {}: {e}", out.display());
    }
}
