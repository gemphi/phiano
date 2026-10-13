#[allow(unused_imports)]
use super::*;

pub(super) fn documents(sentences: &[String], n: usize) -> Vec<Vec<String>> {
    sentences.chunks(n).map(|c| c.to_vec()).collect()
}

/// Sentences with the same lengths and the same word frequencies, assembled at
/// random.
///
/// This is the null the whole comparison rests on. It preserves everything about
/// the corpus except which words occur *together*, so an effect that survives
/// here is not about sentences.
pub(super) fn shuffled(sentences: &[Vec<String>], seed: u64) -> Vec<Vec<String>> {
    let mut bag: Vec<&String> = sentences.iter().flatten().collect();
    // Fisher-Yates with a fixed generator: the null must be identical between
    // runs or it is not a control.
    let mut r = seed | 1;
    for i in (1..bag.len()).rev() {
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;
        bag.swap(i, (r % (i as u64 + 1)) as usize);
    }
    let mut out = Vec::with_capacity(sentences.len());
    let mut at = 0usize;
    for s in sentences {
        let take = s.len().min(bag.len() - at);
        out.push(bag[at..at + take].iter().map(|w| (*w).clone()).collect());
        at += take;
    }
    out
}
