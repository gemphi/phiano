use crate::command::Context;
use crate::generate::Generator;
use crate::instruction::generate_response;

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

        let prompt = crate::command::Parser::strip_quotes(ctx.arg);
        let generator = Generator::new(128, 0.15);

        let response = generate_response(
            ctx.manifold,
            ctx.trainer,
            ctx.cognitive_core,
            ctx.context_buffer,
            &generator,
            &prompt,
        );

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
