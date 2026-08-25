use crate::command::Context;
use crate::command::Parser;
use crate::oscillator::{
    OscillatorField, OscillatorEval, ComparisonResult, SphereView,
    OscillatorTrainer,
};


/// OscillatorCmd - oscillator model commands.
///
/// The oscillator model is an alternative to the transform model.
/// Words are oscillators on a sphere, not static points on a circle.
///
/// Subcommands:
///   oscillator eval "text"           - Evaluate text in oscillator mode
///   oscillator sphere "text"         - Show the sphere projection for text
///   oscillator wheel                 - Show the equatorial color wheel
///   oscillator compare "text"        - Compare transform vs oscillator models
///   oscillator train "text" [epochs] - Train using oscillator synchronization
pub struct OscillatorCmd;

impl OscillatorCmd {
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            self.print_help();
            return true;
        }

        let parts: Vec<&str> = ctx.arg.splitn(2, char::is_whitespace).collect();
        let subcmd = parts[0].to_lowercase();
        let rest = parts.get(1).map(|s| s.trim()).unwrap_or("");

        match subcmd.as_str() {
            "eval" => self.eval(ctx, rest),
            "sphere" => self.sphere(ctx, rest),
            "wheel" => self.wheel(ctx),
            "compare" => self.compare(ctx, rest),
            "train" => self.train(ctx, rest),
            "help" | "?" => {
                self.print_help();
                true
            }
            _ => {
                println!("  Unknown oscillator subcommand: '{}'", subcmd);
                self.print_help();
                true
            }
        }
    }

    fn print_help(&self) {
        println!("  oscillator eval \"text\"        - Evaluate text in oscillator mode");
        println!("  oscillator sphere \"text\"      - Show sphere projection for text");
        println!("  oscillator wheel              - Show the equatorial color wheel");
        println!("  oscillator compare \"text\"     - Compare transform vs oscillator models");
        println!("  oscillator train \"text\" [n]   - Train using oscillator sync (n epochs)");
    }

    fn eval(&self, ctx: &mut Context, rest: &str) -> bool {
        let text = Parser::strip_quotes(rest);
        if text.is_empty() {
            println!("  Usage: oscillator eval \"text to evaluate\"");
            return true;
        }

        let field = OscillatorField::from_facet(ctx.manifold);
        let result = OscillatorEval::evaluate(&field, &text);
        println!("{}", result);
        true
    }

    fn sphere(&self, ctx: &mut Context, rest: &str) -> bool {
        let text = Parser::strip_quotes(rest);
        if text.is_empty() {
            println!("  Usage: oscillator sphere \"text\"");
            return true;
        }

        let field = OscillatorField::from_facet(ctx.manifold);
        let output = SphereView::render_sphere(&field, 0.0);
        println!("{}", output);

        let result = OscillatorEval::evaluate(&field, &text);
        println!("{}", result);
        true
    }

    fn wheel(&self, ctx: &mut Context) -> bool {
        let field = OscillatorField::from_facet(ctx.manifold);
        let output = SphereView::render_wheel(&field, 0.0);
        println!("{}", output);
        true
    }

    fn compare(&self, ctx: &mut Context, rest: &str) -> bool {
        let text = Parser::strip_quotes(rest);
        if text.is_empty() {
            println!("  Usage: oscillator compare \"text\"");
            return true;
        }

        let result = ComparisonResult::compare(ctx.manifold, &text);
        println!("{}", result);
        true
    }

    fn train(&self, ctx: &mut Context, rest: &str) -> bool {
        let parts: Vec<&str> = rest.splitn(2, char::is_whitespace).collect();
        let text = Parser::strip_quotes(parts.get(0).map(|s| *s).unwrap_or(""));
        if text.is_empty() {
            println!("  Usage: oscillator train \"text\" [epochs]");
            return true;
        }
        let epochs: usize = parts
            .get(1)
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(10);

        let trainer = OscillatorTrainer::new();
        let result = trainer.train(ctx.manifold, &text, epochs);
        println!("{}", result);
        true
    }
}
