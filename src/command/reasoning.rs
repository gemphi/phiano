use crate::command::Context;
use crate::reasoning::ReasoningEngine;

pub struct ReasoningCmd;

impl ReasoningCmd {
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: reason <problem or question>");
            println!("  Example: reason solve concurrency thread safety and mutex locking");
            return true;
        }

        let problem = crate::command::Parser::strip_quotes(ctx.arg);
        let engine = ReasoningEngine;

        println!("  ── phase-space pathfinding & reasoning ──");
        println!("  problem: \"{}\"\n", problem);

        let chain = engine.solve(ctx.manifold, &problem);

        for step in &chain.steps {
            println!(
                "  Step {:>2}: focus -> {:<16} | phase {:>6.3} rad | delta {:>6.4}",
                step.step_number, step.focus_word, step.phase_angle, step.phase_delta
            );
        }

        println!("\n  Convergence Status: {}", if chain.converged { "CONVERGED (Harmonic Equilibrium)" } else { "TRAVERSED" });
        println!("  Path Chain: {}\n", chain.final_answer);

        // Update context buffer
        ctx.context_buffer.push_turn(ctx.manifold, &problem);
        ctx.context_buffer.push_turn(ctx.manifold, &chain.final_answer);

        true
    }
}
