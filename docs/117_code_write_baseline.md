# The code reader writing

Trained on 691 files, 471838 tokens, vocabulary 5522 (last half). Asked to write the body of 95 function signatures from `code_quest`.

| reader | parses as Rust | mean tokens written | Phi-4's body reproduced from its start |
|---|---|---|---|
| 4-gram | 0 of 95 (0%) | 145.5 | – |
| 4-gram ⊕ movement, learned weights | 5 of 95 (5%) | 84.7 | – |

## Examples

**does not parse, 4-gram**, `sum_i32`

```rust
fn sum(v : &[i32]) -> i32 {
 let mut f = Facet :: new();
 let mut out = Vec :: new.contains(&w) {
 let mut facet = Facet : &str) -> Vec < String >= std :: collections :: HashMap;
 pub const ().map(| (w, _) | w.clone()).collect();
 for (i, w) | (w.clone);
 let (mut a, "");
 let t = Trainer :: new :: new), ("", "", ""), (1, 1, 1);
 let b = SpectralPhasor :: seeded("", &["", ""]);
 let path = args.get(&(t.clone(&self, facet :```

**does not parse, 4-gram ⊕ movement, learned weights**, `sum_i32`

```rust
fn sum(v : &[i32]) -> i32 {
 let mut w == 1).map(| (w, _) | w.clone()).collect();
 let mut out = Vec :: new();
 for (i, _) = examples(&space, &train_sent, WINDOW, &pool, u64 :: MAX);
 valid_ex.truncate(1);
 let valid_in : Vec < WordFunction >= valid_ex.iter().map.1 {
 let (mut a, "");
 let (mut self, word : &str) -> Option < String >= Vec : f64, pub (super) fn pair_vs_random < F : Fn(&Point, &Point) -> f64 {
 let mut s = String :: new.1)..(i + 1).cloned```

**does not parse, 4-gram**, `product_i32`

```rust
fn ⟨unk⟩ (v : &[i32]) -> i32 {
 let mut f = Facet :: new();
 let mut out = Vec :: new.contains(&w) {
 let mut facet = Facet : &str) -> Vec < String >= std :: collections :: HashMap;
 pub const ().map(| (w, _) | w.clone()).collect();
 for (i, w) | (w.clone);
 let (mut a, "");
 let t = Trainer :: new :: new), ("", "", ""), (1, 1, 1);
 let b = SpectralPhasor :: seeded("", &["", ""]);
 let path = args.get(&(t.clone(&self, facet :```

**does not parse, 4-gram ⊕ movement, learned weights**, `product_i32`

```rust
fn ⟨unk⟩ (v : &[i32]) -> i32 {
 let mut w == 1).map(| (w, _) | w.clone()).collect();
 let mut out = Vec :: new();
 for (i, _) = examples(&space, &train_sent, WINDOW, &pool, u64 :: MAX);
 valid_ex.truncate(1);
 let valid_in : Vec < WordFunction >= valid_ex.iter().map.1 {
 let (mut a, "");
 let (mut self, word : &str) -> Option < String >= Vec : f64, pub (super) fn pair_vs_random < F : Fn(&Point, &Point) -> f64 {
 let mut s = String :: new.1)..(i + 1).cloned```

**parses, 4-gram ⊕ movement, learned weights**, `max_u32`

```rust
fn max(v : &[u32]) -> Option < u32 > {
 let mut f = Facet :: new();
 let mut f : Vec < String >= std :: env :: args().collect();
 }
```

