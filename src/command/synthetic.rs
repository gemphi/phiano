use crate::command::Context;
use crate::synthetic::{SyntheticCurriculumPipeline, SyntheticGenerator};

pub struct SyntheticCmd;

impl SyntheticCmd {
    pub fn apply(&self, ctx: &mut Context) -> bool {
        let parts: Vec<&str> = ctx.arg.split_whitespace().collect();
        let mode = parts.get(0).map(|s| *s).unwrap_or("pairs");

        match mode {
            "pairs" | "contrast" => {
                println!("  ── generating synthetic contrast pairs ──");
                let pairs = SyntheticGenerator::generate_contrast_pairs(ctx.manifold);
                println!("  Generated {} contrast / synonym pairs:", pairs.len());
                for (i, p) in pairs.iter().take(12).enumerate() {
                    println!("    #{:>2}: {:<16} <── {:^8} ──> {:<16}", i + 1, p.term_a, p.relationship, p.term_b);
                }
                println!();
            }
            "pipeline" | "curriculum" | "train" => {
                println!("  ── running synthetic curriculum self-training pipeline ──");
                let pipeline = SyntheticCurriculumPipeline::new(0.35, 0.60);
                let count = pipeline.run_pipeline(ctx.manifold, ctx.trainer);
                println!("  [synthetic] successfully generated & trained on {} high-quality sentences.\n", count);
            }
            _ => {
                println!("  Usage: synthetic [pairs | pipeline]");
            }
        }

        true
    }
}
