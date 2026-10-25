//! Sections 6 and 7 of the report: what she composes, and the sixty-four operations.

use phiano::eval::Evaluator;
use phiano::facet::Facet;
use phiano::generate::{ContextWaveBuffer, Generator, OperatorGuide};
use phiano::learn::GenWeight;
use phiano::metrics::harness::Split;
use phiano::persona::World;
use phiano::metrics::baseline::kn::KneserNey;
use phiano::phinet::space::{FunctionSpace, Sides};
use phistd::hexagram::KING_WEN_MAP;
use phiano::tokenizer::Tokenizer;
use std::fmt::Write as _;

/// One composition arm: its name, whether the operator guides it, and its
/// novelty (hexagram, every how many tokens).
type Arm = (&'static str, bool, Option<(u8, usize)>);

/// §6 — short and long compositions under three arms, judged by Kneser-Ney.
pub fn compositions(md: &mut String, voice: &Facet, learned: &GenWeight, kn: &KneserNey, split: &Split) {
    let prompts = [
        "It is a truth universally",
        "Emma Woodhouse handsome clever and rich",
        "Captain Wentworth had",
        "Elinor and Marianne",
        "Mr Knightley said",
        "The evening passed",
        "She could not help",
        "Her mother was",
    ];
    let ev = Evaluator::new();
    writeln!(md, "\n## 6. What she composes\n").unwrap();
    writeln!(md, "Continuations of Austen-shaped prompts (none of which is a sentence of hers): **short** (12 tokens) and **long** (60 tokens), temperature 0.3, one fixed seed per prompt across arms. Arms: the learned scorer alone; the learned scorer with the operator; and the operator with **I Ching novelty** — every fourth token its prediction passes through hexagram ䷘ (id 0b101010, King Wen 25 *Wú Wàng*, \"Innocence\"), which keeps octaves 1, 3 and 5 of the prediction's spectrum and drops the rest. Each arm is judged by the Kneser-Ney model of Austen (lower = more like her), by the engine's evaluator, and by its distinct-token ratio.\n").unwrap();
    let novelty_hex: u8 = 0b10_1010;
    let arms: [Arm; 3] = [("learned scorer", false, None), ("learned + operator", true, None), ("learned + operator + I Ching novelty", true, Some((novelty_hex, 4)))];
    let mut summary: Vec<(String, usize, f64, f64, f64)> = Vec::new();
    for (len_name, max_tokens) in [("short", 12usize), ("long", 60usize)] {
        writeln!(md, "### {} compositions ({} tokens)\n", if len_name == "short" { "Short" } else { "Long" }, max_tokens).unwrap();
        for (arm, use_op, nov) in &arms {
            let (mut kn_sum, mut ev_sum, mut dist_sum, mut cnt) = (0.0, 0.0, 0.0, 0usize);
            writeln!(md, "**{arm}**\n").unwrap();
            for (i, prompt) in prompts.iter().enumerate() {
                let mut g = Generator::deterministic(max_tokens, 0.3, 500 + i as u64).with_weights(Some(learned.clone()));
                if *use_op {
                    if let Ok(mut guide) = OperatorGuide::load("data/austen_operator.fno", voice, Sides::Directed, 6) {
                        guide.novelty = *nov;
                        g = g.with_operator(guide);
                    }
                }
                let mut buf = ContextWaveBuffer::new(64);
                let text = g.generate(voice, &mut buf, prompt);
                let toks = Tokenizer::tokenize(&text);
                if toks.len() < 3 {
                    writeln!(md, "- *{prompt}* → (nothing)").unwrap();
                    continue;
                }
                let full = format!("{prompt} {text}");
                let ppl = kn.perplexity(std::slice::from_ref(&full));
                let e = ev.eval(voice, &full).overall;
                let uniq: std::collections::HashSet<&String> = toks.iter().collect();
                kn_sum += ppl.ln();
                ev_sum += e;
                dist_sum += uniq.len() as f64 / toks.len() as f64;
                cnt += 1;
                writeln!(md, "- *{prompt}* → {text}  \n  <small>KN {ppl:.0} · eval {e:.2} · distinct {:.2}</small>", uniq.len() as f64 / toks.len() as f64).unwrap();
            }
            if cnt > 0 {
                summary.push((format!("{len_name} · {arm}"), cnt, (kn_sum / cnt as f64).exp(), ev_sum / cnt as f64, dist_sum / cnt as f64));
            }
            writeln!(md).unwrap();
        }
    }
    writeln!(md, "### Summary\n\n| arm | n | KN perplexity (geo. mean) | evaluator | distinct |\n|---|---|---|---|---|").unwrap();
    for (a, n, k, e, d) in &summary {
        writeln!(md, "| {a} | {n} | {k:.1} | {e:.3} | {d:.3} |").unwrap();
    }
    writeln!(md, "\nFor scale: Austen's own held-out sentences read {:.1} under the same judge; the Rust Book generator read 12–14 on its corpus. A number far below the corpus's own is a copy; far above it is not her.\n", kn.perplexity(&split.valid)).unwrap();

}

/// §7 — hexagram readings of a few of her words.
pub fn operations(md: &mut String, space: &FunctionSpace, novelty_hex: u8) {
    writeln!(md, "## 7. The sixty-four operations\n").unwrap();
    writeln!(md, "`phistd::hexagram`: a hexagram's six lines gate the six octaves of a function's spectrum — line *j* yang passes octave *j*. ䷀ (111111) is the identity, ䷁ (000000) keeps only the mean; `changing_lines` walks between them. The hexagram a word *is* is its centroid sector (Q64). ReadingSet of five of Austen's words (octave energy, bottom line first):\n").unwrap();
    writeln!(md, "| word | Q64 id | King Wen | lines (octaves 0–5) | novelty of ䷘ |\n|---|---|---|---|---|").unwrap();
    for w in ["elinor", "emma", "love", "marriage", "said", "the"] {
        if let Some(&i) = space.index.get(w) {
            let f = &space.table()[i];
            let id = phistd::hexagram::hexagram_of(f);
            let l = phistd::hexagram::lines(f);
            writeln!(md, "| {w} | {id} | {} | {} | {:.3} |", KING_WEN_MAP[id as usize], l.iter().map(|v| format!("{v:.2}")).collect::<Vec<_>>().join(" "), phistd::hexagram::novelty(f, novelty_hex)).unwrap();
        }
    }
    writeln!(md, "\nWhat the gates are for is §6's third arm: novelty *on demand* in composition, exact, reversible, and named. What they are not (measured on the Rust Book, `bin/hexagram_eval`): a way to predict the next word's hexagram — the operator's predicted hexagram matches the true next word's 0.9% of the time, the King Wen successor 1.1%, both at or under chance (1.6%), while \"the most frequent hexagram, always\" gets 12.8% because centroid sectors are skewed. The sixty-four operations are a keyboard, not an oracle.\n").unwrap();
}

/// §3 — attribution on held-out paragraphs of four sentences.
pub fn attribution(md: &mut String, world: &World, pfacet: &Facet, held_by_author: &[(String, Vec<String>)]) {
    writeln!(md, "\n## 3. Who wrote this? Attribution on held-out paragraphs\n").unwrap();
    writeln!(md, "Held-out sentences grouped into paragraphs of four; `World::match_text` picks the persona with the highest likelihood. Chance is one in three.\n").unwrap();
    writeln!(md, "| true author | paragraphs | picked Austen | picked Chesterton | picked Carroll | accuracy | mean confidence |\n|---|---|---|---|---|---|---|").unwrap();
    let mut total = (0usize, 0usize);
    for (author, held) in held_by_author {
        let (mut n, mut ok, mut conf) = (0usize, 0usize, 0.0);
        let mut picks = std::collections::BTreeMap::new();
        for chunk in held.chunks(4).take(150) {
            let text = chunk.join(". ");
            if let Some(m) = world.match_text(pfacet, &text) {
                n += 1;
                *picks.entry(m.best_name.clone()).or_insert(0usize) += 1;
                if &m.best_name == author {
                    ok += 1;
                }
                conf += m.confidence;
            }
        }
        total.0 += ok;
        total.1 += n;
        writeln!(md, "| {} | {} | {} | {} | {} | {:.0}% | {:.2} |", author, n, picks.get("Jane Austen").unwrap_or(&0), picks.get("G. K. Chesterton").unwrap_or(&0), picks.get("Lewis Carroll").unwrap_or(&0), 100.0 * ok as f64 / n.max(1) as f64, conf / n.max(1) as f64).unwrap();
    }
    writeln!(md, "\nOverall {:.0}% ({} of {}).\n", 100.0 * total.0 as f64 / total.1.max(1) as f64, total.0, total.1).unwrap();
}
