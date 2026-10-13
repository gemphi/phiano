//! How a division of a text is scored against the true one. Boundaries are
//! token offsets where a segment starts (never 0). `pk` is Beeferman et
//! al.'s probe error: two positions `w` apart, `w` half the mean true segment,
//! counted wrong when the truth and the guess disagree on whether they share
//! a segment (0 perfect, ~0.5 chance). `f1` matches each guessed boundary
//! to an unmatched true one within `tol` tokens.

/// The segment index of every position, given the starts of segments 1…
fn labels(n: usize, cuts: &[usize]) -> Vec<usize> {
    let mut c = cuts.to_vec();
    c.sort_unstable();
    let mut seg = 0;
    (0..n)
        .map(|i| {
            while seg < c.len() && i >= c[seg] {
                seg += 1;
            }
            seg
        })
        .collect()
}

pub fn pk(n: usize, truth: &[usize], guess: &[usize]) -> f64 {
    let w = ((n as f64 / (truth.len() + 1) as f64) / 2.0).round().max(1.0) as usize;
    if n <= w {
        return 0.0;
    }
    let (t, g) = (labels(n, truth), labels(n, guess));
    let wrong = (0..n - w).filter(|&i| (t[i] == t[i + w]) != (g[i] == g[i + w])).count();
    wrong as f64 / (n - w) as f64
}

pub fn f1(truth: &[usize], guess: &[usize], tol: usize) -> f64 {
    if truth.is_empty() && guess.is_empty() {
        return 1.0;
    }
    let mut used = vec![false; truth.len()];
    let mut hit = 0;
    for g in guess {
        if let Some(j) = (0..truth.len()).filter(|j| !used[*j] && truth[*j].abs_diff(*g) <= tol).min_by_key(|j| truth[*j].abs_diff(*g)) {
            used[j] = true;
            hit += 1;
        }
    }
    let (p, r) = (hit as f64 / guess.len().max(1) as f64, hit as f64 / truth.len().max(1) as f64);
    if p + r == 0.0 { 0.0 } else { 2.0 * p * r / (p + r) }
}

/// `k − 1` boundaries splitting `n` tokens into equal parts.
pub fn equal(n: usize, k: usize) -> Vec<usize> {
    (1..k).map(|i| i * n / k).collect()
}

/// A small deterministic generator (splitmix64), for draws a reader can repeat.
pub struct Rng(pub u64);

impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    pub fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }

    /// `k − 1` distinct random boundaries in `1..n`.
    pub fn cuts(&mut self, n: usize, k: usize) -> Vec<usize> {
        let mut v: Vec<usize> = Vec::new();
        while v.len() + 1 < k.min(n) {
            let c = 1 + self.below(n - 1);
            if !v.contains(&c) {
                v.push(c);
            }
        }
        v.sort_unstable();
        v
    }

    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.below(i + 1);
            v.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scores_agree_with_hand_counts() {
        assert_eq!(pk(100, &[50], &[50]), 0.0);
        assert!((pk(100, &[50], &[]) - 25.0 / 75.0).abs() < 1e-9, "a missed boundary: 25 of 75 probes");
        assert_eq!(f1(&[50, 80], &[52, 30], 3), 0.5);
        assert_eq!(equal(9, 3), [3, 6]);
        let mut r = Rng(7);
        assert_eq!(r.cuts(10, 4).len(), 3);
    }
}
