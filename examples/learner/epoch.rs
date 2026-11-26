//! The epochs, and the best of them kept.
//!
//! An epoch tells the learner everything it has ever been told once more;
//! the watched register is measured after each one and its row printed. The
//! best reading is kept as it is found, and the run stops when `patience`
//! epochs have passed without a better one (docs/203).
//!
//! **Kept as it is found, not at the end.** The machine restarted under an
//! eleven-hour run on 2026-11-26 and the whole of Tatoeba 19 was lost with
//! it, epoch 1's reading of 67.1 included, because nothing was written
//! until the epochs were done. A run cut short now costs the epoch and not
//! the run, and `Learner::save` renames its bytes over the checkpoint, so
//! the file a later run resumes from is whole or untouched.
//!
//! A run taken up again is told nothing new - `--resume ckpt --no-guide`
//! with no `--more` - and asked for the epochs that are left: telling it
//! the same text a second time would count those lines twice.

use phient::lineup::learner::Learner;
use std::time::Instant;

/// Runs the epochs and answers with the best learner of them. `from` is the
/// watched register's perplexity before the first epoch, so a run whose
/// epochs all read worse keeps what it started with; `keep` is given each
/// new best as it is found.
pub fn run(
    l: &mut Learner,
    watch: &[Vec<String>],
    epochs: usize,
    patience: usize,
    from: f64,
    keep: impl Fn(&Learner),
) -> Learner {
    let (mut best, mut best_ppl, mut since) = (l.clone(), from, 0);
    keep(&best);
    for e in 1..=epochs {
        let t = Instant::now();
        let loss = l.epoch();
        let r = l.measure(watch);
        println!("| {e} | {loss:.3} | {:.1}% | {:.1} | {:.1} |", r.top1(), r.perplexity(), t.elapsed().as_secs_f64());
        match r.perplexity() < best_ppl {
            true => {
                (best, best_ppl, since) = (l.clone(), r.perplexity(), 0);
                keep(&best);
            }
            false => {
                since += 1;
                match since >= patience {
                    true => break,
                    false => {}
                }
            }
        }
    }
    best
}
