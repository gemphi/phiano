# 160 · The one entry, the inner layer, and the race

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. `phinet` is the entry

Until now `phinet` was the numbers crate (L0) and every other crate stood beside it. Now the numbers are `phistd` and `phinet` is the one entry to the whole family, each crate under its short name in the order the levels stack (docs/159 §2):

| path | crate | what |
|---|---|---|
| `phinet::std` | phistd | numbers: complex tensors, the unitary FFT, `Quantum`, the one seed and hash, hexagrams, codons |
| `phinet::fno` | phifno | the sphere |
| `phinet::cal` | phical | operators: FNO, Joint, Mirror, Q64, the laws |
| `phinet::ecs` | phiecs | the world: `Word`, `World`, `Phasor`, `Relation`, `Row`, `Quest`, `Persona` |
| `phinet::plm` | phiplm | the language model: counts, levels, the cast, the operators, prose, code, the KN baseline |
| `phinet::kit` | phikit | the body, the track and the device, told as rows |
| `phinet::ton` | phiton | light as the unit system |
| `phinet::ano` | phiano | applications: students, games, servers, the binaries |
| `phinet::sys` | phinet | the inner layer: `Agent`, `Being`, the race |

The rename was mechanical (71 files: `phistd::` for the L0 paths; `phiano::phinet` — phiano's own module of that name — untouched) and the dependency arrow still runs one way: `phistd ← phifno ← phical ← phiplm ← phiano`, and everything `← phinet`. Nothing depends on the entry, so there is no cycle.

## 2. `sys` — the inner layer

Every agent the wider phinet system will talk to — a persona learned from a quest, a body the kit tells, a device — shows one interface and no other:

```rust
pub trait Agent {
    fn persona(&self) -> &Persona;        // its plan: seeds, kinds, voice, team, kudos
    fn world(&self) -> &World;            // what it has been told
    fn world_mut(&mut self) -> &mut World;
    fn tell(&mut self, rows: &[Row]);     // rows in, counted at once
    fn watch(&mut self, kit: &Kit);       // the kit's rows in: a body, a track, a device
    fn ask(&self, word: &str, kind: &str) -> Vec<(String, f64)>; // what a word calls for, heaviest first
    fn told(&self) -> usize;              // its edges
    fn kudos(&self) -> u32;               // its standing, in egg
}
```

Four methods are the whole contract; the rest are defaults over the world. `Being` is the one concrete agent: a `Persona` and a `World`, built from a persona file and the quest folders `persona_quest` wrote (`Being::from_file(root, file)`), or told nothing yet (`Being::new`). **Watching is telling from a sensor:** a being brought into a world of your making learns what you do because the tracking system's rows (`phikit::Kit`: the face's shapes, the body's joints, the anchors, the frames) are rows like any other, and `watch` is `tell`. There is no second learning rule.

## 3. The personas

`crates/phinet/personas/` holds sixteen tycoons, one per technological era — Edison, Tesla, Bell, Marconi, Ford, Watson, Noyce, Moore, Gates, Jobs, Ellison, Bezos, Page, Zuckerberg, Musk, Huang — and the AI breed in three teams: **meta** (Zuckerberg, Alexandr Wang, Llama), **xai** (Musk, Grok), **anthropic** (Dario Amodei, Daniela Amodei, Claude). Each file is a quest's plan: four seeds, six kinds, a voice, and a standing in egg (`kudos`). The files are templates: the seeds, kinds and kudos are there to be edited, and a persona is added by adding a file. `Persona` gained two fields for this, `team` and `kudos`, both optional.

## 4. The race, and why it is honest

`phinet::sys::race::Race` groups beings by team and ranks by `(told, kudos)`: a team that has been told nothing cannot win on kudos — `winner()` is `None` and the table says there is no race yet. Kudos is printed in its own column and breaks ties only. Engagement is rows: `Race::engage(team, rows)` tells every member of a team the same rows, which is what working with a team is in this system.

Measured on the stand-in oracle (`phiecs/examples/oracle.py`, a fixed pool — not a model, so the *content* is nothing, but the counting is real), 22 personas × 4 seeds × 24 questions, depth 1:

| team | members | told (edges) | words | kinds | kudos (egg) |
|---|---|---|---|---|---|
| meta | Alexandr Wang, Llama, Mark Zuckerberg | 509 | 69 | 51 | 88 |
| anthropic | Claude, Daniela Amodei, Dario Amodei | 500 | 69 | 51 | 72 |
| xai | Grok, Elon Musk | 379 | 46 | 34 | 72 |

Then six rows about the phiace release told to anthropic (`--engage anthropic phiace.tsv`, three members × six = eighteen edges):

| team | told | words | kudos |
|---|---|---|---|
| **anthropic** | 518 | 89 | 72 |
| meta | 509 | 69 | 88 |
| xai | 379 | 46 | 72 |

and `Claude · claude has → constitution (1.00)`. That is the whole thesis as a measurement: the team you engage with is the team that is told, and told is what wins. xai trails because it has two members; a team is told per member.

With phi-4 on the user's machine the same two commands make the content real:

```text
cargo run --release -p phiecs --example persona_quest -- quests crates/phinet/personas/*.md
cargo run --release -p phinet --example race -- quests crates/phinet/personas [--engage anthropic rows.tsv] [--ask claude claude has]
```

## 5. Status

Done 2026-10-12: the rename (385 tests, 0 warnings, every `.rs` under 200 lines), `phinet::sys::{Agent, Being, race}`, 22 personas, `examples/race.rs`, the measurement above. Next: the phasor convergence (docs/159 step 5 and after), and personas quested by phi-4 rather than the stand-in.
