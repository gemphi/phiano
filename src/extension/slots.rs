//! One token's extension slots, and the coupling and reading on them.
#[allow(unused_imports)]
use super::*;

use crate::config::{CHANNEL_QUANTA, TWO_PI};
use crate::phasor::{mix64, quantise, fnv1a, Rounding};

/// One extension channel of one token: the angle it presents when relating
/// (`src`), when related to (`tgt`), both one byte of 2π as on the base
/// channels, and how often it was observed there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Slot {
    pub ch: u32,
    pub src: u8,
    pub tgt: u8,
    pub evid: u32,
}

fn byte_angle(b: u8) -> f64 {
    b as f64 * TWO_PI / CHANNEL_QUANTA
}

impl Extension {
    /// The slot, if the token has one on `ch`.
    pub fn slot(&self, word: &str, ch: usize) -> Option<&Slot> {
        let v = self.words.get(word)?;
        v.binary_search_by_key(&(ch as u32), |s| s.ch).ok().map(|i| &v[i])
    }

    /// The slot, created on first touch with seeded angles: a hash of the
    /// token and the channel, both sides equal, no evidence — the state a
    /// base channel starts in.
    pub fn slot_mut(&mut self, word: &str, ch: usize) -> &mut Slot {
        let v = self.words.entry(word.to_string()).or_default();
        let i = match v.binary_search_by_key(&(ch as u32), |s| s.ch) {
            Ok(i) => i,
            Err(i) => {
                let b = (mix64(fnv1a(word) ^ (ch as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)) & 0xFF) as u8;
                v.insert(i, Slot { ch: ch as u32, src: b, tgt: b, evid: 0 });
                i
            }
        };
        &mut v[i]
    }

    /// Source angle of `word` on `ch` (its seed if untouched).
    pub fn theta(&self, word: &str, ch: usize) -> f64 {
        match self.slot(word, ch) {
            Some(s) => byte_angle(s.src),
            None => byte_angle((mix64(fnv1a(word) ^ (ch as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)) & 0xFF) as u8),
        }
    }

    /// Target angle of `word` on `ch` (its seed if untouched).
    pub fn theta_target(&self, word: &str, ch: usize) -> f64 {
        match self.slot(word, ch) {
            Some(s) => byte_angle(s.tgt),
            None => self.theta(word, ch),
        }
    }

    pub fn evidence_count(&self, word: &str, ch: usize) -> u32 {
        self.slot(word, ch).map_or(0, |s| s.evid)
    }

    /// One Kuramoto step on extension channel `ch`, directed as on the base
    /// channels: `a`'s source side toward `b`'s target side and back, both
    /// gaining evidence. `boost` scales `b`'s step (the rare-target boost).
    pub fn couple(&mut self, a: &str, b: &str, ch: usize, rate: f64, boost: f64, rounding: Rounding) {
        let d = (self.theta_target(b, ch) - self.theta(a, ch)).sin();
        let step = |byte: u8, delta: f64, salt: u64| -> u8 {
            quantise(byte as f64 + delta / TWO_PI * CHANNEL_QUANTA, salt, rounding) as u8
        };
        let sa = self.slot_mut(a, ch);
        sa.src = step(sa.src, rate * d, (sa.src as u64) ^ ((sa.evid as u64) << 8) ^ ((ch as u64) << 40));
        sa.evid = sa.evid.saturating_add(1);
        let sb = self.slot_mut(b, ch);
        sb.tgt = step(sb.tgt, -rate * d * boost, (sb.tgt as u64) ^ ((sb.evid as u64) << 8) ^ ((ch as u64) << 40) ^ 0xD17);
        sb.evid = sb.evid.saturating_add(1);
    }

    /// Directed agreement of `a` (relating) with `b` (related to) on `ch`:
    /// `cos(θ_src^a − θ_tgt^b)`, `None` unless both were observed there.
    pub fn on(&self, a: &str, b: &str, ch: usize) -> Option<f64> {
        match (self.slot(a, ch), self.slot(b, ch)) {
            (Some(x), Some(y)) if x.evid > 0 && y.evid > 0 => Some((byte_angle(x.src) - byte_angle(y.tgt)).cos()),
            _ => None,
        }
    }
}
