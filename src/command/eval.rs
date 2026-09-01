use crate::command::Context;
use crate::command::Parser;
use crate::envision::Envision;
use crate::eval::Evaluator;

/// Eval - evaluates text against the facet's semantic space.
///
/// Usage: `eval "some text to evaluate"`
///
/// Reports coherence, novelty, resonance, and an overall verdict.
/// Also runs the envision phase to detect knowledge gaps.
pub struct Eval;

impl Eval {
    /// Evaluates the provided text and prints the results.
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: eval \"some text to evaluate\"");
            return true;
        }

        let text = Parser::strip_quotes(ctx.arg);
        let eval = Evaluator::new().eval(ctx.manifold, &text);
        println!("{}", eval);

        // No dictionary escalation here: the model-level envision pass runs after
        // every input and owns both the chunk store and the persistent gap ledger.
        if let Some(v) = Envision::new().detect_gaps(ctx.manifold, None, &text) {
            println!("{}", v);
        }
        true
    }
}
