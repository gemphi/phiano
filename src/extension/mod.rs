//! Channels past the base 64.
//!
//! In Phient's world, a word (w) is nothing but what it relates to, under what
//! conditions (c). Every token has the 64 base channels phiano has always had
//! — 8 offsets for word order, the rest for the first relation kinds — packed
//! densely in its [`SpectralPhasor`](crate::phasor::SpectralPhasor). Past them
//! a channel exists only where it is used: a sparse list per token of
//! `(channel, source byte, target byte, evidence)`, with ids from 64 upward
//! handed out by one registry as kinds, answer slots and conditions are
//! declared. There is no upper bound; a token pays for what it touches
//! (docs/90 §2).
//!
//! The first use is **answer slots**. The base design gives a word one angle
//! per kind, so every answer it gives under that kind is pulled to that one
//! angle and so is every other word that shares one of them: on the Phi-4
//! quest `cat · is-a` read back `mammal` tied with `allergies`, `group` and
//! `calmness` (docs/89 §5). With `n` slots a kind owns `n` extension channels
//! and each answer lives on one of them, chosen by a hash of the answer; a
//! word is read on a slot only where it was trained on it, so an answer
//! reaches `cat` only if `cat` was told something on that answer's slot.

mod slots;
#[cfg(test)]
mod tests;

pub use slots::Slot;

use crate::config::PHASE_CHANNELS;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The first extension channel id.
pub const BASE: usize = PHASE_CHANNELS;

/// The channel registry and every token's extension slots.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Extension {
    /// Channel `BASE + i` is named `names[i]` (`is-a#3`, `scope:fn`, …).
    names: Vec<String>,
    #[serde(skip)]
    index: HashMap<String, usize>,
    /// How many answer slots a base kind channel is split into (absent: 1,
    /// the base channel alone).
    slots_per: HashMap<usize, u32>,
    /// Each token's slots, sorted by channel.
    words: HashMap<String, Vec<Slot>>,
}

impl Extension {
    /// Declares a channel by name and returns its id (≥ [`BASE`]); the same
    /// name always returns the same id.
    pub fn declare(&mut self, name: &str) -> usize {
        if self.index.len() != self.names.len() {
            self.reindex();
        }
        if let Some(&c) = self.index.get(name) {
            return c;
        }
        let c = BASE + self.names.len();
        self.names.push(name.to_string());
        self.index.insert(name.to_string(), c);
        c
    }

    /// The id of a declared channel.
    pub fn channel(&self, name: &str) -> Option<usize> {
        match self.index.len() == self.names.len() {
            true => self.index.get(name).copied(),
            false => self.names.iter().position(|n| n == name).map(|i| BASE + i),
        }
    }

    /// The name of channel `ch` (≥ [`BASE`]).
    pub fn name(&self, ch: usize) -> Option<&str> {
        ch.checked_sub(BASE).and_then(|i| self.names.get(i)).map(String::as_str)
    }

    fn reindex(&mut self) {
        self.index = self.names.iter().enumerate().map(|(i, n)| (n.clone(), BASE + i)).collect();
    }

    /// Splits base channel `k` into `n` answer slots (`n ≤ 1` turns it off).
    pub fn set_slots(&mut self, k: usize, n: u32) {
        match n > 1 {
            true => self.slots_per.insert(k, n),
            false => self.slots_per.remove(&k),
        };
    }

    /// Answer slots on base channel `k`; 1 means none.
    pub fn slots(&self, k: usize) -> u32 {
        self.slots_per.get(&k).copied().unwrap_or(1)
    }

    fn slot_name(&self, k: usize, kind: &str, answer: &str) -> String {
        let n = self.slots(k) as u64;
        format!("{kind}#{}", crate::phasor::fnv1a(answer) % n)
    }

    /// The channel `answer` lives on under base channel `k` named `kind`,
    /// declaring its slot if needed; `k` itself when `k` has no slots.
    pub fn route(&mut self, k: usize, kind: &str, answer: &str) -> usize {
        match self.slots(k) > 1 {
            true => {
                let name = self.slot_name(k, kind, answer);
                self.declare(&name)
            }
            false => k,
        }
    }

    /// [`Extension::route`] without declaring: `None` when the slot was never
    /// used, `Some(k)` when `k` has no slots.
    pub fn route_existing(&self, k: usize, kind: &str, answer: &str) -> Option<usize> {
        match self.slots(k) > 1 {
            true => self.channel(&self.slot_name(k, kind, answer)),
            false => Some(k),
        }
    }

    /// Declared extension channels.
    pub fn channels(&self) -> usize {
        self.names.len()
    }

    /// Angles stored past the base: two per slot (source and target).
    pub fn angles(&self) -> usize {
        2 * self.words.values().map(Vec::len).sum::<usize>()
    }

    /// Tokens holding at least one extension slot.
    pub fn tokens(&self) -> usize {
        self.words.len()
    }
}
