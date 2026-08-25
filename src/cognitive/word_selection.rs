//! Word selection and output synthesis for the MentalCausation agent.
//!
//! Provides intentional state-guided word retrieval from the phase manifold
//! and template-directed sentence synthesis based on speech act classifications.
//! All public operations are encapsulated in [`WordSelector`], following the Diem
//! convention that all public symbols belong to named types.
//!
//! # Architecture
//!
//! ```text
//! Intentional States (Belief / Desire / Intention)
//!   │
//!   ▼
//! WordSelector::select_words()
//!   ├─▶ Beliefs: TorusPhasor resonance matching
//!   ├─▶ Desires/Intentions: Trigram & Bigram transition graph
//!   └─▶ Prompt Lexicon Overlap
//!   │
//!   ▼
//! WordSelector::synthesize()
//!   └─▶ SpeechActType (Directive / Commissive / Expressive / Declarative / Assertive)
//! ```

use super::types::*;
use crate::facet::Facet;
use crate::phasor::{SpectralPhasor, TorusPhasor};
use crate::tokenizer::Tokenizer;

/// Selector and synthesizer for intentional state-driven language generation.
pub struct WordSelector;

impl WordSelector {
    /// Selects words guided by intentional states.
    ///
    /// - **Beliefs** pull words matching the world state (torus phase resonance).
    /// - **Desires** pull words projecting possible states (bigram/trigram followers).
    /// - **Intentions** pull words committing to action (definition-linked transitions).
    pub fn select_words(
        facet: &Facet,
        prompt: &str,
        states: &[IntentionalState],
        collective_phase: f64,
    ) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();
        let prompt_tokens = Tokenizer::tokenize(prompt);

        for token in &prompt_tokens {
            match !Tokenizer::is_function_word(token)
                && facet.lexicon.contains_key(token)
                && !words.contains(token)
            {
                true => words.push(token.clone()),
                false => {}
            }
        }

        // For each intentional state, pull words according to its mode
        for state in states {
            match state.mode {
                PsychologicalMode::Belief => {
                    // Beliefs pull multi-frequency torus-resonant words (matching the world)
                    let target_phasor = SpectralPhasor::new(collective_phase, 1.0, 0);
                    let target_torus = TorusPhasor::from_spectral(&target_phasor);

                    let mut scored: Vec<(String, f64)> = facet
                        .lexicon
                        .iter()
                        .map(|(w, p)| {
                            let word_torus = TorusPhasor::from_spectral(p);
                            (w.clone(), target_torus.resonance(&word_torus))
                        })
                        .collect();
                    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    for (w, _) in scored.into_iter().take(4) {
                        match !words.contains(&w) {
                            true => words.push(w),
                            false => {}
                        }
                    }
                }
                PsychologicalMode::Desire | PsychologicalMode::Intention => {
                    // Desires/intentions pull trigram followers first, then bigram
                    for word in words.clone().iter().take(3) {
                        // Try trigram with prompt context
                        let prompt_tokens = Tokenizer::tokenize(prompt);
                        match prompt_tokens.len() >= 2 {
                            true => {
                                let prev = &prompt_tokens[prompt_tokens.len() - 2];
                                let curr = &prompt_tokens[prompt_tokens.len() - 1];
                                let tri = facet.trigram_candidates(prev, curr);
                                for (w, _) in tri.iter().take(2) {
                                    match !Tokenizer::is_function_word(w) && !words.contains(w) {
                                        true => words.push(w.clone()),
                                        false => {}
                                    }
                                }
                            }
                            false => {}
                        }
                        let candidates = facet.next_word_candidates(word);
                        for (w, _) in candidates.iter().take(3) {
                            match !Tokenizer::is_function_word(w) && !words.contains(w) {
                                true => words.push(w.clone()),
                                false => {}
                            }
                        }
                    }
                }
                _ => {}
            }
            match words.len() >= 12 {
                true => break,
                false => {}
            }
        }

        words
    }

    /// Synthesizes output using intentional states to shape the response.
    ///
    /// The sentence structure is determined by the speech act type, and
    /// the content is drawn from the selected words.
    pub fn synthesize(
        words: &[String],
        act: SpeechActType,
        states: &[IntentionalState],
        prompt: &str,
    ) -> String {
        match words.len() < 3 {
            true => return format!("I need more information to respond to: {}", prompt),
            false => {}
        }

        let w = |i: usize| -> &str {
            words.get(i).map(|s| s.as_str()).unwrap_or("concepts")
        };

        let has_belief = states.iter().any(|s| s.mode == PsychologicalMode::Belief);

        match act {
            SpeechActType::Directive => {
                let mut s = Vec::new();
                s.push(format!("{} concerns {} and {}.", w(0), w(1), w(2)));
                match words.len() > 4 {
                    true => s.push(format!("The relationship involves {} through {}.", w(3), w(4))),
                    false => {}
                }
                match words.len() > 6 {
                    true => s.push(format!("Key aspects: {}, {}, and {}.", w(5), w(6), w(7.min(words.len()-1)))),
                    false => {}
                }
                s.join(" ")
            }
            SpeechActType::Commissive => {
                format!("I will address {} by examining {} and {}.", w(0), w(1), w(2))
            }
            SpeechActType::Expressive => {
                format!("{} evokes {} and {} - a sense of {}.", w(0), w(1), w(2), w(3))
            }
            SpeechActType::Declarative => {
                format!("{} is established as {}, defined through {}.", w(0), w(1), w(2))
            }
            SpeechActType::Assertive => {
                let mut s = Vec::new();
                s.push(format!("{} is connected to {}.", w(0), w(1)));
                match words.len() > 3 {
                    true => s.push(format!("This involves {} and {}.", w(2), w(3))),
                    false => {}
                }
                match has_belief && words.len() > 5 {
                    true => s.push(format!("Evidence suggests {} relates to {}.", w(4), w(5))),
                    false => {}
                }
                s.join(" ")
            }
        }
    }
}
