use crate::command::Context;
use crate::layers::HierarchicalPhaseField;
use crate::tokenizer::Tokenizer;
use crate::wave::Wave;

pub struct LayersCmd;

impl LayersCmd {
    pub fn apply(&self, ctx: &mut Context) -> bool {
        let mut field = HierarchicalPhaseField::new();
        field.build_hierarchy(ctx.manifold);

        println!("  ── 4-layer hierarchical phase field ──");
        println!("  Layer 0: Surface Words      ({} phasors)", ctx.manifold.vocabulary_size());
        for level in 1..=3 {
            let layer = &field.layers[level];
            let name = match level {
                1 => "Concept Clusters",
                2 => "Domain Sectors",
                3 => "Meta-Patterns",
                _ => "Sub-Band",
            };
            println!(
                "  Layer {}: {:<18} ({} active clusters / {} max sectors)",
                level, name, layer.clusters.len(), layer.sector_count
            );
        }

        if !ctx.arg.is_empty() {
            let query = crate::command::Parser::strip_quotes(ctx.arg);
            let tokens = Tokenizer::tokenize(&query);
            let wave = Wave::sentence(ctx.manifold, &tokens);
            let target_phase = (wave.im.atan2(wave.re)).rem_euclid(2.0 * std::f64::consts::PI);

            println!("\n  Resonance Depth for \"{}\" (phase {:.3} rad):", query, target_phase);
            let depth_res = field.resonate_depth(target_phase);
            for (level, sector, diff) in depth_res {
                let name = match level {
                    1 => "Concept Cluster",
                    2 => "Domain Sector",
                    3 => "Meta-Pattern",
                    _ => "Layer",
                };
                println!("    Layer {} [{}]: sector {} | phase distance: {:.4} rad", level, name, sector, diff);
            }
        }
        println!();

        true
    }
}
