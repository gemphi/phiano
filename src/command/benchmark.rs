/// Benchmark command: runs all metrics and prints a comprehensive report.

use crate::command::Context;
use crate::metrics::benchmark_runner::BenchmarkRunner;
use crate::metrics::benchmark_runner::BenchmarkReport;
use crate::lifelong::history::BenchmarkHistory;

pub struct Benchmark;

impl Benchmark {
    pub fn apply(&self, ctx: &mut Context) -> bool {
        println!("\n  ═══ Running Phiano Benchmark Suite ═══\n");

        let report = BenchmarkRunner::run_all(ctx.manifold, ctx.trainer);
        println!("{}", report);

        let history = BenchmarkHistory::load("data/benchmark_history.json");
        if let Some(latest) = history.latest() {
            println!("\n  ── Comparison with previous benchmark ──");
            println!("  {}", BenchmarkReport::compare(&latest.report, &report));
        }

        let mut new_history = history;
        new_history.record(report);
        new_history.save("data/benchmark_history.json");

        println!("\n  Benchmark recorded to data/benchmark_history.json");
        true
    }
}
