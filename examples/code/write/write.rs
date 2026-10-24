//! Writing: tokens back to text, greedy writing, and a beam (docs/117–118).

use super::score;
use std::collections::HashSet;

/// Tokens back to text, close enough for `syn`.
pub fn render(toks: &[String]) -> String {
    let mut s = String::new();
    for (i, t) in toks.iter().enumerate() {
        let t = match t.as_str() {
            "⟨num⟩" => "1",
            "⟨str⟩" => "\"\"",
            "⟨char⟩" => "'c'",
            x => x,
        };
        let prev = if i > 0 { toks[i - 1].as_str() } else { "" };
        // Rust's two-character operators, split by the lexer, joined again.
        let joint = matches!((prev, t), ("-", ">") | ("=", ">") | (":", ":") | ("=", "=") | ("!", "=") | ("<", "=") | (">", "=") | ("&", "&") | ("|", "|") | (".", ".") | ("+", "=") | ("-", "=") | ("*", "=") | ("/", "=") | ("%", "=") | ("<", "<") | (">", ">") | ("&", "=") | ("|", "=") | ("^", "="));
        let glue = joint || matches!(t, ")" | "]" | ";" | "," | "." | "?") || matches!(prev, "(" | "[" | "." | "&" | "!") || (t == "(" && score::is_identifier(prev));
        if !s.is_empty() && !glue {
            s.push(' ');
        }
        s.push_str(t);
        if matches!(t, "{" | ";" | "}") {
            s.push('\n');
        }
    }
    s
}

/// Greedy writing from `prompt` until its braces close; a token that would
/// repeat a 4-gram already written is passed over (no loops).
pub fn write(prompt: &[u32], vocab: &[String], next: &dyn Fn(&[u32]) -> Vec<f64>) -> Vec<u32> {
    let mut h = prompt.to_vec();
    let mut depth: i32 = prompt.iter().map(|&t| (vocab[t as usize] == "{") as i32 - (vocab[t as usize] == "}") as i32).sum();
    let mut seen: HashSet<Vec<u32>> = h.windows(4).map(|w| w.to_vec()).collect();
    for _ in 0..160 {
        let p = next(&h);
        let mut order: Vec<usize> = (0..p.len()).collect();
        order.sort_by(|a, b| p[*b].total_cmp(&p[*a]));
        let pick = order.into_iter().find(|&t| {
            let mut w: Vec<u32> = h[h.len().saturating_sub(3)..].to_vec();
            w.push(t as u32);
            t != 0 && !seen.contains(&w)
        });
        let Some(t) = pick else { break };
        h.push(t as u32);
        if h.len() >= 4 {
            seen.insert(h[h.len() - 4..].to_vec());
        }
        depth += (vocab[t] == "{") as i32 - (vocab[t] == "}") as i32;
        if depth == 0 {
            break;
        }
    }
    h
}


/// Beam search: `width` partial bodies kept by mean log-probability; a body
/// is complete when its braces close. Returns the complete ones, best first.
pub fn beam(prompt: &[u32], vocab: &[String], next: &dyn Fn(&[u32]) -> Vec<f64>, width: usize) -> Vec<Vec<u32>> {
    let depth0: i32 = prompt.iter().map(|&t| (vocab[t as usize] == "{") as i32 - (vocab[t as usize] == "}") as i32).sum();
    let mut live: Vec<(Vec<u32>, f64, i32)> = vec![(prompt.to_vec(), 0.0, depth0)];
    let mut done: Vec<(Vec<u32>, f64)> = Vec::new();
    for step in 1..=90 {
        let mut grown: Vec<(Vec<u32>, f64, i32)> = Vec::new();
        for (h, lp, depth) in &live {
            let p = next(h);
            let mut order: Vec<usize> = (1..p.len()).collect();
            order.sort_by(|a, b| p[*b].total_cmp(&p[*a]));
            for &t in order.iter().take(width) {
                let mut g = h.clone();
                g.push(t as u32);
                let d = depth + (vocab[t] == "{") as i32 - (vocab[t] == "}") as i32;
                grown.push((g, lp + p[t].max(1e-300).ln(), d));
            }
        }
        grown.sort_by(|a, b| (b.1 / step as f64).total_cmp(&(a.1 / step as f64)));
        live.clear();
        for (g, lp, d) in grown {
            if d == 0 {
                done.push((g, lp / step as f64));
            } else if live.len() < width {
                live.push((g, lp, d));
            }
        }
        if live.is_empty() || done.len() >= 2 * width {
            break;
        }
    }
    done.sort_by(|a, b| b.1.total_cmp(&a.1));
    done.into_iter().map(|x| x.0).collect()
}

/// Whether Rust's compiler accepts the text as a library (types checked).
pub fn compiles(text: &str) -> bool {
    let dir = std::env::temp_dir().join(format!("code_write_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let f = dir.join("w.rs");
    if std::fs::write(&f, text).is_err() {
        return false;
    }
    std::process::Command::new("rustc")
        .args(["--edition", "2021", "--crate-type", "lib", "--emit=metadata", "-A", "warnings", "-o"])
        .arg(dir.join("w.rmeta"))
        .arg(&f)
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}
