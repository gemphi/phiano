# The code reader writing

Trained on 252 files, 12254 tokens, vocabulary 236 (last fold). Asked to write the body of 248 function signatures from `code_quest`, each task held out alone (its type siblings and the other families in training).

| writer | parses as Rust | compiles (rustc) | works (family's asserts pass) | mean tokens written | Phi-4's body reproduced from its start |
|---|---|---|---|---|---|
| 4-gram, greedy | 12 of 248 (5%) | 0 (0%) | 0 of 248 (0%) | 93.7 | 16.4% |
| 4-gram ⊕ movement, greedy | 193 of 248 (78%) | 11 (4%) | 6 of 248 (2%) | 34.9 | 24.1% |
| the same + signature as condition (relations, copy), greedy | 131 of 248 (53%) | 95 (38%) | 23 of 248 (9%) | 43.8 | 32.3% |
| the same, beam 8, first that compiles | 200 of 248 (81%) | 171 (69%) | 25 of 248 (10%) | 11.8 | 23.1% |
| family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam | 243 of 248 (98%) | 243 (98%) | 189 of 248 (76%) | 31.2 | 67.0% |
| proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam | 243 of 248 (98%) | 243 (98%) | 194 of 248 (78%) | 30.5 | 71.0% |
| the same, with Phi-4's glosses of the words (above ⇒ >, below ⇒ <) | 243 of 248 (98%) | 243 (98%) | 197 of 248 (79%) | 29.2 | 68.0% |
| the same, with Phi-4's expressions for the words (even ⇒ x % 2 == 0) | 243 of 248 (98%) | 243 (98%) | 197 of 248 (79%) | 29.2 | 68.0% |
| **Phi-4 itself (the teacher)** | – | – | 224 of 248 (90%) | – | 100% |

## Examples

**the same, beam 8, first that compiles**, `sum_i32` (compiles)

```rust
fn sum(v : &[i32]) -> i32 {
 0 }
```

**family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam**, `sum_i32` (works)

```rust
fn sum(v : &[i32]) -> i32 {
 v.iter().fold(0, | acc, &x | acc + x) }
```

**proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam**, `sum_i32` (works)

```rust
fn sum(v : &[i32]) -> i32 {
 v.iter().fold(0, | acc, &x | acc + x) }
```

**the same, with Phi-4's glosses of the words (above ⇒ >, below ⇒ <)**, `sum_i32` (works)

```rust
fn sum(v : &[i32]) -> i32 {
 v.iter().fold(0, | acc, &x | acc + x) }
```

**the same, with Phi-4's expressions for the words (even ⇒ x % 2 == 0)**, `sum_i32` (works)

```rust
fn sum(v : &[i32]) -> i32 {
 v.iter().fold(0, | acc, &x | acc + x) }
```

**the same, beam 8, first that compiles**, `sum_i64` (compiles)

```rust
fn sum(v : &[i64]) -> i64 {
 0 }
```

**family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam**, `sum_i64` (works)

```rust
fn sum(v : &[i64]) -> i64 {
 v.iter().fold(0, | acc, &x | acc + x) }
```

**proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam**, `sum_i64` (works)

```rust
fn sum(v : &[i64]) -> i64 {
 v.iter().fold(0, | acc, &x | acc + x) }
```

**the same, with Phi-4's glosses of the words (above ⇒ >, below ⇒ <)**, `sum_i64` (works)

```rust
fn sum(v : &[i64]) -> i64 {
 v.iter().fold(0, | acc, &x | acc + x) }
```

**the same, with Phi-4's expressions for the words (even ⇒ x % 2 == 0)**, `sum_i64` (works)

```rust
fn sum(v : &[i64]) -> i64 {
 v.iter().fold(0, | acc, &x | acc + x) }
```

**family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam**, `sum_u32` (works)

```rust
fn sum(v : &[u32]) -> u32 {
 v.iter().fold(0, | acc, &x | acc + x) }
```

**proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam**, `sum_u32` (works)

```rust
fn sum(v : &[u32]) -> u32 {
 v.iter().fold(0, | acc, &x | acc + x) }
```

**the same, with Phi-4's glosses of the words (above ⇒ >, below ⇒ <)**, `sum_u32` (works)

```rust
fn sum(v : &[u32]) -> u32 {
 v.iter().fold(0, | acc, &x | acc + x) }
```

**the same, with Phi-4's expressions for the words (even ⇒ x % 2 == 0)**, `sum_u32` (works)

```rust
fn sum(v : &[u32]) -> u32 {
 v.iter().fold(0, | acc, &x | acc + x) }
```

**family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam**, `sum_f64` (works)

```rust
fn sum(v : &[f64]) -> f64 {
 v.iter().sum() }
```

**proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam**, `sum_f64` (works)

```rust
fn sum(v : &[f64]) -> f64 {
 v.iter().sum() }
```

**the same, with Phi-4's glosses of the words (above ⇒ >, below ⇒ <)**, `sum_f64` (works)

```rust
fn sum(v : &[f64]) -> f64 {
 v.iter().sum() }
```

**the same, with Phi-4's expressions for the words (even ⇒ x % 2 == 0)**, `sum_f64` (works)

```rust
fn sum(v : &[f64]) -> f64 {
 v.iter().sum() }
```

