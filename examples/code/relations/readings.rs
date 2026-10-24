//! What the inherited reading says a held-out function calls, uses and
//! returns (docs/124): the operator read with the function's words.

use super::newfn::Trained;
use phiano::relate::{kind_token, Triple};

fn context(table: &[phistd::Function], about: usize, kind: usize) -> phistd::Function {
    let mut f: phistd::Function = table[about].iter().zip(&table[kind]).map(|(x, y)| x + y).collect();
    phistd::normalise(&mut f);
    f
}

/// What the inherited reading says a held-out function `calls` and
/// `uses-op`: the top `n` of the kind's pool.
pub fn readings(t: &mut Trained, names: &[&str], n: usize) -> String {
    let Trained { m, inh, by_kind, .. } = t;
    let table = m.joint.functions();
    let mut s = String::new();
    for name in names {
        let Some(&about) = m.index.get(*name) else { continue };
        for kind in ["calls", "uses-op", "returns"] {
            let Some(&k) = m.index.get(&kind_token(kind)) else { continue };
            let row = Triple { about, kind: k, other: 0 };
            let mut ctx = vec![context(&table, about, k)];
            ctx.extend(inh.of(&row).iter().map(|&p| context(&table, p, k)));
            let ys = m.joint.fno.apply(&ctx);
            let lambda = inh.lambda / (ctx.len() - 1).max(1) as f64;
            let pred: phistd::Function = (0..ys[0].len()).map(|i| ys[0][i] + ys[1..].iter().map(|y| y[i]).sum::<phistd::c64>() * lambda).collect();
            let mut ranked: Vec<(f64, usize)> = by_kind.get(&k).into_iter().flatten().map(|&v| (phistd::cosine(&pred, &table[v]), v)).collect();
            ranked.sort_by(|a, b| b.0.total_cmp(&a.0));
            let words: Vec<String> = inh.of(&row).iter().map(|&p| m.names[p].clone()).collect();
            s += &format!("{name} ({}) · {kind} ⇒ {}\n", words.join(" "), ranked.iter().take(n).map(|(c, v)| format!("{} ({c:.2})", m.names[*v])).collect::<Vec<_>>().join(", "));
        }
    }
    s
}
