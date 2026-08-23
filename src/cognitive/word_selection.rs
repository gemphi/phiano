/// Word selection and output synthesis for MentalCausation agent.
///
/// Split from synthesis.rs to keep files under 200 lines.
/// Contains the word selection logic (guided by intentional states)
/// and the sentence construction templates (shaped by speech act type).

use super::types::*;
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use crate::wave::Wave;

/// Selects words guided by intentional states.
/// Beliefs pull words that match the world (phase-resonant).
/// Desires pull words that could change the world (bigram-adjacent).
/// Intentions pull words that commit to action (definition-linked).
pub fn select_words(
    facet: &Facet,
    prompt: &str,
    states: &[IntentionalState],
    collective_phase: f64,
) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let prompt_tokens = Tokenizer::tokenize(prompt);

    // Start with prompt words that are known
    for token in &prompt_tokens {
        if facet.lexicon.contains_key(token) && !words.contains(token) {
            words.push(token.clone());
        }
    }

    // For each intentional state, pull words according to its mode
    for state in states {
        match state.mode {
            PsychologicalMode::Belief => {
                // Beliefs pull phase-resonant words (matching the world)
                let target = crate::wave::c64::new(
                    collective_phase.cos(),
                    collective_phase.sin(),
                );
                let ray = Wave::ray_cast(facet, target, 4);
                for (w, _) in ray {
                    if !words.contains(&w) { words.push(w); }
                }
            }
            PsychologicalMode::Desire | PsychologicalMode::Intention => {
                // Desires/intentions pull trigram followers first, then bigram
                for word in words.clone().iter().take(3) {
                    // Try trigram with prompt context
                    let prompt_tokens = Tokenizer::tokenize(prompt);
                    if prompt_tokens.len() >= 2 {
                        let prev = &prompt_tokens[prompt_tokens.len() - 2];
                        let curr = &prompt_tokens[prompt_tokens.len() - 1];
                        let tri = facet.trigram_candidates(prev, curr);
                        for (w, _) in tri.iter().take(2) {
                            if !words.contains(w) { words.push(w.clone()); }
                        }
                    }
                    // Fall back to bigram followers
                    let candidates = facet.next_word_candidates(word);
                    for (w, _) in candidates.iter().take(3) {
                        if !words.contains(w) { words.push(w.clone()); }
                    }
                }
            }
            _ => {}
        }
        if words.len() >= 12 { break; }
    }

    words
}

/// Synthesizes output using intentional states to shape the response.
/// The sentence structure is determined by the speech act type, and
/// the content is drawn from the selected words.
pub fn synthesize(
    words: &[String],
    act: SpeechActType,
    states: &[IntentionalState],
    prompt: &str,
) -> String {
    if words.len() < 3 {
        return format!("I need more information to respond to: {}", prompt);
    }

    let w = |i: usize| -> &str {
        words.get(i).map(|s| s.as_str()).unwrap_or("concepts")
    };

    let has_belief = states.iter().any(|s| s.mode == PsychologicalMode::Belief);

    match act {
        SpeechActType::Directive => {
            let mut s = Vec::new();
            s.push(format!("{} concerns {} and {}.", w(0), w(1), w(2)));
            if words.len() > 4 {
                s.push(format!("The relationship involves {} through {}.", w(3), w(4)));
            }
            if words.len() > 6 {
                s.push(format!("Key aspects: {}, {}, and {}.", w(5), w(6), w(7.min(words.len()-1))));
            }
            s.join(" ")
        }
        SpeechActType::Commissive => {
            format!("I will address {} by examining {} and {}.", w(0), w(1), w(2))
        }
        SpeechActType::Expressive => {
            format!("{} evokes {} and {} — a sense of {}.", w(0), w(1), w(2), w(3))
        }
        SpeechActType::Declarative => {
            format!("{} is established as {}, defined through {}.", w(0), w(1), w(2))
        }
        SpeechActType::Assertive => {
            let mut s = Vec::new();
            s.push(format!("{} is connected to {}.", w(0), w(1)));
            if words.len() > 3 {
                s.push(format!("This involves {} and {}.", w(2), w(3)));
            }
            if has_belief && words.len() > 5 {
                s.push(format!("Evidence suggests {} relates to {}.", w(4), w(5)));
            }
            s.join(" ")
        }
    }
}
