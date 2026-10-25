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
        let mut generator = Generator::new(32, 0.15);
        let mut guided = false;
        if std::path::Path::new(crate::config::OPERATOR_FILE).exists() {
            match crate::generate::OperatorGuide::load(
                crate::config::OPERATOR_FILE,
                ctx.manifold,
                crate::phinet::space::Sides::Directed,
                6,
            ) {
                Ok(guide) => {
                    generator = generator.with_operator(guide);
                    guided = true;
                }
                Err(e) => println!("  [operator] {e}; generating unguided"),
            }
        }

        println!("  ── phase-guided sequence generation{} ──", if guided { " · operator" } else { "" });
        println!("  prompt: \"{}\"", prompt);

        let generated = generator.generate(ctx.manifold, ctx.context_buffer, &prompt);
        println!("\n    {}\n", generated);

        true
    }
}
