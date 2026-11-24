//! The curriculum as a record, and the next instalment proposed from it
//! (Note D at the learner's scale; paper 28).
//!
//! Every instalment of the learner's training is one line of
//! `data/curriculum/instalments.tsv`: what it resumed from, which lever it
//! pulled, what text, the guide's share, and what it scored on the guide's
//! held-out test, and since docs/222 on the never-told register too (`held`:
//! perplexity on the first `--test-more` file; docs/217). `--record` appends the line after a run, so the record is
//! kept by the run and not by the writer; `--propose` reads the record and
//! says which lever to pull next: the one whose last application paid most,
//! measured as the perplexity it took off the instalment it resumed from.
//! A lever whose last application paid nothing is not proposed; a lever
//! whose gain halved against its previous application is proposed with that
//! said. Gains are compared at a tenth, and two levers level there are
//! ordered by the never-told register (docs/230), said too. A lever whose
//! last pull paid one register with the other - the guide's gain smaller
//! than what the never-told register lost, or the other way round - is not
//! proposed, and the proposal says so (docs/237: the guide alone, with the
//! stride on, took 2.2 off its own register and put 22 on the other). The proposal is a reading of the record, not a prediction: the
//! engine follows the analysis it is given and anticipates nothing.
//!
//! The levers are named in `data/curriculum/lever.tsv`, each with the lever
//! it is read as: a lever renamed when the learner took it over keeps one
//! series under the new name (`forget` by flag is `stride`'s history since
//! docs/234), so the table shows the record's name and the proposer reads
//! the series. A lever read as `!` is not a lever to pull: either a change
//! of measure (`reader`, docs/237: the same learner told nothing, scored on
//! the guide read anew) or the learner's own state, which every checkpoint
//! carries and no run can pull (`stride`, and `forget` before it, docs/238).
//! Its rows are kept, so the next row's gain is read against them, and it
//! is never proposed.

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
    /// Perplexity on the never-told register (docs/217); NaN when not scored.
    pub held: f64,
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
            let parsed = (f.len() == 10 || f.len() == 11).then(|| Instalment {
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
                held: f.get(10).and_then(|h| h.parse().ok()).unwrap_or(f64::NAN),
            });
            if parsed.is_none() {
                eprintln!("instalments.tsv: a line with {} fields, not 10 or 11, skipped: {line}", f.len());
            }
            parsed
        })
        .collect()
}

/// Appends an instalment to the record. Returns whether it was new.
pub fn record(i: &Instalment) -> std::io::Result<bool> {
    let line = format!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{:.1}\t{:.1}\t{:.1}\t{}\t{:.1}",
        i.name, i.from, i.lever, i.text, i.guide_times, i.epochs, i.top1, i.top5, i.ppl, i.doc, i.held
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
    let mut out = String::from("| instalment | from | lever | text | guide × | top-1 | top-5 | ppl | gain | never-told | doc |\n|---|---|---|---|---|---|---|---|---|---|---|\n");
    for i in all {
        let g = gain(i, all).map_or("—".to_string(), |g| format!("{g:+.1}"));
        let pct = |x: f64| match x.is_nan() {
            true => "—".to_string(),
            false => format!("{x:.1}%"),
        };
        let num = |x: f64| match x.is_nan() {
            true => "—".to_string(),
            false => format!("{x:.1}"),
        };
        out += &format!("| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n", i.name, i.from, i.lever, i.text, i.guide_times, pct(i.top1), pct(i.top5), num(i.ppl), g, num(i.held), i.doc);
    }
    // Each lever's last application and the one before it, a lever's
    // series read under the name `lever.tsv` gives it.
    let mut levers: Vec<String> = all.iter().map(|i| read_as(&i.lever)).filter(|l| l != "!").collect();
    levers.sort_unstable();
    levers.dedup();
    let mut ranked: Vec<(&str, f64, Option<f64>)> = levers
        .iter()
        .filter_map(|lever| {
            let gains: Vec<f64> = all.iter().filter(|i| read_as(&i.lever) == *lever).filter_map(|i| gain(i, all)).collect();
            let last = *gains.last()?;
            let before = gains.len().checked_sub(2).map(|k| gains[k]);
            Some((lever.as_str(), last, before))
        })
        .collect();
    // Gains are compared at the record's resolution, a tenth. A tie on the
    // guide's register is broken by the never-told one (docs/230): the lever
    // whose last pull moved both registers wins, not the first by name.
    ranked.sort_by(|a, b| tenths(b.1).cmp(&tenths(a.1)).then_with(|| tenths(held_gain(b.0, all)).cmp(&tenths(held_gain(a.0, all)))));
    out += "\n| lever | last gain | the one before |\n|---|---|---|\n";
    for (lever, last, before) in &ranked {
        out += &format!("| {lever} | {last:+.1} | {} |\n", before.map_or("—".into(), |b| format!("{b:+.1}")));
    }
    // A lever that paid one register with the other is not proposed: the
    // sum of what its last pull took off both must be positive (docs/237).
    let (paid, robbed): (Vec<_>, Vec<_>) = ranked.iter().partition(|(l, g, _)| {
        let h = held_gain(l, all);
        h.is_nan() || g + h > 0.0
    });
    for (l, g, _) in &robbed {
        out += &format!("\nnot proposed: `{l}` paid the guide's register {g:+.1} with the never-told one ({:+.1}).", held_gain(l, all));
    }
    let Some((lever, last, before)) = paid.first().filter(|(_, g, _)| *g > 0.0) else {
        return out + "\nproposal: no lever's last application paid; the record proposes nothing — the next move is a new lever, not a repeat.\n";
    };
    let newest = all.last().map(|i| i.name.as_str()).unwrap_or("-");
    let mut why = format!("its last application took {last:.1} off the perplexity");
    if let Some(b) = before {
        if *last < b / 2.0 {
            why += &format!(", though that is less than half of the {b:.1} before it: the lever is paying less each time, and the guide's share is the first thing to raise beside it");
        }
    }
    let tied: Vec<&str> = paid.iter().skip(1).filter(|(_, g, _)| tenths(*g) == tenths(*last)).map(|(l, _, _)| *l).collect();
    if let Some(other) = tied.first() {
        why += &format!(", level with `{other}` there and ahead of it on the never-told register ({:+.1} beside {:+.1})", held_gain(lever, all), held_gain(other, all));
    }
    out += &format!("\nproposal: pull `{lever}` again, resuming `{newest}` — {why}.\n");
    out + &registers(all)
}

