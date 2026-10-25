//! The texts the reading tests draw (docs/145, docs/148): joins of *Emma*'s
//! chapters, joins of four kinds of text, synthetic topics at two
//! strengths, and the talk. Each suite draws from its own generator, so a
//! suite whose pool grows (the cross joins read the live workspace) does
//! not move the draws of the suites after it.

use super::{bench, cross, score, synth};
use phient::prose::strand::Prior;
use phient::prose::tokens;

/// One text: its tokens, where its true segments begin, the prior it is read against.
pub type Text = (Vec<String>, Vec<usize>, Prior);

/// Every suite's texts, `trials` a suite (the talk once), by name.
pub fn draw(trials: usize, seed: u64) -> Vec<(String, Vec<Text>)> {
    let root = std::path::Path::new("crates/phiano/data/text/emma");
    let (emma, pool, english) = (bench::Emma::read(root), cross::Pool::read(std::path::Path::new(".")), Prior::read(root));
    let vocab = synth::vocabulary(&english);
    let mut suite = 0u64;
    let mut out: Vec<(String, Vec<Text>)> = Vec::new();
    let mut add = |name: &str, f: &mut dyn FnMut(&mut score::Rng) -> Text| {
        suite += 1;
        let mut rng = score::Rng(seed ^ (suite << 32));
        out.push((name.to_string(), (0..trials).map(|_| f(&mut rng)).collect()));
    };
    add("emma joins", &mut |r| { let k = 2 + r.below(7); let j = emma.draw(r, k, 120); (tokens(&j.text), j.truth, j.prior) });
    add("cross joins", &mut |r| { let (t, truth, p, _) = pool.draw(r, 120); (tokens(&t), truth, p) });
    for l in [0.2, 0.4] {
        add(&format!("synthetic λ={l}"), &mut |r| { let (w, truth) = synth::draw(&vocab, r, l, 120); (w, truth, english.clone()) });
    }
    let talk = tokens(&std::fs::read_to_string("crates/phiano/data/talks/world_models_close_and_qa.txt").expect("talk"));
    out.push(("the talk".into(), vec![(talk, Vec::new(), english)]));
    out
}
