//! Our own context tests, on the tree (docs/129): what ContextBench asks
//! of a Context Language Model, asked of a session kept as a tree of
//! conditions. A session is `session/b<block>/t<turn>.txt`, one turn per
//! file, filler from *Emma*'s paragraphs; needles and keys are told in
//! some turns and asked for at the end, after all the others.
//!
//! - **needle**: "the code for the vault is 4471." told once at a random
//!   turn; asked "the code for the vault is" after N turns;
//! - **kv**: K keys set once ("remember that alpha is seventeen.");
//!   asked "alpha is";
//! - **update**: the same keys set again later with another value; the
//!   latest must be read (phiecs's `Condition`: now over was — in the
//!   cast, the scope told later stands first among equals).
//!
//! The reading is `prose::Knowing` unchanged: cast, then `next` under the
//! path, the last word, the pair. No edit, no summary; the cost is rows.
//!
//! `--phasor R` casts by the field of phasors (docs/136) instead of by counts;
//! `--order attraction` ranks the scopes by cosine alone (no now over was).
//!
//! `cargo run --release --bin context -- [--turns 25,100,400,2000] [--needles 10] [--keys 20] [--seed 7] [--phasor 64]`

use phiano::prose::Knowing;
use std::time::Instant;

use phiano::lab::flag;

const PLACES: [&str; 20] = ["vault", "garden", "cellar", "attic", "stable", "library", "chapel", "mill", "orchard", "tower", "bridge", "well", "barn", "gate", "harbour", "lodge", "pantry", "study", "arbour", "kitchen"];
const KEYS: [&str; 20] = ["alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india", "juliet", "kilo", "lima", "mike", "november", "oscar", "papa", "quebec", "romeo", "sierra", "tango"];
const VALUES: [&str; 12] = ["seventeen", "forty", "ninety", "twelve", "eleven", "sixty", "thirty", "eighty", "fifty", "twenty", "seventy", "nineteen"];

struct Dice(u64);
impl Dice {
    fn below(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % n as u64) as usize
    }
}

/// Emma's paragraphs, each cut to 40 words, as the filler turns.
fn filler() -> Vec<String> {
    let mut out = Vec::new();
    for (_, rows) in phiano::prose::read_tree(std::path::Path::new("data/text/emma")) {
        let words: Vec<String> = rows.iter().filter(|r| r.kind == "has-word").map(|r| r.other.clone()).take(40).collect();
        if words.len() > 8 {
            out.push(words.join(" "));
        }
    }
    out
}

/// What a session told, as (name, value) pairs.
type Told = Vec<(String, String)>;

/// Writes a session of `turns` turns with `needles` codes and `keys` keys
/// (each key set twice when `update`); returns what was told, latest last.
fn session(dir: &std::path::Path, fill: &[String], turns: usize, needles: usize, keys: usize, update: bool, seed: u64) -> (Told, Told) {
    let _ = std::fs::remove_dir_all(dir);
    let mut d = Dice(seed | 1);
    let mut lines: Vec<String> = (0..turns).map(|i| fill[(i * 7 + d.below(fill.len())) % fill.len()].clone()).collect();
    let mut told_needles = Vec::new();
    let mut told_keys = Vec::new();
    for place in PLACES.iter().take(needles) {
        let code = 1000 + d.below(9000);
        let t = d.below(turns);
        lines[t] += &format!(" the code for the {place} is {code}.");
        told_needles.push((place.to_string(), code.to_string()));
    }
    for key in KEYS.iter().take(keys) {
        let v = VALUES[d.below(VALUES.len())];
        let t = d.below(turns);
        lines[t] += &format!(" remember that {key} is {v}.");
        let mut latest = v;
        if update {
            let v2 = VALUES[(VALUES.iter().position(|x| *x == v).unwrap() + 1 + d.below(VALUES.len() - 1)) % VALUES.len()];
            let t2 = t + 1 + d.below(turns - t);
            if t2 < turns {
                lines[t2] += &format!(" now {key} is {v2}.");
                latest = v2;
            }
        }
        told_keys.push((key.to_string(), latest.to_string()));
    }
    for (i, line) in lines.iter().enumerate() {
        let block = dir.join(format!("b{:03}", i / 100));
        std::fs::create_dir_all(&block).expect("block");
        std::fs::write(block.join(format!("t{:05}.txt", i + 1)), format!("{line}\n")).expect("turn");
    }
    (told_needles, told_keys)
}

/// The word read next after `prompt`: the top of the ranking.
fn answer(k: &Knowing, prompt: &str) -> String {
    let said = phiano::prose::tokens(prompt);
    k.next(&said).1.into_iter().next().map(|x| x.0).unwrap_or_default()
}

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    let turns: Vec<usize> = flag(&mut a, "--turns").unwrap_or_else(|| "25,100,400,2000".into()).split(',').filter_map(|s| s.trim().parse().ok()).collect();
    let needles: usize = flag(&mut a, "--needles").and_then(|s| s.parse().ok()).unwrap_or(10);
    let keys: usize = flag(&mut a, "--keys").and_then(|s| s.parse().ok()).unwrap_or(20);
    let seed: u64 = flag(&mut a, "--seed").and_then(|s| s.parse().ok()).unwrap_or(7);
    let phasor: Option<usize> = flag(&mut a, "--phasor").and_then(|s| s.parse().ok());
    let order = match flag(&mut a, "--order").as_deref() {
        Some("attraction") => phiano::relate::Order::Attraction,
        _ => phiano::relate::Order::Told,
    };
    let fill = filler();
    let dir = std::env::temp_dir().join("phiano_context").join("session");
    let mut md = String::from("| turns | words told | rows | learned in | needles found | keys read | updated keys read (latest) | read in |\n|---|---|---|---|---|---|---|---|\n");
    for &n in &turns {
        let mut cells = Vec::new();
        let (mut words, mut rows, mut learn_t, mut read_t) = (0usize, 0usize, 0.0, 0.0);
        for update in [false, true] {
            let (needs, kvs) = session(&dir, &fill, n, needles, keys, update, seed ^ n as u64);
            let t0 = Instant::now();
            let (mut k, told) = Knowing::learn(&dir);
            if let Some(r) = phasor {
                k = k.with_field(r);
                k.order = order;
            }
            k.within = phiano::relate::scopes("session", 0);
            learn_t = t0.elapsed().as_secs_f64();
            words = told.iter().map(|x| x.1).sum();
            rows = k.rows;
            let t1 = Instant::now();
            let found = needs.iter().filter(|(p, c)| answer(&k, &format!("the code for the {p} is")) == *c).count();
            let read = kvs.iter().filter(|(key, v)| answer(&k, &format!("{key} is")) == *v).count();
            read_t = t1.elapsed().as_secs_f64() / (needs.len() + kvs.len()) as f64;
            if !update {
                cells.push(format!("{found} of {}", needs.len()));
                cells.push(format!("{read} of {}", kvs.len()));
            } else {
                cells.push(format!("{read} of {}", kvs.len()));
                if std::env::var("CONTEXT_DEBUG").is_ok() {
                    for (key, v) in &kvs {
                        let got = answer(&k, &format!("{key} is"));
                        if got != *v {
                            eprintln!("{key}: wanted {v}, read {got}");
                        }
                    }
                }
            }
        }
        md += &format!("| {n} | {words} | {rows} | {learn_t:.2} s | {} | {} | {} | {:.0} ms |\n", cells[0], cells[1], cells[2], 1000.0 * read_t);
        println!("{n} turns done");
    }
    println!("\n{md}");
}
