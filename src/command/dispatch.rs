#[allow(unused_imports)]
use super::*;

impl Command {
    /// Parses a command name string into a `Command` variant.
    pub fn from_name(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "help" | "?" => Self::Help,
            "learn" => Self::Learn,
            "define" => Self::Define,
            "eval" | "judge" => Self::Eval,
            "compose" => Self::Compose,
            "generate" | "gen" => Self::Generate,
            "instruct" | "instruction" | "ask" => Self::Instruct,
            "chat" => Self::Chat,
            "reason" | "solve" | "chain" => Self::Reason,
            "layers" | "hierarchy" | "tree" => Self::Layers,
            "synthetic" | "synth" => Self::Synthetic,
            "oscillator" | "om" => Self::Oscillator,
            "persona" => Self::Persona,
            "cognitive" | "cog" => Self::Cognitive,
            "synonym" | "synonyms" => Self::Synonym,
            "resonance" => Self::Resonance,
            "wave" => Self::Wave,
            "save" => Self::Save,
            "load" => Self::Load,
            "stats" => Self::Stat,
            "benchmark" | "bench" => Self::Benchmark,
            "exit" | "quit" => Self::Exit,
            _ => Self::Unknown,
        }
    }
}

impl Dispatcher {
    /// Dispatches a command line to the appropriate handler.
    pub fn dispatch<'a>(line: &'a str, ctx: &mut Context<'a>) -> bool {
        // Drivers first - source/device commands are separate from core
        if let Some(result) = crate::drivers::Driver::try_dispatch(line, ctx) {
            return result;
        }

        let parts: Vec<&str> = line.splitn(2, char::is_whitespace).collect();
        let cmd = Command::from_name(parts[0]);
        ctx.arg = parts.get(1).map(|s| s.trim()).unwrap_or("");

        match cmd {
            Command::Help => help::Help.apply(ctx),
            Command::Learn => learn::Learn.apply(ctx),
            Command::Define => define::Define.apply(ctx),
            Command::Eval => eval::Eval.apply(ctx),
            Command::Compose => compose::Compose.apply(ctx),
            Command::Generate => generate::GenerateCmd.apply(ctx),
            Command::Instruct => instruction::InstructionCmd.apply(ctx),
            Command::Chat => chat::ChatCmd.apply(ctx),
            Command::Reason => reasoning::ReasoningCmd.apply(ctx),
            Command::Layers => layers::LayersCmd.apply(ctx),
            Command::Synthetic => synthetic::SyntheticCmd.apply(ctx),
            Command::Oscillator => oscillator::OscillatorCmd.apply(ctx),
            Command::Persona => persona::PersonaCmd.apply(ctx),
            Command::Cognitive => cognitive::CognitiveCmd.apply(ctx),
            Command::Synonym => synonym::Synonym.apply(ctx),
            Command::Resonance => resonance::Resonance.apply(ctx),
            Command::Wave => wave::WaveCmd.apply(ctx),
            Command::Save => save::Save.save(ctx),
            Command::Load => save::Save.load(ctx),
            Command::Stat => stats::Stat.apply(ctx),
            Command::Benchmark => benchmark::Benchmark.apply(ctx),
            Command::Exit => false,
            Command::Unknown => learn::Learn.default(ctx),
        }
    }
}

impl Parser {
    /// Strips surrounding single or double quotes from a string.
    ///
    /// Returns the string unchanged if it's not quoted.
    /// Splits `"quoted text" 8` into the text and the trailing count.
    ///
    /// The obvious version — `split_whitespace()` then `strip_quotes(parts[0])`
    /// — silently truncates every multi-word argument to its first token. The
    /// `resonance` command shipped with exactly that, so
    /// `resonance "a large four-footed animal" 8` queried the word `a` and
    /// returned `a, the, to, or, in`. It looked like a bag-of-words failure of
    /// the model and was a parser reading one word.
    ///
    /// Unquoted input keeps a trailing integer as the count only when there is
    /// other text before it, so `synonym 7 3` still asks about the word `7`.
    pub fn text_and_count(arg: &str, default: usize) -> (String, usize) {
        let arg = arg.trim();

        // Quoted: everything inside the quotes is the text, anything after is
        // the count.
        for q in ['"', '\''] {
            if let Some(rest) = arg.strip_prefix(q) {
                if let Some(end) = rest.find(q) {
                    let text = rest[..end].to_string();
                    let count = rest[end + 1..]
                        .trim()
                        .parse()
                        .unwrap_or(default);
                    return (text, count);
                }
            }
        }

        let parts: Vec<&str> = arg.split_whitespace().collect();
        match parts.len() {
            0 => (String::new(), default),
            1 => (parts[0].to_string(), default),
            n => match parts[n - 1].parse::<usize>() {
                Ok(c) => (parts[..n - 1].join(" "), c),
                Err(_) => (parts.join(" "), default),
            },
        }
    }

    pub fn strip_quotes(s: &str) -> String {
        let s = s.trim();
        let is_quoted = (s.starts_with('"') && s.ends_with('"'))
            || (s.starts_with('\'') && s.ends_with('\''));

        match is_quoted {
            true => s[1..s.len() - 1].to_string(),
            false => s.to_string(),
        }
    }
}
