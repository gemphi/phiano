//! The transcript page's rendered body (R3.4, R3.10, R3.12).
//!
//! The page opens with the two reported names — the Oracle's own and the
//! Checker's — and a totals line, then one section per ask carrying every
//! recorded field: the index, the held-row count, the empty flag or the
//! rejection, the counts parsed, refused and told, and each claim as the
//! row-shaped line task 18.3 copies a false claim from verbatim.

use super::{Recorded, Recording};
use phiecs::quest::{Claim, Standing};
use phiecs::quest::Standing as S;
use crate::seam::Rejected;

/// Render the whole transcript: header, totals, one section per ask.
pub(super) fn render(rec: &Recording) -> String {
    let mut s = String::new();
    s.push_str("# oracle transcript\n\n");
    s.push_str(&format!("- oracle: `{}`\n", esc(rec.oracle())));
    s.push_str(&format!("- checker: `{}`\n", esc(rec.checker())));
    let (parsed, refused, told) = rec.totals();
    s.push_str(&format!(
        "- asks {}; claims parsed {parsed}; claims refused {refused}; rows told {told}\n\n",
        rec.asks().len()
    ));
    for ask in rec.asks() {
        s.push_str(&ask_section(ask));
    }
    s
}

/// One ask's markdown section: the index and held count, the empty flag or
/// the rejection, and the parsed, refused and told claims with their classes.
fn ask_section(ask: &Recorded) -> String {
    let mut s = format!("## ask {} (held {})\n\n", ask.index, ask.held);
    s.push_str(&format!("- prompt: `{}`\n", esc(&one_line(&ask.prompt))));
    if ask.empty {
        s.push_str("- empty: yes\n");
    }
    if let Some(why) = &ask.rejection {
        s.push_str(&format!("- rejected: {}\n", reject_word(why)));
    }
    s.push_str(&format!(
        "- parsed {}; refused {}; told {}\n",
        ask.parsed_count(),
        ask.refused_count(),
        ask.told_count()
    ));
    for claim in &ask.told {
        s.push_str(&format!("  - told: {}\n", claim_line(claim)));
    }
    for (claim, standing) in &ask.refused {
        s.push_str(&format!("  - refused ({}): {}\n", standing_word(standing), claim_line(claim)));
    }
    s.push('\n');
    s
}

/// A claim as one row-shaped line, so task 18.3 can copy a false claim from
/// the page verbatim.
fn claim_line(c: &Claim) -> String {
    esc(&format!("{} | {} | {}", c.kind, c.other, c.weight))
}

/// The class a Checker gave, as one word for the page and its reader.
fn standing_word(s: &Standing) -> &'static str {
    match s {
        S::Unreadable { .. } => "unreadable",
        S::Degenerate { .. } => "degenerate",
        S::Contradicting { .. } => "contradicting",
        S::Unsupported => "unsupported",
        S::Consistent => "consistent",
    }
}

/// A rejection as one word for the page.
fn reject_word(r: &Rejected) -> &'static str {
    match r {
        Rejected::AnsiEscape { .. } => "ansi-escape",
        Rejected::TooLong { .. } => "too-long",
        Rejected::TimedOut { .. } => "timed-out",
        Rejected::Exited { .. } => "exited",
    }
}

/// A seed turned into a page-name slug: lower case, letters, digits and dashes.
pub(super) fn slug(s: &str) -> String {
    let out: String = s
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    out.trim_matches('-').to_string()
}

/// A prompt folded to one line so a multi-line prompt does not break the page.
fn one_line(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Escape a cell's backtick-breaking characters so a value never splits a row.
fn esc(s: &str) -> String {
    s.replace('`', "'").replace('\n', " ")
}
