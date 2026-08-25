/// PersonaCmd - manages personas and impersonation.
///
/// Subcommands:
///   persona add <name> "example 1" "example 2" ...
///   persona from <name> "block of text"
///   persona list
///   persona show <name>
///   persona compare <name_a> <name_b>
///   persona impersonate <name> "prompt"
///   persona match "unknown text"
///   persona chat <name>

mod chat;

use crate::command::Context;
use crate::persona::Impersonator;

pub struct PersonaCmd;

impl PersonaCmd {
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            self.print_help();
            return true;
        }

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
            "chat" => chat::chat(ctx, rest),
            "help" | "?" => { self.print_help(); true }
            _ => {
                println!("  Unknown persona subcommand: '{}'", subcmd);
                self.print_help();
                true
            }
        }
    }

    fn print_help(&self) {
        println!("  persona add <name> \"ex1\" \"ex2\" ...  - Create persona from examples");
        println!("  persona from <name> \"block of text\"  - Create persona from a text block");
        println!("  persona list                         - List all personas");
        println!("  persona show <name>                  - Show persona fingerprint");
        println!("  persona compare <a> <b>              - Compare two personas");
        println!("  persona impersonate <name> \"prompt\"  - Compose as persona");
        println!("  persona match \"unknown text\"         - Attribute text to a persona");
        println!("  persona chat <name>                  - Chat with a persona interactively");
    }

    fn add(&self, ctx: &mut Context, rest: &str) -> bool {
        let parts = parse_quoted(rest);
        if parts.len() < 2 {
            println!("  Usage: persona add <name> \"example 1\" \"example 2\" ...");
            return true;
        }
        let name = parts[0].clone();
        let examples: Vec<String> = parts[1..].to_vec();
        ctx.world.add_persona(&name, &examples, ctx.manifold, ctx.trainer);
        if let Some(p) = ctx.world.get(&name) { println!("{}", p); }
        true
    }

    fn from(&self, ctx: &mut Context, rest: &str) -> bool {
        let parts = parse_quoted(rest);
        if parts.len() < 2 {
            println!("  Usage: persona from <name> \"block of their text\"");
            println!("  The text is auto-split into sentences as examples.");
            return true;
        }
        let name = parts[0].clone();
        let raw_text = parts[1..].join(" ");
        let examples = crate::tokenizer::Tokenizer::split_sentences(&raw_text);
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
            println!("  Usage: persona show <name>");
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
            println!("  Usage: persona compare <name_a> <name_b>");
            return true;
        }
        match ctx.world.compare(parts[0], parts[1]) {
            Some(cmp) => println!("{}", cmp),
            None => println!("  One or both personas not found"),
        }
        true
    }

    fn impersonate(&self, ctx: &mut Context, rest: &str) -> bool {
        let parts = parse_quoted(rest);
        if parts.len() < 2 {
            println!("  Usage: persona impersonate <name> \"prompt\"");
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
        let parts = parse_quoted(rest);
        if parts.is_empty() {
            println!("  Usage: persona match \"unknown text\"");
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

fn parse_quoted(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = '"';

    for ch in s.chars() {
        if !in_quotes && (ch == '"' || ch == '\'') {
            in_quotes = true;
            quote_char = ch;
            current.clear();
        } else if in_quotes && ch == quote_char {
            in_quotes = false;
            if !current.is_empty() { parts.push(current.clone()); }
            current.clear();
        } else if !in_quotes && ch.is_whitespace() {
            if !current.is_empty() { parts.push(current.clone()); }
            current.clear();
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() { parts.push(current); }
    parts
}
