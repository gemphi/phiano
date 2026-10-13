# The code reader writing

Trained on 99 files, 6135 tokens, vocabulary 176 (last fold). Asked to write the body of 95 function signatures from `code_quest`, each family held out in turn (Phi-4's answers to the other families in training).

| writer | parses as Rust | compiles (rustc) | works (family's asserts pass) | mean tokens written | Phi-4's body reproduced from its start |
|---|---|---|---|---|---|
| 4-gram, greedy | 52 of 95 (55%) | 0 (0%) | 0 of 95 (0%) | 39.8 | 8.6% |
| 4-gram ⊕ movement, greedy | 44 of 95 (46%) | 10 (11%) | 0 of 95 (0%) | 40.7 | 13.8% |
| the same + signature as condition (relations, copy), greedy | 32 of 95 (34%) | 7 (7%) | 0 of 95 (0%) | 61.4 | 14.5% |
| the same, beam 8, first that compiles | 67 of 95 (71%) | 28 (29%) | 3 of 95 (3%) | 13.0 | 10.9% |
| family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam | 61 of 95 (64%) | 61 (64%) | 8 of 95 (8%) | 31.3 | 13.4% |
| **Phi-4 itself (the teacher)** | – | – | 90 of 95 (95%) | – | 100% |

## Examples

**the same + signature as condition (relations, copy), greedy**, `cumulative_i32` (compiles)

```rust
fn cumulative(v : &[i32]) -> Vec < i32 > {
 v.iter().cloned().rev().collect() }
```

**the same + signature as condition (relations, copy), greedy**, `cumulative_i64` (compiles)

```rust
fn cumulative(v : &[i64]) -> Vec < i64 > {
 v.iter().cloned().rev().collect() }
```

**the same + signature as condition (relations, copy), greedy**, `even_i32` (compiles)

```rust
fn count_even(v : &[i32]) -> usize {
 v.iter().filter(| &&x | x > x).count() }
```

**the same + signature as condition (relations, copy), greedy**, `even_i64` (compiles)

```rust
fn count_even(v : &[i64]) -> usize {
 v.iter().filter(| &&x | x > x).count() }
```

**the same + signature as condition (relations, copy), greedy**, `even_u32` (compiles)

```rust
fn count_even(v : &[u32]) -> usize {
 v.iter().filter(| &&x | x > x).count() }
```

**the same, beam 8, first that compiles**, `factorial_i32` (compiles)

```rust
fn factorial(n : i32) -> i32 {
 n.abs() }
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

**family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam**, `min_i32` (works)

```rust
fn min(v : &[i32]) -> Option < i32 > {
 v.iter().min().copied() }
```

