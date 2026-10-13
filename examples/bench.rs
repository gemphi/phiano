use phiano::config::CHROMA_FILE;
use phiano::eval::Evaluator;
use phiano::facet::Facet;
use phiano::generate::{ContextWaveBuffer, Generator};
use phiano::storage::Storage;
use phiano::trainer::Trainer;

const PROMPTS: &[&str] = &[
    "the cat sat on the mat",
    "time changes everything in life",
    "knowledge and wisdom come from learning",
    "the mushroom is growing in the forest",
    "ownership and borrowing rules in rust",
    "love and hope give meaning to life",
    "science seeks truth through reason",
    "the child learns language through conversation",
];

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| CHROMA_FILE.to_string());

    let mut facet = match Storage::load(&path) {
        Ok(f) => f,
        Err(_) => {
            println!("  [bench] no facet at {} — training a tiny in-memory set", path);
            let mut f = Facet::new();
            let trainer = Trainer::new(0.05);
            for p in PROMPTS {
                trainer.train_sentence(&mut f, p);
            }
            f
        }
    };

    let evaluator = Evaluator::new();
    let generator = Generator::new(16, 0.15);
    let mut ctx = ContextWaveBuffer::new(256);

    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  PHIANO — coherence / novelty / resonance bench      ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!("  facet: {}  vocab: {}\n", path, facet.vocabulary_size());
    println!(
        "  {:<48} {:>8} {:>8} {:>8} {:>8}",
        "prompt", "coh", "nov", "res", "ovr"
    );

    let mut sum_c = 0.0;
    let mut sum_n = 0.0;
    let mut sum_r = 0.0;
    let mut sum_o = 0.0;
    let mut gen_ok = 0usize;

    for prompt in PROMPTS {
        let e = evaluator.eval(&facet, prompt);
        sum_c += e.coherence;
        sum_n += e.novelty;
        sum_r += e.resonance;
        sum_o += e.overall;
        let generated = generator.generate(&facet, &mut ctx, prompt);
        if !generated.trim().is_empty() {
            gen_ok += 1;
        }
        println!(
            "  {:<48} {:8.3} {:8.3} {:8.3} {:8.3}",
            truncate(prompt, 48),
            e.coherence,
            e.novelty,
            e.resonance,
            e.overall
        );
        if !generated.is_empty() {
            println!("      → {}", truncate(&generated, 72));
        }
        let _ = &mut facet;
    }

    let n = PROMPTS.len() as f64;
    println!("\n  mean coherence={:.3}  novelty={:.3}  resonance={:.3}  overall={:.3}",
        sum_c / n, sum_n / n, sum_r / n, sum_o / n);
    println!("  generated non-empty: {}/{}", gen_ok, PROMPTS.len());
}

fn truncate(s: &str, n: usize) -> String {
    match s.chars().count() > n {
        true => s.chars().take(n.saturating_sub(1)).collect::<String>() + "…",
        false => s.to_string(),
    }
}
