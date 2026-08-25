/// Benchmark report formatting: human-readable output and comparison.

use super::benchmark_runner::BenchmarkReport;
use std::fmt;

impl fmt::Display for BenchmarkReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "═══ Phiano Benchmark Report ═══")?;
        writeln!(f)?;
        writeln!(f, "  Baselines:")?;
        writeln!(f, "    Random:     {:.4}", self.baselines.0)?;
        writeln!(f, "    Frequency:  {:.4}", self.baselines.1)?;
        writeln!(f, "    Phase:      {:.4}", self.baselines.2)?;
        writeln!(f)?;
        writeln!(f, "  Robustness:")?;
        writeln!(f, "    Brittleness:      {:.4}", self.brittleness)?;
        writeln!(f, "    OOD Score:        {:.4}", self.ood_score)?;
        writeln!(f)?;
        writeln!(f, "  Adaptation:")?;
        writeln!(f, "    Efficiency:       {:.4}", self.adaptation_efficiency)?;
        writeln!(f, "    Novel Task Score: {:.4}", self.novel_task_score)?;
        writeln!(f)?;
        writeln!(f, "  Generalization:")?;
        writeln!(f, "    Local:    {:.4}", self.generalization.local_score)?;
        writeln!(f, "    Extreme:  {:.4}", self.generalization.extreme_score)?;
        writeln!(f, "    Gap:      {:.4}", self.generalization.gap)?;
        writeln!(f)?;

        if let Some(arc) = &self.arc_results {
            writeln!(f, "  ARC Tasks:")?;
            writeln!(f, "    Total: {}  Correct: {}  Partial: {}  Failed: {}",
                arc.total, arc.correct, arc.partial, arc.failed)?;
        }

        if !self.shortcut_warnings.is_empty() {
            writeln!(f, "  Shortcut Warnings:")?;
            for w in &self.shortcut_warnings {
                writeln!(f, "    [{}] {:.0}% — {}", w.shortcut_type, w.severity * 100.0, w.description)?;
            }
        }

        writeln!(f)?;
        write!(f, "═══ End Report ═══")
    }
}

/// Compares two reports and returns a diff string.
pub fn compare_reports(old: &BenchmarkReport, new: &BenchmarkReport) -> String {
    let mut lines = Vec::new();

    let coh_delta = new.baselines.2 - old.baselines.2;
    lines.push(format!("Phase baseline:    {:.4} → {:.4} ({:+.4})", old.baselines.2, new.baselines.2, coh_delta));

    let brit_delta = new.brittleness - old.brittleness;
    lines.push(format!("Brittleness:       {:.4} → {:.4} ({:+.4})", old.brittleness, new.brittleness, brit_delta));

    let adapt_delta = new.adaptation_efficiency - old.adaptation_efficiency;
    lines.push(format!("Adaptation:        {:.4} → {:.4} ({:+.4})", old.adaptation_efficiency, new.adaptation_efficiency, adapt_delta));

    let gap_delta = new.generalization.gap - old.generalization.gap;
    lines.push(format!("Gen gap:           {:.4} → {:.4} ({:+.4})", old.generalization.gap, new.generalization.gap, gap_delta));

    lines.join("\n")
}
