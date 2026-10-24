//! The demo's report, as markdown.

use super::train::Reading;

pub struct Report {
    names: Vec<String>,
    epochs: usize,
    knob: Vec<String>,
    stages: Vec<(bool, String)>,
}

impl Report {
    pub fn new(names: &[String], epochs: usize) -> Self {
        Self { names: names.to_vec(), epochs, knob: Vec::new(), stages: Vec::new() }
    }

    pub fn knob(&mut self, r: usize, numbers: usize, x: &Reading, secs: f64) {
        let line = format!(
            "| {r} | {numbers} | {:.3} | {:.3} | {:.3} | {:.1}% | {:.3} / {:.3} | {secs:.0} s |",
            x.told.mrr,
            x.held.mrr,
            x.held.within,
            100.0 * x.held.hit5,
            x.prior.mrr,
            x.prior.within
        );
        println!("{line}");
        self.knob.push(line);
    }

    pub fn stage(&mut self, keep: bool, i: usize, learned: &str, read: &str, x: &Reading) {
        let line = format!("| {} | {learned} | {read} | {:.3} | {:.3} | {:.3} | {:.3} |", i + 1, x.told.mrr, x.held.mrr, x.held.within, x.prior.mrr);
        println!("{} {line}", if keep { "keep " } else { "naive" });
        self.stages.push((keep, line));
    }

    pub fn markdown(&self) -> String {
        let mut s = format!(
            "# Learning demo\n\nPersonas: {} · {} epochs a stage · a fifth of each persona's rows held out.\n\n## 1. One knob: relations per word\n\n| relations per word (R) | numbers | told MRR | held-out MRR | held-out within kind | held-out hit@5 | prior MRR / within | time |\n|---|---|---|---|---|---|---|---|\n",
            self.names.join(", "),
            self.epochs
        );
        for l in &self.knob {
            s += l;
            s += "\n";
        }
        for (keep, title) in [(false, "naive — each stage trains on the new persona only"), (true, "keep — each stage trains on the new persona and every earlier told row")] {
            s += &format!("\n## 2. Learning one persona after another: {title}\n\n| stage | learned | read | told MRR | held-out MRR | held-out within kind | prior MRR |\n|---|---|---|---|---|---|---|\n");
            for (k, l) in &self.stages {
                if *k == keep {
                    s += l;
                    s += "\n";
                }
            }
        }
        s
    }
}
