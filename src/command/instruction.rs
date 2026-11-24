use crate::command::answer::answer;
use crate::command::Context;

/// Instruct - executes an instruction using the shared cognitive pipeline.
///
/// Usage: `instruct "explain ownership in Rust"`
///        `instruct "write a haiku about stars"`
///
/// Shows the response with cognitive metadata (verbose mode).
/// For clean conversational output, use `chat` instead.
pub struct InstructionCmd;

impl InstructionCmd {
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: instruct <instruction or question>");
            println!("  Example: instruct explain ownership and borrowing in Rust");
            println!("  Example: instruct write a haiku about stars");
            println!("  Tip: use 'chat' for clean conversational mode");
            return true;
        }

        let response = answer(ctx, ctx.arg);

        println!("\n  ── instruction ({:?}) ──\n", response.intent);
        println!("  {}\n", response.text);
        println!("  ── cognitive synthesis ──");
        println!("  {}\n", response.cognitive_synthesis);
        println!("  [speech act: {} | fit: {} | satisfaction: {:.0}%]",
            response.speech_act, response.direction_of_fit, response.satisfaction * 100.0);
        println!("  [phase trace: {}]\n", response.phase_trace);

        true
    }
}
