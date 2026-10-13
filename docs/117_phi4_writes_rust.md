# 117 · Phi-4 writes Rust, and the relation model writes: a baseline

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

**Why.** Up to now the code reader has *predicted* (docs/109–110): given the true tokens so far, rank the next one. Coding, as Phi-4 does it, is *writing*: going on from its own tokens, and ending with a body that works. This doc sets up both halves of the test.

- **The teacher's half (`code_quest`, `scripts/phi4_code.ps1`).** Phi-4 is asked 100 small Rust tasks in 35 families. Most families come in four type variants, so the reader can be tested on whole families it never saw. The answers go to `data/phi4_code/<family>/<task>.rs`. Not run yet.
- **The student's half (`code_write`).** Given a signature, the reader writes greedily until the body's braces close. It passes over any token that would repeat a 4-gram it already wrote. The output is scored by whether it parses as Rust (`syn`). With `--phi4`, the families are split in halves: the reader learns Phi-4's answers to one half and writes the other. It is also scored on how much of Phi-4's own body it reproduces from the start.

## Baseline: written from our own Rust only (691 files, 471,838 tokens)

| reader | parses as Rust | mean tokens written |
|---|---|---|
| 4-gram | 0 of 95 (0%) | 145.5 |
| 4-gram ⊕ movement, learned weights (docs/110) | 5 of 95 (5%) | 84.7 |

The first body that parses, for `fn sum(v: &[f64]) -> f64`:

```rust
fn sum(v : &[f64]) -> f64 {
 let mut out = Vec :: new();
 let mut f = Facet :: new :: < f64 > ().ok()?;
 let mut out : Vec < String >= std :: env :: args().collect();
 }
```

## Reading

- **Predicting is not writing.** The same reader that picks 60% of next tokens correctly (docs/110) writes nothing that works. Once its own first wrong token is in the history, its conditions are those of our phasor code (`Facet`, `Trainer`, `SpectralPhasor`), not of the task. It goes on as *our* code goes on (§185 again: it continues in the way it was trained).
- **Movement helps the form, not the sense.** With the learned weights, 5 of 95 bodies parse, against none for the 4-gram. The bodies are shorter (85 tokens against 146) because movement closes the braces sooner. None of the five does what the signature asks.
- **What this baseline measures is the gap the teacher must fill.** Our code has no `sum` or `is_prime` in it. Phi-4's answers will be the first corpus of the practice we want: short, task-shaped functions. The next numbers, from `--phi4` with families held out, say how much of Phi-4's writing a relation model picks up from about 50 answers.

## Run

`scripts\phi4_code.ps1` asks Phi-4, reads its code with whole families held out (`code_reader --by-dir`, with and without our own Rust), then writes with and without Phi-4's answers (`code_write`). Raw: `117_code_write_baseline.md`.
