# The code reader writing

Trained on 794 files, 485015 tokens, vocabulary 5588 (last fold). Asked to write the body of 95 function signatures from `code_quest`, each family held out in turn (Phi-4's answers to the other families in training).

| writer | parses as Rust | compiles (rustc) | works (family's asserts pass) | mean tokens written | Phi-4's body reproduced from its start |
|---|---|---|---|---|---|
| 4-gram, greedy | 0 of 95 (0%) | 0 (0%) | 0 of 95 (0%) | 145.5 | 0.8% |
| 4-gram ⊕ movement, greedy | 32 of 95 (34%) | 0 (0%) | 0 of 95 (0%) | 82.7 | 2.8% |
| the same + signature as condition (relations, copy), greedy | 11 of 95 (12%) | 0 (0%) | 0 of 95 (0%) | 81.5 | 12.5% |
| the same, beam 8, first that compiles | 82 of 95 (86%) | 58 (61%) | 3 of 95 (3%) | 8.9 | 7.6% |
| family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam | 67 of 95 (71%) | 67 (71%) | 8 of 95 (8%) | 31.3 | 13.4% |
| **Phi-4 itself (the teacher)** | – | – | 90 of 95 (95%) | – | 100% |

## Examples

**the same, beam 8, first that compiles**, `above_i32` (compiles)

```rust
fn count_above(v : &[i32], x : i32) -> usize {
 v.len() }
```

**the same, beam 8, first that compiles**, `above_f64` (compiles)

```rust
fn count_above(v : &[f64], x : f64) -> usize {
 v.len() }
```

**the same, beam 8, first that compiles**, `caesar_str` (compiles)

```rust
fn caesar(s : &str, shift : u8) -> String {
 s.split_whitespace().collect() }
```

**the same, beam 8, first that compiles**, `capital_str` (compiles)

```rust
fn capitalize_words(s : &str) -> String {
 s.split_whitespace().collect() }
```

**family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam**, `max_u32` (works)

```rust
fn max(v : &[u32]) -> Option < u32 > {
 v.iter().max().copied() }
```

**family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam**, `mean_f64` (works)

```rust
fn mean(v : &[f64]) -> Option < f64 > {
 let mut largest = None;
 let mut mean = None;
 for &value in v {
 match largest {
 None => largest = Some(value), Some(l) if value > l => {
 mean = largest;
 largest = Some(value);
 }
 Some(l) if value < l &&(mean.is_none() || value > mean.unwrap()) => {
 mean = Some(value);
 }
 _ => {
 }
 }
 }
 mean }
```

