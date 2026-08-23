use crate::command::Context;
use crate::generate::Generator;

pub struct GenerateCmd;

impl GenerateCmd {
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: generate <prompt>");
            println!("  Example: generate Rust concurrency and memory model");
            return true;
        }

        let prompt = crate::command::Parser::strip_quotes(ctx.arg);
        let generator = Generator::new(32, 0.15);

        println!("  ── phase-guided sequence generation ──");
        println!("  prompt: \"{}\"", prompt);

        let generated = generator.generate(ctx.manifold, ctx.context_buffer, &prompt);
        println!("\n    {}\n", generated);

        true
    }
}
