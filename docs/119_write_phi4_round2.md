# The code reader writing

Trained on 252 files, 12302 tokens, vocabulary 236 (last fold). Asked to write the body of 248 function signatures from `code_quest`, each family held out in turn (Phi-4's answers to the other families in training).

| writer | parses as Rust | compiles (rustc) | works (family's asserts pass) | mean tokens written | Phi-4's body reproduced from its start |
|---|---|---|---|---|---|
| 4-gram, greedy | 33 of 248 (13%) | 0 (0%) | 0 of 248 (0%) | 39.5 | 15.8% |
| 4-gram ⊕ movement, greedy | 195 of 248 (79%) | 23 (9%) | 1 of 248 (0%) | 36.5 | 22.2% |
| the same + signature as condition (relations, copy), greedy | 191 of 248 (77%) | 75 (30%) | 1 of 248 (0%) | 24.8 | 22.5% |
| the same, beam 8, first that compiles | 228 of 248 (92%) | 151 (61%) | 2 of 248 (1%) | 9.8 | 13.4% |
| family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam | 216 of 248 (87%) | 216 (87%) | 14 of 248 (6%) | 28.1 | 20.5% |
| proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam | 220 of 248 (89%) | 220 (89%) | 63 of 248 (25%) | 29.1 | 27.2% |
| **Phi-4 itself (the teacher)** | – | – | 224 of 248 (90%) | – | 100% |

## Examples

**the same + signature as condition (relations, copy), greedy**, `above_i32` (compiles)

```rust
fn count_above(v : &[i32], x : i32) -> usize {
 v.iter().filter(| &&y | y == x).count() }
```

**proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam**, `above_i32` (works)

```rust
fn count_above(v : &[i32], x : i32) -> usize {
 v.iter().filter(| &&y | y > x).map(| &y | y * 2).count() }
```

**the same + signature as condition (relations, copy), greedy**, `above_i64` (compiles)

```rust
fn count_above(v : &[i64], x : i64) -> usize {
 v.iter().filter(| &&y | y == x).count() }
```

**proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam**, `above_i64` (works)

```rust
fn count_above(v : &[i64], x : i64) -> usize {
 v.iter().filter(| &&y | y > x).map(| &y | y * y).count() }
```

**the same + signature as condition (relations, copy), greedy**, `above_u32` (compiles)

```rust
fn count_above(v : &[u32], x : u32) -> usize {
 v.iter().filter(| &&y | y == x).count() }
```

**proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam**, `above_u32` (works)

```rust
fn count_above(v : &[u32], x : u32) -> usize {
 v.iter().filter(| &&y | y > x).map(| &y | y * 2).count() }
```

**proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam**, `above_f64` (works)

```rust
fn count_above(v : &[f64], x : f64) -> usize {
 v.iter().filter(| &&y | y > x).map(| &y | y * y).count() }
```

**the same, beam 8, first that compiles**, `abs_diff_i32` (compiles)

```rust
fn abs_diff(a : i32, b : i32) -> i32 {
 if a > b {
 a }
 else {
 b }
 }
```

**family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam**, `max_u32` (works)

```rust
fn max(v : &[u32]) -> Option < u32 > {
 v.iter().max().copied() }
```

**family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam**, `min_i32` (works)

```rust
fn min(v : &[i32]) -> Option < i32 > {
 v.iter().min().copied() }
```

**family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam**, `min_u32` (works)

```rust
fn min(v : &[u32]) -> Option < u32 > {
 v.iter().min().copied() }
```

**family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam**, `sorted_i32` (works)

```rust
fn is_sorted(v : &[i32]) -> bool {
 v.iter().is_sorted() }
```

