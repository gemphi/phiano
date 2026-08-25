//! Interactive persona chat loop.
//!
//! Emulates interactive conversational flow with a persona by projecting
//! prompts through the persona's phase fingerprint. All operations are encapsulated
//! in [`PersonaChatLoop`], following the Diem convention that all public symbols
//! belong to named types.

use crate::command::Context;
use crate::persona::Impersonator;
use std::io::{BufRead, Write};

/// Interactive REPL loop for conversing with a specific persona.
pub struct PersonaChatLoop;

impl PersonaChatLoop {
    /// Starts the interactive chat session with the target persona.
    pub fn chat(ctx: &mut Context, rest: &str) -> bool {
        let name = rest.trim();
        if name.is_empty() {
            println!("  Usage: persona chat <name>");
            return true;
        }

        let persona = match ctx.world.get(name) {
            Some(p) => p,
            None => {
                println!("  No persona named '{}'", name);
                return true;
            }
        };

        let traits = persona.fingerprint.personality_traits();
        let dominant = persona.fingerprint.dominant_sectors(5);
        let traits_str = traits.join(", ");

        let top_colors: Vec<String> = dominant
            .iter()
            .map(|(s, w)| {
                let color = crate::compose::SectorPalette::color(*s);
                format!("{} ({:.1}%)", color, w * 100.0)
            })
            .collect();

        println!();
        println!("  ╔══════════════════════════════════════════════════════════════╗");
        println!("  ║  Hello, I'm {:<12} - {:<44} ║", name, traits_str);
        println!("  ║                                                              ║");
        println!("  ║  Phase signature: {:<42} ║", top_colors.join(" · "));
        println!("  ║  Diversity: {:.3}  |  Avg length: {:.1} words  |  Samples: {:<5} ║",
            persona.fingerprint.diversity,
            persona.fingerprint.avg_length,
            persona.fingerprint.sample_count);
        println!("  ║                                                              ║");
        println!("  ║  I respond in phase resonance - my word choices reflect      ║");
        println!("  ║  how your prompt vibrates through my fingerprint.            ║");
        println!("  ║  Type 'bye' to end the chat.                                 ║");
        println!("  ╚══════════════════════════════════════════════════════════════╝");
        println!();

        let impersonator = Impersonator::new();
        let mut stdin = std::io::stdin().lock();

        loop {
            print!("  {}> ", name);
            std::io::stdout().flush().ok();

            let mut input = String::new();
            if stdin.read_line(&mut input).is_err() || input.is_empty() {
                break;
            }

            let prompt = input.trim();
            if prompt.is_empty() {
                continue;
            }

            if prompt.eq_ignore_ascii_case("bye")
                || prompt.eq_ignore_ascii_case("exit")
                || prompt.eq_ignore_ascii_case("quit")
            {
                println!("  {} waves goodbye.", name);
                break;
            }

            let result = impersonator.impersonate(
                ctx.manifold,
                ctx.trainer,
                &persona.fingerprint,
                name,
                prompt,
            );

            println!();
            println!("{}", result);
            println!();
        }

        true
    }
}

/// Convenience alias for [`PersonaChatLoop::chat`].
#[inline]
pub fn chat(ctx: &mut Context, rest: &str) -> bool {
    PersonaChatLoop::chat(ctx, rest)
}
