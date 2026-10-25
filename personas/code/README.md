# The code ring — languages as personas

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. A language's persona is told from its own source code (`sources`, read by `phiplm::code`), not quested from an oracle: its world is what its programs do. BCPL first; Rust when its reader moves here.

```text
cargo run --release -p phiano --example circus -- quests crates\phiano\personas bcpl wittgenstein --turns 8
cargo run --release -p phiano --example think -- quests crates\phiano\personas bcpl bertrand-russell
```
