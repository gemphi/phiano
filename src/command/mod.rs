pub mod compose;
pub mod define;
pub mod eval;
pub mod generate;
pub mod help;
pub mod instruction;
pub mod layers;
pub mod learn;
pub mod oscillator;
pub mod persona;
pub mod reasoning;
pub mod resonance;
pub mod save;
pub mod stats;
pub mod synthetic;
pub mod synonym;
pub mod wave;

use crate::facet::Facet;
use crate::generate::ContextWaveBuffer;
use crate::memory::Memo;
use crate::persona::World;
use crate::trainer::Trainer;

/// Shared context passed to every command handler.
///
/// Provides mutable access to the facet, memory, world, and multi-turn context buffer.
pub struct Context<'a> {
    /// The facet — mutable so commands can train or modify the lexicon.
    pub manifold: &'a mut Facet,
    /// The trainer — shared across commands for consistent learning.
    pub trainer: &'a Trainer,
    /// The memo — mutable so commands can record interactions.
    pub memory: &'a mut Memo,
    /// The world — mutable so persona commands can add/modify personas.
    pub world: &'a mut World,
    /// Multi-turn context wave buffer for conversational continuity.
    pub context_buffer: &'a mut ContextWaveBuffer,
    /// The argument string after the command name (trimmed).
    pub arg: &'a str,
    /// The full raw input line.
    pub line: &'a str,
}

/// Command — core REPL commands only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Command {
    Help,
    Learn,
    Define,
    Eval,
    Compose,
    Generate,
    Instruct,
    Reason,
    Layers,
    Synthetic,
    Oscillator,
    Persona,
    Synonym,
    Resonance,
    Wave,
    Save,
    Load,
    Stats,
    Exit,
    Unknown,
}

impl Command {
    /// Parses a command name string into a `Command` variant.
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "help" | "?" => Self::Help,
            "learn" => Self::Learn,
            "define" => Self::Define,
            "eval" | "judge" => Self::Eval,
            "compose" => Self::Compose,
            "generate" | "gen" => Self::Generate,
            "instruct" | "instruction" | "ask" => Self::Instruct,
            "reason" | "solve" | "chain" => Self::Reason,
            "layers" | "hierarchy" | "tree" => Self::Layers,
            "synthetic" | "synth" => Self::Synthetic,
            "oscillator" | "om" => Self::Oscillator,
            "persona" => Self::Persona,
            "synonym" | "synonyms" => Self::Synonym,
            "resonance" => Self::Resonance,
            "wave" => Self::Wave,
            "save" => Self::Save,
            "load" => Self::Load,
            "stats" => Self::Stats,
            "exit" | "quit" => Self::Exit,
            _ => Self::Unknown,
        }
    }
}

/// Dispatcher — routes input lines to the appropriate command handler.
pub struct Dispatcher;

impl Dispatcher {
    /// Dispatches a command line to the appropriate handler.
    pub fn dispatch<'a>(line: &'a str, ctx: &mut Context<'a>) -> bool {
        // Drivers first — source/device commands are separate from core
        if let Some(result) = crate::drivers::Driver::try_dispatch(line, ctx) {
            return result;
        }

        let parts: Vec<&str> = line.splitn(2, char::is_whitespace).collect();
        let cmd = Command::from_str(parts[0]);
        ctx.arg = parts.get(1).map(|s| s.trim()).unwrap_or("");

        match cmd {
            Command::Help => help::Help.apply(ctx),
            Command::Learn => learn::Learn.apply(ctx),
            Command::Define => define::Define.apply(ctx),
            Command::Eval => eval::Eval.apply(ctx),
            Command::Compose => compose::Compose.apply(ctx),
            Command::Generate => generate::GenerateCmd.apply(ctx),
            Command::Instruct => instruction::InstructionCmd.apply(ctx),
            Command::Reason => reasoning::ReasoningCmd.apply(ctx),
            Command::Layers => layers::LayersCmd.apply(ctx),
            Command::Synthetic => synthetic::SyntheticCmd.apply(ctx),
            Command::Oscillator => oscillator::OscillatorCmd.apply(ctx),
            Command::Persona => persona::PersonaCmd.apply(ctx),
            Command::Synonym => synonym::Synonym.apply(ctx),
            Command::Resonance => resonance::Resonance.apply(ctx),
            Command::Wave => wave::WaveCmd.apply(ctx),
            Command::Save => save::Save.save(ctx),
            Command::Load => save::Save.load(ctx),
            Command::Stats => stats::Stats.apply(ctx),
            Command::Exit => false,
            Command::Unknown => learn::Learn.default(ctx),
        }
    }
}

/// Parser — utility for parsing command arguments.
pub struct Parser;

impl Parser {
    /// Strips surrounding single or double quotes from a string.
    ///
    /// Returns the string unchanged if it's not quoted.
    pub fn strip_quotes(s: &str) -> String {
        let s = s.trim();
        let is_double_quoted = s.starts_with('"') && s.ends_with('"');
        let is_single_quoted = s.starts_with('\'') && s.ends_with('\'');

        if is_double_quoted || is_single_quoted {
            s[1..s.len() - 1].to_string()
        } else {
            s.to_string()
        }
    }
}
