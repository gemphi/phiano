//! A dialogue with a persona.
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions — so answering is not retrieval. The user's turn names a
//! condition; the persona's manifold says what that condition calls for
//! (`Facet::calls_for`, X ⇒ Y); and the reply is generated toward that word,
//! by a generator conditioned on the persona's own sector histogram and pair
//! evidence (`generate::persona`). The context wave carries across turns, so
//! the third answer is spoken after the first two.
//!
//! Every candidate reply is scored by how the persona's own fingerprint reads
//! it, and by the evaluator; the best is returned with both numbers, so a
//! transcript can be read with its confidence beside it.

use crate::eval::Evaluator;
use phiecs::phasor::facet::Facet;
use crate::generate::{ContextWaveBuffer, Decode, Generator, PersonaPrior};
use crate::persona::{Fingerprint, Style};
use crate::tokenizer::Tokenizer;
use std::rc::Rc;

mod reading;

/// Candidate replies drawn per turn.
pub const DIALOG_CANDIDATES: usize = 9;
/// A reply that repeats more of what was already said than this is refused
/// even when it scores best: the trigram tables have strong attractors
/// (docs/78 measured 26% shared 4-grams across prompts), and a dialogue is
/// where that shows most.
pub const DIALOG_REPEAT_MAX: f64 = 0.25;
/// Tokens per reply.
pub const DIALOG_TOKENS: usize = 28;
/// Tokens of the user's turn used to seed the reply.
pub const DIALOG_SEED: usize = 6;
/// How many of the persona's own frequent words the answer is chosen from.
pub const DIALOG_POOL: usize = 600;
/// How hard the reply is pulled toward that word.
pub const DIALOG_GOAL_PULL: f64 = 1.5;

/// A seed offset for every generator a dialogue runs (`PHIANO_DIALOG_SEED`,
/// default 0 = the shipped replies), so a comparison can be repeated over
/// several draws instead of resting on one.
fn dialog_seed() -> u64 {
    std::env::var("PHIANO_DIALOG_SEED").ok().and_then(|v| v.parse().ok()).map(|s: u64| s.wrapping_mul(0x9E37_79B9_7F4A_7C15)).unwrap_or(0)
}
/// How much of a candidate's score a repeat of what this dialogue has
/// already said costs. A persona has favourite phrases; a speaker who
/// answers six questions with the same clause is not one.
pub const DIALOG_REPEAT_COST: f64 = 0.8;

/// One turn's answer, with what it was about and how it scored.
pub struct Reply {
    pub text: String,
    /// Share of this reply's 4-grams the dialogue had already spoken.
    pub repeat: f64,
    /// The word in the user's turn the persona answered under.
    pub topic: Option<String>,
    /// What that condition called for, in this persona's manifold.
    pub calls_for: Option<String>,
    /// The persona's own likelihood of the reply's fingerprint.
    pub fit: f64,
    /// The evaluator's quality (coherence against the manifold).
    pub quality: f64,
}

pub struct Dialogue {
    pub name: String,
    pub(super) fingerprint: Fingerprint,
    prior: Rc<PersonaPrior>,
    /// The persona's frequent content words: the words it can answer with.
    pub(super) pool: Vec<String>,
    buffer: ContextWaveBuffer,
    /// Every 4-gram this dialogue has already spoken.
    said: std::collections::HashSet<String>,
    turn: u64,
    pub candidates: usize,
    pub tokens: usize,
}

impl Dialogue {
    /// Opens a dialogue with an ingested persona against the lexicon it was
    /// ingested into.
    pub fn open(facet: &Facet, persona: &Style) -> Self {
        let prior = PersonaPrior::new(facet, &persona.fingerprint, 0.0).with_examples(&persona.examples, 1.0);
        Self {
            name: persona.name.clone(),
            fingerprint: Fingerprint::extract(facet, &persona.examples),
            prior: Rc::new(prior),
            pool: Self::answer_pool(facet, &persona.examples),
            buffer: ContextWaveBuffer::new(crate::config::DEFAULT_CONTEXT_WINDOW),
            said: std::collections::HashSet::new(),
            turn: 0,
            candidates: DIALOG_CANDIDATES,
            tokens: DIALOG_TOKENS,
        }
    }

