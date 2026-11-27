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
//!
//! **A second register, shown and never read.** Tatoeba 19 read 67.1 on
//! the watched register at three of its four epochs and the run kept the
//! one of them with the lowest top-1: at 1,085 sentences the register
//! cannot tell its own epochs apart, while the never-told register moved
//! 3.3 over the instalment (docs/241). `--beside` prints a second
//! register's perplexity in the table so a run says where it was going,
//! and that register stops nothing and keeps nothing - a register that
//! chose an epoch would be a register the learner had been fitted to.

use phient::lineup::learner::Learner;
use std::time::Instant;

/// The registers an epoch is read on.
pub struct Register<'a> {
    /// The stop rule's and the kept epoch's: the only one either reads.
    pub watch: &'a [Vec<String>],
    /// Shown beside it, deciding nothing. Empty when none was named.
    pub beside: &'a [Vec<String>],
}

/// The second register's cell in the table's row, empty when none was named.
pub fn beside(l: &Learner, on: &[Vec<String>]) -> String {
    match on.is_empty() {
        true => String::new(),
        false => format!(" {:.1} |", l.measure(on).perplexity()),
    }
}

/// Runs the epochs and answers with the best learner of them. `from` is the
/// watched register's perplexity before the first epoch, so a run whose
/// epochs all read worse keeps what it started with; `keep` is given each
/// new best as it is found.
pub fn run(l: &mut Learner, on: Register, epochs: usize, patience: usize, from: f64, keep: impl Fn(&Learner)) -> Learner {
    let (mut best, mut best_ppl, mut since) = (l.clone(), from, 0);
    keep(&best);
    for e in 1..=epochs {
        let t = Instant::now();
        let loss = l.epoch();
        let r = l.measure(on.watch);
        let second = beside(l, on.beside);
        println!("| {e} | {loss:.3} | {:.1}% | {:.1} |{second} {:.1} |", r.top1(), r.perplexity(), t.elapsed().as_secs_f64());
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
