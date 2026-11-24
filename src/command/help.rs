use crate::command::Context;

/// Help - prints the list of available commands.
pub struct Help;

impl Help {
    /// Prints the listing the table holds (`data/command/usage.tsv`,
    /// `command::usage`). The text used to be forty-five `println!` lines
    /// here, a second wording of what every command says of itself.
    pub fn apply(&self, _ctx: &mut Context) -> bool {
        print!("{}", super::usage::listing());
        println!();
        true
    }
}