    /// The 4-grams of a text.
    fn grams(text: &str) -> Vec<String> {
        Tokenizer::tokenize(text).windows(4).map(|w| w.join(" ")).collect()
    }

    /// How much of a candidate is already spoken — by this dialogue on an
    /// earlier turn, or by the candidate itself a few words earlier.
    fn repeat(&self, text: &str) -> f64 {
        let g = Self::grams(text);
        if g.is_empty() {
            return 0.0;
        }
        let mut seen: std::collections::HashSet<&String> = std::collections::HashSet::new();
        let stale = g.iter().filter(|x| self.said.contains(*x) || !seen.insert(x)).count();
        stale as f64 / g.len() as f64
    }

    /// Answers one turn. The lexicon is read, never written: a dialogue does
    /// not train the persona on its own words.
    pub fn reply(&mut self, facet: &Facet, user: &str) -> Reply {
        let said = Tokenizer::tokenize(user);
        let topic = self.topic(facet, user);
        let goal = topic.as_ref().and_then(|t| self.calls_for(facet, t, &said));
        // Every candidate reads the question from a different place: the
        // tail of the question, the condition it names, or that condition
        // with what it calls for. Nine walks down one trigram path are one
        // answer; three starting points are three.
        let tail = |k: usize| match said.is_empty() {
            true => user.to_string(),
            false => said[said.len().saturating_sub(k)..].join(" "),
        };
        let seeds: Vec<String> = (0..self.candidates.max(1))
            .map(|i| match (i % 3, &topic, &goal) {
                (1, Some(t), _) => t.clone(),
                (2, Some(t), Some(g)) => format!("{t} {g}"),
                (2, Some(t), None) => format!("{t} {}", tail(2)),
                _ => tail(DIALOG_SEED - i / 3),
            })
            .collect();

        let eval = Evaluator::new();
        // (score, fit, quality, text, repeat)
        let mut best: Option<(f64, f64, f64, String, f64)> = None;
        let mut freshest: Option<(f64, f64, f64, String, f64)> = None;
        for (i, seed) in seeds.iter().enumerate() {
            let temperature = 0.3 + 0.15 * ((i / 3) % 3) as f64;
            let g = Generator::deterministic(self.tokens, temperature, phistd::mix64(self.turn * 7919 + i as u64 + dialog_seed()))
                .with_decode(Decode::Trigram)
                .with_persona_prior(self.prior.clone());
            let g = match &goal {
                Some(w) => g.with_goal(w, DIALOG_GOAL_PULL),
                None => g,
            };
            let mut buf = ContextWaveBuffer::new(crate::config::DEFAULT_CONTEXT_WINDOW);
            std::mem::swap(&mut buf, &mut self.buffer);
            let text = g.generate(facet, &mut buf, seed);
            if Tokenizer::tokenize(&text).len() < 3 {
                std::mem::swap(&mut buf, &mut self.buffer);
                continue;
            }
            let fit = self.fingerprint.likelihood(&Fingerprint::extract(facet, std::slice::from_ref(&text)));
            let quality = eval.eval(facet, &text).overall;
            let repeat = self.repeat(&text);
            let combined = (fit * crate::config::IMPERSONATE_FIT_WEIGHT + quality * crate::config::IMPERSONATE_QUALITY_WEIGHT) * (1.0 - DIALOG_REPEAT_COST * repeat);
            if freshest.as_ref().is_none_or(|b| repeat < b.4) {
                freshest = Some((combined, fit, quality, text.clone(), repeat));
            }
            if repeat <= DIALOG_REPEAT_MAX && best.as_ref().is_none_or(|b| combined > b.0) {
                best = Some((combined, fit, quality, text, repeat));
            }
            std::mem::swap(&mut buf, &mut self.buffer);
        }
        self.turn += 1;
        // Nothing fresh enough: the least repetitive candidate speaks.
        let (_, fit, quality, text, repeat) = best.or(freshest).unwrap_or((0.0, 0.0, 0.0, String::new(), 0.0));
        // Only the turn that was spoken advances the context, or is
        // remembered as said.
        self.buffer.push_turn(facet, user);
        self.buffer.push_turn(facet, &text);
        self.said.extend(Self::grams(&text));
        Reply { text, topic, calls_for: goal, fit, quality, repeat }
    }
}