/// A gain at the record's resolution, a tenth: what the table shows is what
/// the proposer compares.
fn tenths(x: f64) -> i64 {
    match x.is_nan() {
        true => i64::MIN,
        false => (x * 10.0).round() as i64,
    }
}

/// The lever a record's lever is read as (`data/curriculum/lever.tsv`):
/// itself unless the file names another, as it does for `forget`, whose
/// rows are `stride`'s history since the checkpoint carries the stride
/// (docs/234).
fn read_as(lever: &str) -> String {
    phistd::lexfile::cached(phiano::data::path("curriculum/lever.tsv"))
        .iter()
        .find_map(|line| {
            let f: Vec<&str> = line.split('\t').collect();
            (f.len() >= 2 && f[0] == lever && f[1] != "-").then(|| f[1].to_string())
        })
        .unwrap_or_else(|| lever.to_string())
}

/// What a lever's last scored application took off the never-told register
/// (docs/217); NaN when neither it nor its parent was scored there.
fn held_gain(lever: &str, all: &[Instalment]) -> f64 {
    all.iter()
        .rev()
        .filter(|i| read_as(&i.lever) == lever)
        .find_map(|i| {
            let parent = all.iter().find(|p| p.name == i.from)?;
            let h = parent.held - i.held;
            (!h.is_nan()).then_some(h)
        })
        .unwrap_or(f64::NAN)
}

/// Whether the newest instalment moved the never-told register with the
/// guide's or against it (docs/217, 221): said when both it and the one it
/// resumed from were scored on it.
fn registers(all: &[Instalment]) -> String {
    let Some(i) = all.last() else { return String::new() };
    let Some(parent) = all.iter().find(|p| p.name == i.from) else { return String::new() };
    let (Some(g), held) = (gain(i, all), parent.held - i.held) else { return String::new() };
    if held.is_nan() {
        return String::new();
    }
    let how = match (g > 0.0) == (held > 0.0) {
        true => "with",
        false => "against",
    };
    format!("registers: `{}` moved the never-told register {how} the guide's ({held:+.1} beside {g:+.1}).\n", i.name)
}

#[cfg(test)]
mod tests {
    use super::{propose, Instalment};

    fn row(name: &str, from: &str, lever: &str, ppl: f64, held: f64) -> Instalment {
        Instalment { name: name.into(), from: from.into(), lever: lever.into(), text: "-".into(), guide_times: 1, epochs: 8, top1: 20.0, top5: 40.0, ppl, doc: "0".into(), held }
    }

    #[test]
    fn a_lever_that_paid_one_register_with_the_other_is_not_proposed() {
        let all = vec![
            row("a", "-", "guide", 80.0, 100.0),
            row("b", "a", "text+guide", 79.0, 99.0),
            row("c", "b", "guide", 77.0, 120.0),
        ];
        let out = propose(&all);
        assert!(out.contains("not proposed: `guide` paid the guide's register +2.0 with the never-told one (-21.0)"), "{out}");
        assert!(out.contains("proposal: pull `text+guide` again"), "{out}");
    }

    /// The learner's state is not a lever: `stride` is carried in every
    /// checkpoint (docs/235), so the record must not propose pulling it
    /// however well its rows read (docs/238).
    #[test]
    fn the_learners_own_state_is_not_proposed() {
        let all = vec![
            row("a", "-", "guide", 80.0, 100.0),
            row("b", "a", "stride", 76.0, 96.0),
            row("c", "b", "text+guide", 75.9, 95.9),
        ];
        let out = propose(&all);
        assert!(!out.contains("pull `stride`"), "{out}");
        assert!(!out.contains("pull `forget`"), "{out}");
        assert!(out.contains("proposal: pull `text+guide` again"), "{out}");
    }

    #[test]
    fn a_change_of_measure_is_kept_but_never_proposed() {
        let all = vec![
            row("a", "-", "guide", 80.0, 100.0),
            row("a_words", "a", "reader", 70.0, 98.0),
            row("b", "a_words", "text+guide", 69.5, 97.5),
        ];
        let out = propose(&all);
        assert!(!out.contains("pull `reader`"), "{out}");
        assert!(out.contains("proposal: pull `text+guide` again"), "{out}");
    }
}
