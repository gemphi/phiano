use crate::command::Context;
use crate::command::Parser;

/// Cognitive - runs the full 16-agent Searle-inspired cognitive pipeline.
///
/// Usage: `cognitive "a dog is a 4 legged animal"`
///
/// Processes the sentence through all 16 cognitive agents and prints
/// a structured breakdown of speech act classification, intentionality,
//  semantics, reference, truth conditions, social ontology, and synthesis.
pub struct CognitiveCmd;

impl CognitiveCmd {
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: cognitive \"a sentence to analyze\"");
            println!("  Example: cognitive \"a dog is a 4 legged animal\"");
            return true;
        }

        let text = Parser::strip_quotes(ctx.arg);
        let result = ctx.cognitive_core.process(
            ctx.manifold,
            ctx.context_buffer,
            &text,
        );

        println!("\n  ╔══════════════════════════════════════════════════════════════╗");
        println!("  ║  COGNITIVE ANALYSIS - 16-Agent Searle Pipeline               ║");
        println!("  ╚══════════════════════════════════════════════════════════════╝\n");

        println!("  ┌─ Prompt: \"{}\"", text);
        println!("  │  Coherence: {:.3}", result.coherence);
        println!("  │\n");

        println!("  ├─ SPEECH ACT ──────────────────────────────────────────────");
        println!("  │  Type:              {}", result.speech_act);
        println!("  │  Direction of fit:  {}", result.direction_of_fit);
        println!("  │  Propositional:     \"{}\"", result.propositional_content);
        println!("  │  Perlocutionary:    {}", result.perlocutionary_effect);
        println!("  │  Literal meaning:   \"{}\"", result.literal_meaning);
        println!("  │  Speaker meaning:   \"{}\"", result.speaker_meaning);
        println!("  │  Felicity:          {}", if result.felicity_conditions.satisfied { "met" } else { "unmet" });
        println!("  │    Content rule:    {}", result.felicity_conditions.propositional_content_rule);
        println!("  │    Preparatory:     {}", result.felicity_conditions.preparatory_condition);
        println!("  │    Sincerity:       {}", result.felicity_conditions.sincerity_condition);
        println!("  │    Essential:       {}", result.felicity_conditions.essential_condition);
        println!("  │");

        println!("  ├─ INTENTIONAL STATES ──────────────────────────────────────");
        for state in &result.intentional_states {
            println!("  │  [{}] \"{}\"", state.mode.as_str(), state.content);
            println!("  │    fit: {} | satisfaction: {} | sincerity: {:.2}",
                state.direction_of_fit.as_str(), state.satisfaction_condition, state.sincerity);
        }
        println!("  │");

        println!("  ├─ AGENT OUTPUTS ({} agents) ───────────────────────────────", result.agent_outputs.len());
        for contrib in &result.agent_outputs {
            println!("  │  [{:<18}] conf={:.2} | {}", contrib.agent_name, contrib.confidence, contrib.agent_role);
            for line in contrib.output.lines() {
                println!("  │    {}", line);
            }
        }
        println!("  │");

        println!("  └─ SYNTHESIS ───────────────────────────────────────────────");
        println!("     \"{}\"", result.synthesized_output);
        println!();

        ctx.context_buffer.push_turn(ctx.manifold, &text);
        ctx.context_buffer.push_turn(ctx.manifold, &result.synthesized_output);

        true
    }
}
