//! A persona called Jane Austen, and what she can compose.
//! `cargo run --release --bin austen [out.md]`
//!
//! Reads `data/corpora/austen/*.txt` (Project Gutenberg texts) and the
//! contrast authors in `data/corpora/contrast/`, trains a two-sided lexicon
//! on all of them, builds personas, measures attribution ("who wrote this?")
//! on held-out paragraphs, trains a directed operator and fits the learned
//! scorer on Austen, then composes short and long continuations of
//! Austen-shaped prompts — plain, with the operator, and with I Ching
//! novelty — each judged by a Kneser-Ney model of Austen the composer never
//! saw. Everything is printed as Markdown so it can be set as a PDF.

use phiano::config::LEARNING_RATE;
use phiecs::phasor::facet::Facet;
use phiano::generate::OperatorGuide;
use phiano::learn::{corpus_positions, GenWeight, Tuner, FEATURE_NAMES};
use phient::metrics::harness::{Harness, Split};
use phient::metrics::baseline::kn::KneserNey;
use phiano::persona::Styles;
use phient::net::space::{examples, FunctionSpace, Sides, WordFunction};
use phistd::Adam;
use phical::{Fno, FnoConfig};
use phistd::hexagram::KING_WEN_MAP;
use phiecs::phasor::facet::storage::Storage;
use phiano::tokenizer::Tokenizer;
use phient::train::trainer::Trainer;
use phical::rank;
use std::fmt::Write as _;


