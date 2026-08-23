use crate::command::Context;
use crate::instruction::InstructionEngine;

pub struct InstructionCmd;

impl InstructionCmd {
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: instruct <instruction or question>");
            println!("  Example: instruct explain ownership and borrowing in Rust");
            println!("  Example: instruct write a haiku about stars");
            return true;
        }

        let prompt = crate::command::Parser::strip_quotes(ctx.arg);
        let mut engine = InstructionEngine::new();

        println!("  ── executing instruction ──");
        let result = engine.execute_instruction(ctx.manifold, ctx.trainer, &prompt);
        println!("\n{}\n", result);

        // Record into multi-turn context wave buffer
        ctx.context_buffer.push_turn(ctx.manifold, &prompt);
        ctx.context_buffer.push_turn(ctx.manifold, &result);

        true
    }
}
