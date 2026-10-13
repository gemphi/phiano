//! The story read by the trained readers (docs/150–154): the Joint FNO and
//! the phase-conjugate mirror, each a matched filter. Every position of the
//! story is read forward (which word comes next) and backwards (which word
//! came before, by the conjugate filter — never trained), and the two are
//! compared with what the story actually says.

use phiano::relate::{FnoWave, MirrorWave, RelationModel, Wave};
use phical::conjugate::Mirror;
use serde_json::{json, Value};

/// A reader's forward and backward reading at every position of the story.
fn read(name: &str, w: &mut dyn Wave, values: &[String], words: &[String]) -> Value {
    let rank = |c: &Option<Vec<phical::c64>>, truth: &String| -> Option<(usize, f64)> {
        let c = c.as_ref()?;
        let at = values.iter().position(|v| v == truth)?;
        let s = c[at].re;
        Some((c.iter().filter(|z| z.re > s).count(), s))
    };
    let best = |c: &Option<Vec<phical::c64>>| -> Vec<Value> {
        let Some(c) = c else { return Vec::new() };
        let mut v: Vec<(&String, f64)> = values.iter().zip(c.iter().map(|z| z.re)).collect();
        v.sort_by(|a, b| b.1.total_cmp(&a.1));
        v.into_iter().take(5).map(|(w, s)| json!([w, s])).collect()
    };
    let positions: Vec<Value> = (1..words.len())
        .map(|i| {
            let (prev, truth) = (&words[i - 1], &words[i]);
            let ahead = w.ahead(prev, 1, values);
            let back = w.back(truth, values);
            json!({
                "at": i,
                "after": prev,
                "truth": truth,
                "forward": { "rank": rank(&ahead, truth).map(|r| r.0), "best": best(&ahead) },
                "backward": { "rank": rank(&back, prev).map(|r| r.0), "best": best(&back) },
            })
        })
        .collect();
    let got = |f: &dyn Fn(&Value) -> Option<u64>| positions.iter().filter(|p| f(p) == Some(0)).count();
    json!({
        "reader": name,
        "positions": positions,
        "top1_forward": got(&|p| p["forward"]["rank"].as_u64()),
        "top1_backward": got(&|p| p["backward"]["rank"].as_u64()),
        "asked": positions.len(),
    })
}

pub fn waves(toks: &[String]) -> Value {
    let words: Vec<String> = toks.iter().filter(|t| t.chars().any(char::is_alphanumeric)).cloned().collect();
    let mut out = Vec::new();
    if let Ok(mut m) = RelationModel::load("data/runs/operator_r64_s7_dev10/epoch_32.relate") {
        let values: Vec<String> = m.names.iter().filter(|n| !n.starts_with('⟨')).cloned().collect();
        out.push(read("Joint FNO, 32 epochs", &mut FnoWave::new(&mut m), &values, &words));
    }
    if let Ok(m) = Mirror::load("data/runs/mirror_r64_s7_dev10/epoch_128.mirror") {
        let values = m.names.clone();
        out.push(read("mirror, 128 epochs", &mut MirrorWave::new(m, false), &values, &words));
    }
    json!({ "readers": out })
}
