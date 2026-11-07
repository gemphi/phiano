//! The curriculum as a record, and the next instalment proposed from it
//! (Note D at the learner's scale; paper 28).
//!
//! Every instalment of the learner's training is one line of
//! `data/curriculum/instalments.tsv`: what it resumed from, which lever it
//! pulled, what text, the guide's share, and what it scored on the guide's
//! held-out test. `--record` appends the line after a run, so the record is
//! kept by the run and not by the writer; `--propose` reads the record and
//! says which lever to pull next: the one whose last application paid most,
//! measured as the perplexity it took off the instalment it resumed from.
//! A lever whose last application paid nothing is not proposed; a lever
//! whose gain halved against its previous application is proposed with that
//! said. The proposal is a reading of the record, not a prediction: the
//! engine follows the analysis it is given and anticipates nothing.
//!
//! The levers, as the record names them:
//!
//! | lever | what the run did |
//! |---|---|
//! | `guide` | the guide's training sentences told (again) |
//! | `text` | other text told, the guide not |
//! | `text+guide` | other text with the guide told beside it (`--guide-times`) |
//! | `epochs` | more epochs on the same text |
//! | `widen` | more channels |
//! | `forget` | the Adagrad accumulators scaled |

use std::path::PathBuf;

/// One instalment: one line of the record.
#[derive(Clone, Debug, PartialEq)]
pub struct Instalment {
    pub name: String,
    pub from: String,
    pub lever: String,
    pub text: String,
    pub guide_times: usize,
    pub epochs: usize,
    pub top1: f64,
    pub top5: f64,
    pub ppl: f64,
    pub doc: String,
}

/// The record: the crate's `data/curriculum/instalments.tsv`.
pub fn file() -> PathBuf {
    PathBuf::from(phiano::data::path("curriculum/instalments.tsv"))
}

/// The instalments, in the order recorded. A line that does not parse is
/// skipped, and said so on stderr.
pub fn read() -> Vec<Instalment> {
    phistd::lexfile::read(file())
        .iter()
        .filter_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            let parsed = (f.len() == 10).then(|| Instalment {
                name: f[0].into(),
                from: f[1].into(),
                lever: f[2].into(),
                text: f[3].into(),
                guide_times: f[4].parse().unwrap_or(0),
                epochs: f[5].parse().unwrap_or(0),
                top1: f[6].parse().unwrap_or(f64::NAN),
                top5: f[7].parse().unwrap_or(f64::NAN),
                ppl: f[8].parse().unwrap_or(f64::NAN),
                doc: f[9].into(),
            });
            if parsed.is_none() {
                eprintln!("instalments.tsv: a line with {} fields, not 10, skipped: {line}", f.len());
            }
            parsed
        })
        .collect()
}

/// Appends an instalment to the record. Returns whether it was new.
pub fn record(i: &Instalment) -> std::io::Result<bool> {
    let line = format!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{:.1}\t{:.1}\t{:.1}\t{}",
        i.name, i.from, i.lever, i.text, i.guide_times, i.epochs, i.top1, i.top5, i.ppl, i.doc
    );
    phistd::lexfile::append(file(), &line)
}

/// What an instalment took off the perplexity of the one it resumed from;
/// `None` for a first instalment or one whose parent is not in the record.
pub fn gain(i: &Instalment, all: &[Instalment]) -> Option<f64> {
    let parent = all.iter().find(|p| p.name == i.from)?;
    (!i.ppl.is_nan() && !parent.ppl.is_nan()).then(|| parent.ppl - i.ppl)
}

/// The record read back as a table, and the proposal.
pub fn propose(all: &[Instalment]) -> String {
    let mut out = String::from("| instalment | from | lever | text | guide × | top-1 | top-5 | ppl | gain | doc |\n|---|---|---|---|---|---|---|---|---|---|\n");
    for i in all {
        let g = gain(i, all).map_or("—".to_string(), |g| format!("{g:+.1}"));
        let pct = |x: f64| match x.is_nan() {
            true => "—".to_string(),
            false => format!("{x:.1}%"),
        };
        let ppl = match i.ppl.is_nan() {
            true => "—".to_string(),
            false => format!("{:.1}", i.ppl),
        };
        out += &format!("| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n", i.name, i.from, i.lever, i.text, i.guide_times, pct(i.top1), pct(i.top5), ppl, g, i.doc);
    }
    // Each lever's last application and the one before it.
    let mut levers: Vec<&str> = all.iter().map(|i| i.lever.as_str()).collect();
    levers.dedup();
    levers.sort_unstable();
    levers.dedup();
    let mut ranked: Vec<(&str, f64, Option<f64>)> = levers
        .iter()
        .filter_map(|lever| {
            let gains: Vec<f64> = all.iter().filter(|i| i.lever == *lever).filter_map(|i| gain(i, all)).collect();
            let last = *gains.last()?;
            let before = gains.len().checked_sub(2).map(|k| gains[k]);
            Some((*lever, last, before))
        })
        .collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    out += "\n| lever | last gain | the one before |\n|---|---|---|\n";
    for (lever, last, before) in &ranked {
        out += &format!("| {lever} | {last:+.1} | {} |\n", before.map_or("—".into(), |b| format!("{b:+.1}")));
    }
    let Some((lever, last, before)) = ranked.first().filter(|(_, g, _)| *g > 0.0) else {
        return out + "\nproposal: no lever's last application paid; the record proposes nothing — the next move is a new lever, not a repeat.\n";
    };
    let newest = all.last().map(|i| i.name.as_str()).unwrap_or("-");
    let mut why = format!("its last application took {last:.1} off the perplexity");
    if let Some(b) = before {
        if *last < b / 2.0 {
            why += &format!(", though that is less than half of the {b:.1} before it: the lever is paying less each time, and the guide's share is the first thing to raise beside it");
        }
    }
    out + &format!("\nproposal: pull `{lever}` again, resuming `{newest}` — {why}.\n")
}
