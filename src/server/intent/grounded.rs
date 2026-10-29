#[allow(unused_imports)]
use super::*;

impl ChatIntent {
    /// Explanation: Wikipedia extract, then the chunk store, then multi-step reasoning.
    pub(super) fn explain(&self, model: &Model, cog: &CognitiveResult, topic: &str, wiki_extract: Option<&str>) -> String {
        let chunk_store = ChunkStore::new(CHUNK_STORE_DIR);
        let def_opt = chunk_store.load_definition(topic);

        if let Some(wiki) = wiki_extract {
            let clean_wiki = wiki.lines().take(4).collect::<Vec<_>>().join(" ");
            return format!(
                "### Semantic Grounding: **{}**\n\n\
                {}\n\n\
                *Attractor Resonance*: Aligned with Wikipedia knowledge base (Coherence: {:.2}).",
                topic.to_uppercase(),
                clean_wiki,
                cog.coherence
            );
        }

        if let Some(def) = def_opt {
            let clean_def = def.lines().take(3).collect::<Vec<_>>().join("\n");
            return format!(
                "### Concept Definition: **{}**\n\n\
                {}\n\n\
                *Memory Layer Resonance*: Grounded across 16 memory layers with {:.0}% satisfaction.",
                topic,
                clean_def,
                cog.satisfaction * 100.0
            );
        }

        // Fall back to multi-step phase reasoning
        let reasoning = model.cognitive_core.reason(
            &model.facet,
            &mut phient::generate::ContextWaveBuffer::new(DEFAULT_CONTEXT_WINDOW),
            topic,
            DEFAULT_REASONING_STEPS,
        );
        format!(
            "### Attractor Trajectory: **{}**\n\n\
            {}\n\n\
            *Cognitive Path*: Multi-step reasoning across {} steps (Convergence: {})",
            topic,
            reasoning.final_answer,
            reasoning.steps.len(),
            match reasoning.converged {
                true => "Harmonic Equilibrium",
                false => "Approximated",
            }
        )
    }

    /// Recommendation: grounded on the wiki extract when there is one, else reasoned.
    pub(super) fn recommend(&self, model: &Model, cog: &CognitiveResult, query: &str, wiki_extract: Option<&str>) -> String {
        if let Some(wiki) = wiki_extract {
            let clean_wiki = wiki.lines().take(4).collect::<Vec<_>>().join(" ");
            return format!(
                "### Contextual Grounding & Recommendations\n\n\
                {}\n\n\
                *Recommendation Path*: Grounded across active memory context and continuous semantic manifold.",
                clean_wiki
            );
        }

        let reasoning = model.cognitive_core.reason(
            &model.facet,
            &mut phient::generate::ContextWaveBuffer::new(DEFAULT_CONTEXT_WINDOW),
            query,
            DEFAULT_REASONING_STEPS,
        );
        format!(
            "### Explorations & Next Steps\n\n\
            {}\n\n\
            *Semantic Coherence*: {:.2} · Active Lexicon: {} words",
            reasoning.final_answer,
            cog.coherence,
            model.facet.vocabulary_size()
        )
    }
}
