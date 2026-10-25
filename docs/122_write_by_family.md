# The code reader writing

Trained on 252 files, 12302 tokens, vocabulary 238 (last fold). Asked to write the body of 248 function signatures from `code_quest`, each family held out in turn (Phi-4's answers to the other families in training).

| writer | parses as Rust | compiles (rustc) | works (family's asserts pass) | mean tokens written | Phi-4's body reproduced from its start |
|---|---|---|---|---|---|
| 4-gram, greedy | 16 of 248 (6%) | 0 (0%) | 0 of 248 (0%) | 98.4 | 15.8% |
| 4-gram ⊕ movement, greedy | 215 of 248 (87%) | 15 (6%) | 0 of 248 (0%) | 34.2 | 19.2% |
| the same + signature as condition (relations, copy), greedy | 189 of 248 (76%) | 75 (30%) | 0 of 248 (0%) | 25.8 | 21.2% |
| the same, beam 8, first that compiles | 230 of 248 (93%) | 146 (59%) | 1 of 248 (0%) | 10.9 | 16.4% |
| family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam | 217 of 248 (88%) | 217 (88%) | 15 of 248 (6%) | 25.5 | 19.9% |
| proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam | 221 of 248 (89%) | 221 (89%) | 62 of 248 (25%) | 26.6 | 25.3% |
| the same, with Phi-4's glosses of the words (above ⇒ >, below ⇒ <) | 221 of 248 (89%) | 221 (89%) | 68 of 248 (27%) | 25.2 | 28.6% |
| the same, with Phi-4's expressions for the words (even ⇒ x % 2 == 0) | 221 of 248 (89%) | 221 (89%) | 68 of 248 (27%) | 25.2 | 28.6% |
| **Phi-4 itself (the teacher)** | – | – | 224 of 248 (90%) | – | 100% |

## Examples

**the same, beam 8, first that compiles**, `above_i32` (compiles)

```rust
fn count_above(v : &[i32], x : i32) -> usize {
 v.iter().filter(| &&y | y < x).count() }
```

**proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam**, `above_i32` (works)

```rust
fn count_above(v : &[i32], x : i32) -> usize {
 v.iter().filter(| &&y | y > x).map(| &y | y * 2).count() }
```

**the same, with Phi-4's glosses of the words (above ⇒ >, below ⇒ <)**, `above_i32` (works)

```rust
fn count_above(v : &[i32], x : i32) -> usize {
 v.iter().filter(| &&y | y > x).count() }
```

**the same, with Phi-4's expressions for the words (even ⇒ x % 2 == 0)**, `above_i32` (works)

```rust
fn count_above(v : &[i32], x : i32) -> usize {
 v.iter().filter(| &&y | y > x).count() }
```

**the same, beam 8, first that compiles**, `above_i64` (compiles)

```rust
fn count_above(v : &[i64], x : i64) -> usize {
 v.iter().filter(| &&y | y < x).count() }
```

**proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam**, `above_i64` (works)

```rust
fn count_above(v : &[i64], x : i64) -> usize {
 v.iter().filter(| &&y | y > x).map(| &y | y * y).count() }
```

**the same, with Phi-4's glosses of the words (above ⇒ >, below ⇒ <)**, `above_i64` (works)

```rust
fn count_above(v : &[i64], x : i64) -> usize {
 v.iter().filter(| &&y | y > x).count() }
```

**the same, with Phi-4's expressions for the words (even ⇒ x % 2 == 0)**, `above_i64` (works)

```rust
fn count_above(v : &[i64], x : i64) -> usize {
 v.iter().filter(| &&y | y > x).count() }
```

**proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam**, `above_u32` (works)

```rust
fn count_above(v : &[u32], x : u32) -> usize {
 v.iter().filter(| &&y | y > x).map(| &y | y * 2).count() }
```

**the same, with Phi-4's glosses of the words (above ⇒ >, below ⇒ <)**, `above_u32` (works)

```rust
fn count_above(v : &[u32], x : u32) -> usize {
 v.iter().filter(| &&y | y > x).count() }
```

**the same, with Phi-4's expressions for the words (even ⇒ x % 2 == 0)**, `above_u32` (works)

```rust
fn count_above(v : &[u32], x : u32) -> usize {
 v.iter().filter(| &&y | y > x).count() }
```

**proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam**, `above_f64` (works)

```rust
fn count_above(v : &[f64], x : f64) -> usize {
 v.iter().filter(| &&y | y > x).map(| &y | y * y).count() }
```

**the same, with Phi-4's glosses of the words (above ⇒ >, below ⇒ <)**, `above_f64` (works)

```rust
fn count_above(v : &[f64], x : f64) -> usize {
 v.iter().filter(| &&y | y > x).count() }
```

**the same, with Phi-4's expressions for the words (even ⇒ x % 2 == 0)**, `above_f64` (works)

```rust
fn count_above(v : &[f64], x : f64) -> usize {
 v.iter().filter(| &&y | y > x).count() }
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

