//! The engine's answer to one prompt.
//!
//! `chat` and `instruct` differ in what they print, not in what they ask:
//! each had written out the same six-argument call, with the same generator
//! and the same quote stripping, before printing its own way. (The chat's
//! interactive loop keeps one generator across its turns and asks with the
//! line as typed, so it is not this call.)

use super::{Context, Parser};
use crate::instruction::{Answer, InstructionEngine};
use phient::generate::Generator;

/// Channels the commands' generator speaks on.
const CHANNELS: usize = 128;
/// How much of the generator's reach is left silent.
const SILENCE: f64 = 0.15;

/// The generator the commands speak with. The chat's interactive loop
/// makes one before its first turn and keeps it; every other command asks
/// once and lets it go.
pub(super) fn generator() -> Generator {
    Generator::new(CHANNELS, SILENCE)
}

/// The answer to `arg`: its quotes stripped, the model's parts taken from
/// the context, and the online training the engine does on the way.
pub(super) fn answer(ctx: &mut Context, arg: &str) -> Answer {
    let prompt = Parser::strip_quotes(arg);
    let generator = generator();
    InstructionEngine::generate_response(
        ctx.manifold,
        ctx.trainer,
        ctx.cognitive_core,
        ctx.context_buffer,
        &generator,
        &prompt,
    )
}
