use crate::command::Context;
use crate::command::Parser;
use crate::generate::Generator;
use crate::instruction::InstructionEngine;
use std::io::{BufRead, Write};

/// Chat - conversational mode with the cognitive pipeline.
///
/// Runs the full 16-agent cognitive core behind the scenes,
/// but shows only the response. No diagnostic metadata by default.
///
/// Usage:
///   chat "what is a dog?"          - single-shot response
///   chat                           - interactive multi-turn chat
///   chat --verbose "what is a dog" - response + cognitive metadata
///
/// In-chat commands (interactive mode):
///   !correct wrong|correct   - phase-repulsion self-correction
///   bye / exit / quit        - end the chat
pub struct ChatCmd;

impl ChatCmd {
    pub fn apply(&self, ctx: &mut Context) -> bool {
        match ctx.arg.is_empty() {
            true => self.interactive(ctx),
            false => match ctx.arg.starts_with("--verbose ") || ctx.arg == "--verbose" {
                true => {
                    let rest = ctx.arg.strip_prefix("--verbose").unwrap_or("").trim();
                    match rest.is_empty() {
                        true => {
                            println!("  Usage: chat --verbose \"your question\"");
                            true
                        }
                        false => self.single_shot(ctx, rest, true),
                    }
                }
                false => self.single_shot(ctx, ctx.arg, false),
            }
        }
    }

    /// Single-shot chat: process one prompt, print clean response.
    fn single_shot(&self, ctx: &mut Context, arg: &str, verbose: bool) -> bool {
        let prompt = Parser::strip_quotes(arg);
        let generator = Generator::new(128, 0.15);
        let response = InstructionEngine::generate_response(
            ctx.manifold,
            ctx.trainer,
            ctx.cognitive_core,
            ctx.context_buffer,
            &generator,
            &prompt,
        );

        match verbose {
            true => println!("{}", response),
            false => println!("\n  {}\n", response.text),
        }
        true
    }

    /// Interactive multi-turn chat loop.
    fn interactive(&self, ctx: &mut Context) -> bool {
        println!();
        println!("  ╔════════════════════════════════════════════════════════╗");
        println!("  ║  Phiano Chat - phase-aware cognitive conversation       ║");
        println!("  ║  Type 'bye' to end. !correct wrong|correct to fix.     ║");
        println!("  ╚════════════════════════════════════════════════════════╝");
        println!();

        let generator = Generator::new(128, 0.15);
        let mut stdin = std::io::stdin().lock();
        let mut turn = 1u32;

        loop {
            print!("  You> ");
            std::io::stdout().flush().ok();

            let mut input = String::new();
            if stdin.read_line(&mut input).is_err() || input.is_empty() {
                break;
            }

            let question = input.trim().to_string();
            match question.is_empty() {
                true => continue,
                false => {}
            }

            match question.eq_ignore_ascii_case("bye")
                || question.eq_ignore_ascii_case("exit")
                || question.eq_ignore_ascii_case("quit")
            {
                true => {
                    println!("\n  Phiano> Farewell. The phase circle turns.\n");
                    break;
                }
                false => {}
            }

            match question.starts_with("!correct ") {
                true => {
                    let rest = &question["!correct ".len()..];
                    match rest.split_once('|') {
                        Some((wrong, correct)) => {
                            let (wrong, correct) = (wrong.trim(), correct.trim());
                            let evaluator = crate::eval::Evaluator::new();
                            let before = evaluator.eval(ctx.manifold, wrong).coherence;

                            ctx.trainer.correct_graded(
                                ctx.manifold,
                                wrong,
                                correct,
                                crate::config::CORRECTION_STRENGTH,
                            );
                            ctx.corrections.record(
                                wrong,
                                correct,
                                Some(crate::config::CORRECTION_STRENGTH),
                            );

                            // Show the correction took, rather than asserting it.
                            let after = evaluator.eval(ctx.manifold, wrong).coherence;
                            println!(
                                "\n  Phiano> Corrected. '{}' coherence {:.2} → {:.2}; \
                                 reinforced '{}'. Journalled ({} total).\n",
                                wrong, before, after, correct, ctx.corrections.len()
                            );
                        }
                        None => {
                            println!("\n  Phiano> Usage: !correct wrong phrase | correct phrase\n");
                        }
                    }
                    turn += 1;
                    continue;
                }
                false => {}
            }

            let response = InstructionEngine::generate_response(
                ctx.manifold,
                ctx.trainer,
                ctx.cognitive_core,
                ctx.context_buffer,
                &generator,
                &question,
            );

            println!();
            println!("  ── turn {} ──", turn);
            println!();
            for line in response.text.lines() {
                println!("  Phiano> {}", line);
            }
            println!();
            turn += 1;
        }

        true
    }
}
