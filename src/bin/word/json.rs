//! Exports a few words' two-sided phasors as JSON for the 3-D docs.
//! `cargo run --release --bin word_json [lexicon] word...`
use phiano::config::PHASE_CHANNELS;
use phiano::facet::Facet;
use phiano::storage::Storage;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lex = args.get(1).cloned().unwrap_or_else(|| "data/austen_voice.chroma".into());
    let facet: Facet = Storage::load(&lex).expect("lexicon");
    let mut out = Vec::new();
    for w in args.iter().skip(2) {
        if let Some(p) = facet.lexicon.get(w) {
            let mut ch = Vec::new();
            for k in 0..PHASE_CHANNELS {
                ch.push(format!("{{\"k\":{k},\"name\":{:?},\"src\":{:.4},\"tgt\":{:.4},\"evidence\":{}}}", facet.relation_name(k), p.theta(k), p.theta_target(k), p.evidence_count(k)));
            }
            out.push(format!("{{\"word\":{w:?},\"count\":{},\"channels\":[{}]}}", p.count, ch.join(",")));
        }
    }
    println!("[{}]", out.join(","));
}
