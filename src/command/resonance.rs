use crate::command::ranked::ranked;
use crate::command::usage;
use crate::command::Context;
use crate::command::Parser;
use phiecs::phasor::wave::Wave;

/// Resonance - finds words that resonate with a sentence wave.
///
/// Usage: `resonance "some text" [count]`
///
/// Computes the text wave, then ray casts to find words with minimal
/// energy delta to that wave.
pub struct Resonance;

impl Resonance {
    /// Finds and prints words resonating with the given text.
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            usage::say("resonance");
            return true;
        }

        let (text, count) = Parser::text_and_count(ctx.arg, 5);

        // All 64 channels, not one. `Wave::text` collapses a sentence to a
        // single complex number, and `ray_cast` then compares candidates on that
        // one angle — 77,000 words crowded onto one circle, where everything is
        // near everything. `sentence_channels` keeps the channels apart, which
        // is what the torus representation is for; its own doc comment says so,
        // and this command was not using it.
        let words: Vec<String> = phiton::p64::words::Tokenizer::tokenize(&text);
        let query = Wave::sentence_channels(ctx.manifold, &words, false);
        let matches = Wave::ray_cast_channels(ctx.manifold, &query, count);

        ranked(&matches);
        true
    }
}
