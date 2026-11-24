//! PersonaCmd - manages personas and impersonation.
//!
//! Subcommands:
//!   persona add <name> "example 1" "example 2" ...
//!   persona from <name> "block of text"
//!   persona list
//!   persona show <name>
//!   persona compare <name_a> <name_b>
//!   persona impersonate <name> "prompt"
//!   persona match "unknown text"
//!   persona chat <name>


use crate::command::usage;
use crate::command::word::quoted;
use crate::command::Context;
use crate::persona::Impersonator;

pub struct PersonaCmd;

impl PersonaCmd {
    pub fn apply(&self, ctx: &mut Context) -> bool {
        let parts: Vec<&str> = ctx.arg.splitn(2, char::is_whitespace).collect();
        let subcmd = parts[0].to_lowercase();
        let rest = parts.get(1).map(|s| s.trim()).unwrap_or("");

        match subcmd.as_str() {
            "add" => self.add(ctx, rest),
            "from" => self.from(ctx, rest),
            "list" => self.list(ctx),
            "show" => self.show(ctx, rest),
            "compare" => self.compare(ctx, rest),
            "impersonate" => self.impersonate(ctx, rest),
            "match" => self.match_text(ctx, rest),
            "chat" => super::personate::chat(ctx, rest),
            "help" | "?" => {
                print!("{}", usage::forms("persona"));
                true
            }
            _ => {
                println!("  Unknown persona subcommand: '{}'", subcmd);
                print!("{}", usage::forms("persona"));
                true
            }
        }
    }

    fn add(&self, ctx: &mut Context, rest: &str) -> bool {
        let parts = quoted(rest);
        if parts.len() < 2 {
            usage::say("persona add");
            return true;
        }
        let name = parts[0].clone();
        let examples: Vec<String> = parts[1..].to_vec();
        ctx.world.add_persona(&name, &examples, ctx.manifold, ctx.trainer);
        if let Some(p) = ctx.world.get(&name) { println!("{}", p); }
        true
    }

    fn from(&self, ctx: &mut Context, rest: &str) -> bool {
        let parts = quoted(rest);
        if parts.len() < 2 {
            usage::say("persona from");
            println!("  The text is auto-split into sentences as examples.");
            return true;
        }
        let name = parts[0].clone();
        let raw_text = parts[1..].join(" ");
        let examples = phiton::p64::words::Tokenizer::split_sentences(&raw_text);
        if examples.is_empty() {
            println!("  No sentences found in the text.");
            return true;
        }
        println!("  [from] '{}' - {} sentences extracted from text", name, examples.len());
        for (i, ex) in examples.iter().enumerate() {
            println!("    #{}: \"{}\"", i + 1, ex);
        }
        println!();
        ctx.world.add_persona(&name, &examples, ctx.manifold, ctx.trainer);
        if let Some(p) = ctx.world.get(&name) { println!("{}", p); }
        true
    }

    fn list(&self, ctx: &mut Context) -> bool {
        let personas = ctx.world.list();
        if personas.is_empty() {
            println!("  No personas in the world. Use 'persona add' to create one.");
            return true;
        }
        println!("  Personas ({}):", personas.len());
        for p in personas {
            println!("    {} - {} samples, diversity {:.3}", p.name, p.fingerprint.sample_count, p.fingerprint.diversity);
        }
        true
    }

    fn show(&self, ctx: &mut Context, rest: &str) -> bool {
        let name = rest.trim();
        if name.is_empty() {
            usage::say("persona show");
            return true;
        }
        match ctx.world.get(name) {
            Some(p) => println!("{}", p),
            None => println!("  No persona named '{}'", name),
        }
        true
    }

    fn compare(&self, ctx: &mut Context, rest: &str) -> bool {
        let parts: Vec<&str> = rest.split_whitespace().collect();
        if parts.len() < 2 {
            usage::say("persona compare");
            return true;
        }
        match ctx.world.compare(parts[0], parts[1]) {
            Some(cmp) => println!("{}", cmp),
            None => println!("  One or both personas not found"),
        }
        true
    }

    fn impersonate(&self, ctx: &mut Context, rest: &str) -> bool {
        let parts = quoted(rest);
        if parts.len() < 2 {
            usage::say("persona impersonate");
            return true;
        }
        let name = parts[0].clone();
        let prompt = parts[1].clone();
        let persona = match ctx.world.get(&name) {
            Some(p) => p,
            None => {
                println!("  No persona named '{}'", name);
                return true;
            }
        };
        let impersonator = Impersonator::new();
        let result = impersonator.impersonate(
            ctx.manifold, ctx.trainer, &persona.fingerprint, &name, &prompt,
        );
        println!("{}", result);
        true
    }

    fn match_text(&self, ctx: &mut Context, rest: &str) -> bool {
        let parts = quoted(rest);
        if parts.is_empty() {
            usage::say("persona match");
            return true;
        }
        let text = parts.join(" ");
        match ctx.world.match_text(ctx.manifold, &text) {
            Some(result) => println!("{}", result),
            None => println!("  No personas in the world. Use 'persona add' to create one."),
        }
        true
    }
}