mod sections;
use phiano::lab as util;
use util::{chrono_free_date, dir_sentences};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out_path = args.get(1).cloned().unwrap_or_else(|| "docs/papers/15_JANE_AUSTEN_PERSONA.md".to_string());
    let mut md = String::new();
    let t0 = std::time::Instant::now();

    // ── corpora ──
    let austen = dir_sentences("data/corpora/austen");
    let contrast = dir_sentences("data/corpora/contrast");
    let austen_all: Vec<String> = austen.iter().flat_map(|(_, s)| s.clone()).collect();
    let mut everything: Vec<String> = austen_all.clone();
    for (_, s) in &contrast {
        everything.extend(s.iter().cloned());
    }
    let split = Harness::split(austen_all.clone(), 7);
    let train_all: Vec<String> = {
        let mut v = split.train.clone();
        for (_, s) in &contrast {
            v.extend(s.iter().cloned());
        }
        v
    };
    let lex_split = Split { train: train_all.clone(), valid: split.valid.clone(), test: split.test.clone() };
    let facet = Harness::train_ranking_only(&lex_split, &Trainer::new(LEARNING_RATE).with_seed(0), 3);
    Storage::save(&facet, "data/austen.chroma").expect("save");
    let facet = Storage::load("data/austen.chroma").expect("load");
    let kn = KneserNey::train(&split.train);

    writeln!(md, "---\nid: jane-austen-persona\ntitle: \"A Persona Called Jane Austen\"\ndescription: What the engine learns from three novels, and what it can compose in her voice — short and long, measured.\nsidebar_position: 15\nbadge: report\n---\n").unwrap();
    writeln!(md, "# A Persona Called Jane Austen\n\n**Phiano Project** · `bin/austen` · generated {}\n", chrono_free_date()).unwrap();
    writeln!(md, "> **The paradigm.** In Phient's world, a word *w* is a seed: what it relates to, under what conditions *c*, grows from it. A persona is which conditions a writer puts words under, aggregated — and Jane Austen is three novels' worth of those conditions.\n").unwrap();
    writeln!(md, "## 1. The corpus\n").unwrap();
    writeln!(md, "| text | sentences (4–60 words) | words |\n|---|---|---|").unwrap();
    for (n, s) in &austen {
        writeln!(md, "| Austen — {} | {} | {} |", n.replace('_', " "), s.len(), s.iter().map(|x| Tokenizer::tokenize(x).len()).sum::<usize>()).unwrap();
    }
    for (n, s) in &contrast {
        writeln!(md, "| contrast — {} | {} | {} |", n, s.len(), s.iter().map(|x| Tokenizer::tokenize(x).len()).sum::<usize>()).unwrap();
    }
    writeln!(md, "\nProject Gutenberg editions via the NLTK `gutenberg` package (`data/corpora/austen/SOURCES.md`). Austen split 80/10/10 by sentence hash (seed 7); the lexicon is trained on the Austen training split plus every contrast author, three passes, two-sided (format v6): **{} words, {} with relation evidence, {} marker channels**, saved and reloaded as `data/austen.chroma`. Kneser-Ney trigram on the Austen training split — the judge for everything below — reads **{:.1}** on Austen's held-out sentences.\n", facet.vocabulary_size(), facet.evidenced_words(), facet.relations.markers(), kn.perplexity(&split.valid)).unwrap();

    writeln!(md, "The markers the corpus chose as relation channels 8–31: {}.\n", (8..8 + facet.relations.markers()).map(|k| format!("*{}*", facet.relation_name(k))).collect::<Vec<_>>().join(", ")).unwrap();

    // ── personas ──
    let mut world = Styles::new();
    let mut pfacet = facet.clone();
    let persona_trainer = Trainer::new(LEARNING_RATE).with_seed(1);
    let sample = |v: &[String], n: usize, seed: u64| -> Vec<String> {
        let mut idx: Vec<usize> = (0..v.len()).collect();
        // deterministic shuffle
        let mut s = seed | 1;
        for i in (1..idx.len()).rev() {
            s ^= s << 13; s ^= s >> 7; s ^= s << 17;
            let j = (s % (i as u64 + 1)) as usize;
            idx.swap(i, j);
        }
        idx.into_iter().take(n).map(|i| v[i].clone()).collect()
    };
    let austen_examples = sample(&split.train, 600, 11);
    world.add_persona("Jane Austen", &austen_examples, &mut pfacet, &persona_trainer);
    let mut held_by_author: Vec<(String, Vec<String>)> = vec![("Jane Austen".into(), split.test.clone())];
    // Aggregate each contrast author's sentences before splitting, so a
    // persona is built from all of that author's books at once.
    let mut by_author: std::collections::BTreeMap<&str, Vec<String>> = std::collections::BTreeMap::new();
    for (n, s) in &contrast {
        let author = if n.starts_with("chesterton") { "G. K. Chesterton" } else { "Lewis Carroll" };
        by_author.entry(author).or_default().extend(s.iter().cloned());
    }
    for (author, sents) in &by_author {
        let sp = Harness::split(sents.clone(), 7);
        world.add_persona(author, &sample(&sp.train, 600, 13), &mut pfacet, &persona_trainer);
        held_by_author.push(((*author).to_string(), sp.test.clone()));
    }
    writeln!(md, "## 2. The persona\n").unwrap();
    writeln!(md, "`Style::from_examples` on 600 sampled Austen sentences (and 600 each for the rival personas). A persona is a histogram over the 64 sectors of the relation circle; its entropy is its breadth; `PersonalityMapper` reads temperament off the colour bands; its Q64 name is the sector of its centroid.\n").unwrap();
    writeln!(md, "| persona | samples | mean length | entropy | Q64 (King Wen) | temperament | dominant sectors |\n|---|---|---|---|---|---|---|").unwrap();
    let mut names: Vec<&String> = world.personas.keys().collect();
    names.sort();
    for n in &names {
        let p = &world.personas[*n];
        let fp = &p.fingerprint;
        let q = fp.q64();
        writeln!(md, "| {} | {} | {:.1} | {:.3} | {} ({}) | {} | {} |", n, fp.sample_count, fp.avg_length, fp.diversity, q, KING_WEN_MAP[q as usize], fp.personality_traits().join(", "),
            fp.dominant_sectors(5).iter().map(|(s, w)| format!("{s}:{w:.3}")).collect::<Vec<_>>().join(" ")).unwrap();
    }
    if let Some(c) = world.compare("Jane Austen", "G. K. Chesterton") {
        writeln!(md, "\nAusten vs Chesterton: similarity {:.3}; the sectors that most separate them: {}.", c.similarity, c.differences.iter().take(6).map(|(s, d)| format!("{s} ({d:+.3})")).collect::<Vec<_>>().join(", ")).unwrap();
    }
    for t in [0.05, 0.2, 0.5] {
        let a = &world.personas["Jane Austen"].fingerprint;
        let ch = world.personas.get("G. K. Chesterton").map(|p| a.overlap_after(&p.fingerprint, t));
        let lc = world.personas.get("Lewis Carroll").map(|p| a.overlap_after(&p.fingerprint, t));
        writeln!(md, "Overlap after heat flow t = {t}: Austen–Chesterton {:.3}, Austen–Carroll {:.3}.", ch.unwrap_or(f64::NAN), lc.unwrap_or(f64::NAN)).unwrap();
    }

    sections::attribution(&mut md, &world, &pfacet, &held_by_author);

    // ── her voice: a lexicon of Austen alone ──
    // The attribution lexicon above holds every author, so a composer reading
    // its n-gram tables could (and did) borrow Carroll's white rabbit. What she
    // composes from is trained on her training split only.
    let voice = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 3);
    Storage::save(&voice, "data/austen_voice.chroma").expect("save voice");
    let voice: Facet = Storage::load("data/austen_voice.chroma").expect("load voice");
    writeln!(md, "> From here on the machine reads a second lexicon, **`data/austen_voice.chroma`**, trained on Austen's training split alone ({} words, {} with relation evidence) — the attribution lexicon above holds every author, and a composer reading its tables would borrow from Carroll and Chesterton.\n", voice.vocabulary_size(), voice.evidenced_words()).unwrap();

    // ── operator on Austen ──
    let space = FunctionSpace::from_facet_sides(&voice, Sides::Directed);
    let pool = space.pool(20);
    let train_sent: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let valid_sent: Vec<Vec<String>> = split.valid.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let (train_ex, _) = examples(&space, &train_sent, 6, &pool, u64::MAX);
    let (mut valid_ex, _) = examples(&space, &valid_sent, 6, &pool, u64::MAX);
    valid_ex.truncate(2000);
    let valid_in: Vec<WordFunction> = valid_ex.iter().map(|e| e.input.clone()).collect();
    let chance = { let n = pool.len() as f64; (1..=pool.len()).map(|k| 1.0 / k as f64).sum::<f64>() / n };
    let (id_mrr, _) = rank(space.table(), &valid_in, &valid_ex, &pool);
    let cfg = FnoConfig { space: space.space(), width: 8, layers: 2, modes: 12, margin: 0.2, lr: 2e-3, tie: phical::Tie::Circle };
    let mut fno = Fno::new(cfg, 7);
    let mut adam = Adam::new(cfg.lr);
    let (mut best, mut bytes) = (f64::NEG_INFINITY, fno.to_bytes());
    for e in 0..4 {
        fno.train_epoch(space.table(), &train_ex, &pool, 64, &mut adam, e as u64);
        let (m, _) = rank(space.table(), &fno.apply(&valid_in), &valid_ex, &pool);
        if m > best { best = m; bytes = fno.to_bytes(); }
    }
    std::fs::write("data/austen_operator.fno", &bytes).expect("write operator");
    writeln!(md, "## 4. The operator, on Austen\n").unwrap();
    writeln!(md, "A directed Fourier operator (8,360 parameters, hinge, 4 epochs, early-stopped) trained on Austen's training split, ranking the next word among the {} words seen ≥ 20 times. Chance MRR {:.4}. The lexicon's own directed reading, no operator: **{:.4} ({:.1}× chance)**; with the operator: **{:.4} ({:.1}× chance)**. Saved as `data/austen_operator.fno`.\n", pool.len(), chance, id_mrr, id_mrr / chance, best, best / chance).unwrap();

    // ── learned scorer on Austen ──
    let mut guide_opt = OperatorGuide::load("data/austen_operator.fno", &voice, Sides::Directed, 6).ok();
    let (fit, _, _) = corpus_positions(&voice, &split.valid, 30_000, &mut guide_opt);
    let (held, _, _) = corpus_positions(&voice, &split.test, 30_000, &mut guide_opt);
    let (learned, _) = Tuner::default().fit(GenWeight::counts_only().w, &fit, &held);
    learned.save("data/learned_austen.json").expect("write learned");
    writeln!(md, "## 5. The learning machine, on Austen\n").unwrap();
    writeln!(md, "The generator's candidate scorer fit to Austen (validation split), evaluated on her test split: perplexity among the n-gram's candidates — counts only **{:.2}**, learned **{:.2}**. The weights are the value audit for this author:\n", Tuner::nll(&GenWeight::counts_only().w, &held).exp(), learned.heldout_nll.exp()).unwrap();
    writeln!(md, "| feature | weight |\n|---|---|").unwrap();
    for (n, w) in FEATURE_NAMES.iter().zip(&learned.w) {
        writeln!(md, "| {n} | {w:+.3} |").unwrap();
    }


    // ── compositions and the sixty-four operations ──
    let novelty_hex: u8 = 0b10_1010;
    sections::compositions(&mut md, &voice, &learned, &kn, &split);
    sections::operations(&mut md, &space, novelty_hex);


    writeln!(md, "## 8. Honest notes\n\n- Three of the six novels; gutenberg.org is not reachable from the build machine, so *Pride and Prejudice*, *Mansfield Park* and *Northanger Abbey* are absent (`SOURCES.md`).\n- The generator is a trigram sampler with a learned, relationally-featured scorer; the judge is a trigram model of the same author. Numbers near the corpus's own perplexity mean \"as trigram-like as she is\", not \"as good as she is\".\n- One seed per prompt; eight prompts; treat differences under ~10% of perplexity as noise.\n- Long compositions read ~11 under the judge while her own sentences read ~190: they are her trigrams stitched end to end, not her sentences. The distinct-token ratio (0.70–0.78 at 60 tokens) shows the stitching — \"to be sure\", \"her own heart\" — and is the number to move next.\n- Until this run a run of four function words ended a composition outright (a hard stop in `generate.rs`), which is why earlier long arms came back three words long; the run now demands a content word instead (`FUNCTION_STREAK_LIMIT`).\n- Total run {:.0}s.\n", t0.elapsed().as_secs_f64()).unwrap();

    std::fs::write(&out_path, &md).expect("write report");
    println!("wrote {out_path} ({} bytes) in {:.0}s", md.len(), t0.elapsed().as_secs_f64());
    let _ = Facet::new();
}

