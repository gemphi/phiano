# Phiecs and the quest: an entity-component world for language, and learning by asking an oracle one word at a time

*Phient papers, 8 of 16 · 2026-04-20 · phiecs, the entity-component world whose entities are words, and the quest that fills it by asking an oracle*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Abstract

Phiecs is a Rust crate that gives language the shape of an entity-component-system. The shape is taken from Apple's RealityKit, as mirrored in the reality-foundation crate, with one substitution. The entity is a word. What a word carries is a phase on the relation circle: 64 channels, each with an angle, an amplitude and a count of evidence. An edge between two words carries a kind, and the kind is the condition under which one word calls for the other. The crate is small. It has 14 tests (docs/85) and no dependencies. Phiano reads it through a table.

The quest is the part of phiecs that learns. It starts at one word and asks an oracle what that word relates to. It parses the answer, writes each claimed relation into the world as an edge and a coupling, and puts the words it heard on a frontier. It walks outward, breadth first, until a depth or a question budget runs out. Every question becomes a page. Every kept relation becomes a row of `relations.tsv`. The oracle is a trait with three implementations: `Spawned`, which pipes the prompt into a command such as `ollama run phi4`; `Scripted`, which answers from a fixed table; and `Synthetic`, which answers from arithmetic on a hidden structure whose truth is known.

The one measurement made so far uses `Synthetic`. A quest over 256 invented words in families of eight, under five kinds, wrote 1,280 rows naming 293 target words. Phiano's `bin/quest_ingest` turned the rows into marker channels and was asked to rank the true answer on each kind's channel. The shipped trainer scored MRR 0.082 against a chance of 0.021. The cause was a deadband in phiano's byte phasor. With attraction only and dithered rounding, the same rows scored MRR 0.594, hit@1 34.8%, hit@5 97.4%. The wrong channel read 0.015, which is chance. A fifth of the rows held out came back at chance on one hop, and at 0.095 and 0.093 on two hops for is-a and used-for, the two kinds whose structure is shared across words.

No language model has been asked. There is no Phi-4 run. The condition template is written and has not been walked. The claim this paper can support is therefore narrow. The loop runs. The files it writes can be read back by phiano. A known structure survives the trip when the phasor is allowed to move in steps smaller than one quantum. Whether a model's answers carry structure that a channel can hold is not measured.

## 2. The claim

There are two claims, and they have different standing.

The first is a design claim. An entity-component-system is the right shape for meaning, provided the entity is a word and the thing an entity carries is a phase. RealityKit puts entities in a world, hangs components on them, and runs systems over both. Phiecs keeps all three. It renames them so the names say what they hold: `Word` for the entity, `Aspect` for the component, `Phasor` for the transform. It adds one thing Apple's model has no name for, which is `Relation`, the kind of an edge. RealityKit's hierarchy is `Parent` and `Children`: one edge kind and one parent per entity. A language is a multigraph. A word stands in many relations to many words at once, and the kind of each is its payload. In the paradigm's terms the kind is the condition. `w` means nothing except what it relates to, under which `c`. The lib.rs doc comment says the crate is "that sentence as a data structure." This claim is checked by construction and by tests. The mapping table in lib.rs names each RealityKit type and its phiecs counterpart, and the 14 tests exercise coining, forgetting, coupling, staging, the quest loop, the parser, the synthetic walk and the table round trip.

The second is an empirical claim. The quest module states its point in its own comment: to end with a smaller thing than the oracle. That smaller thing is 64 relation channels and a lexicon, which can be read, measured and argued with. The oracle is large and opaque. The quest asks it one word at a time, in a strict format, and keeps only what parses. If the claim holds, what the quest keeps is a set of concepts whose structure can be tested row by row.

Only half of the second claim has been tested. The synthetic oracle has a structure known in advance, so recovery can be scored exactly. That test was run and is reported in section 4. It shows that the path from quest to phiano facet can carry a structure, once two faults in phiano's coupling were found and set aside. It says nothing about what Phi-4 would answer. A model's answers may be inconsistent across words, may invent kinds past the 16 named ones, or may name phrases that the parser drops. None of that has been seen, because the run has not been made.

The paper therefore argues for the shape and for the loop, and reports the synthetic measurement as the only evidence. It makes no claim that a quest against a model yields a useful lexicon. That is the next experiment, and section 12 names its control.

## 3. The mechanism

The entity is `Word`, a `u32` issued by a `Lexicon`. A word cannot be minted outside the lexicon that interned it. The component is the `Aspect` trait, with `on_bind` and `on_unbind` hooks. Storage is `Dense<A>`, one slot per word id, so a lookup is an index. The world is `World`. It holds `lexicon`, `relations`, `edges`, `resources`, a store per aspect type, the roll of coined words, and a list of systems.

A relation kind is `Relation(u8)`, issued by a `Relations` table that also counts how many pairs were seen under each kind. `MAX_KINDS` is 64. `Relations::declare` returns `None` once all 64 channels are used. A fresh world declares the 16 `KIND_NAMES`: is-a, part-of, has, does, done-to, like, unlike, near, before, after, causes, used-for, made-of, belongs-to, more-than, less-than. An `Edge` is `from`, `kind`, `to`, `weight`. `Edges` indexes every edge both ways, so `out_of` and `into_of` are one lookup each. `calls_for(word, kind)` returns what a word calls for under a kind, heaviest first.

The `Phasor` aspect holds three vectors of length 64: `theta`, `amplitude` and `evidence`. `Phasor::seeded` hashes the spelling so a fresh word starts at an angle of its own. `couple(kind, target, rate)` moves one channel a step of `rate` toward `target` the short way round, adds one to evidence, and sets amplitude to `ln(1 + evidence)`. `agreement(other, kind)` is the cosine of the angle between two words on one channel, and `None` if either has no evidence there. `resonance` averages agreement over the channels both words have, weighted by the smaller amplitude.

`World::relate_by(from, kind, to, weight, rate)` is the whole of learning. It adds the edge, counts the kind, binds a seeded phasor to either word that lacks one, and couples each word toward the other's angle on that channel alone. `World::relate` calls it with weight 1.0 and `COUPLE_RATE`, which is 0.08. `coin` and `coin_bare` intern a spelling. `forget` drops the word's aspects and every edge that touched it. `Condition` is an aspect holding `now`, `was` and a step count, with `enter` and `tick`. `System`, `FnSystem` and `Stage` run closures in the order Ingest, PreCouple, Couple, PostCouple, Readout on each `World::step`.

The quest is `Quest`: a world, the interned `Conditions`, a frontier of `(spelling, depth)`, the list of words already asked, a depth, a budget and the `Step`s taken. `DEFAULT_DEPTH` is 3 and `DEFAULT_BUDGET` is 64. `ask_one` fills `template::RELATIONS`, puts it to an `Oracle`, runs `parse_relations`, relates each `Claim` with rate `COUPLE_RATE * weight`, and pushes the new words at depth plus one. `run` repeats until the budget or frontier is spent. `write` produces the pages and the table. `parse_tsv` and `Quest::from_tsv` read the table back.

## 4. What was measured

Everything measured about the quest was measured in phiano, on a table phiecs wrote. The run is recorded in docs/83. The command was the example with the synthetic oracle, seed word `w0`, depth 12, budget 600, 256 words. It walked the whole hidden structure: 1,280 rows, 293 target words. `bin/quest_ingest` turned each row into the document `about kind other`, repeated by its weight. Each kind became a marker channel, and each pair was coupled on it. The reading ranks the true `other` among all 293 targets on the kind's channel. Chance MRR over that pool is 0.021.

Four arms were run for sixty-four epochs each, on 3,584 documents, seed 7 (docs/83):

| negatives | rounding | kind MRR | hit@1 | hit@5 | within kind | wrong channel | offset |
|---|---|---|---|---|---|---|---|
| on (shipped) | nearest (shipped) | 0.082 | 0.5% | 6.2% | 0.303 | 0.013 | 0.030 |
| on | dither | 0.177 | 3.8% | 29.0% | 0.404 | 0.014 | 0.018 |
| off | nearest | 0.092 | 1.0% | 7.0% | 0.330 | 0.014 | 0.013 |
| off | dither | 0.594 | 34.8% | 97.4% | 0.747 | 0.015 | 0.005 |

The wrong channel reads at chance in every arm. The kind channel carries the kind. The offset reading, which is what phiano's dialogue uses today, sees almost none of it.

Within kind, ranking only against words the oracle ever named under that kind, is-a reached 75.0% hit@1 over 32 family heads and used-for reached 100% over 5 tools (docs/83).

With a fifth of the rows held out, 245 rows, one hop scored MRR 0.011, below the chance of 0.021 because the truth ties with every word that has no evidence. A generic second hop, reading what a word's relatives relate to, scored 0.095 for is-a and 0.093 for used-for. Part-of, near and unlike stayed near chance. Over all kinds, two hops scored 0.045 with hit@5 4.1% (docs/83).

Dither was then tried on a corpus. On the three-writer persona attribution of docs/81 it cost six points, 71% to 65%, so it ships off by default and is switched on for quests.

Docs/86 scored the same quest with Liskov's three operators and filed each word by hexagram. The 256 words occupied 61 of 64 hexagrams. Mean better was −0.008, mean worse −0.008, mean loss 0.108 against a margin of 0.1. The table is flat. The truth ties the best wrong word on most rows, which is the byte ceiling seen from a third side.

Nothing was measured with Phi-4. Phiecs's own f64 phasor was not measured on recovery; its tests check that coupling converges.

## 5. The one idea

A word is the edges it stands in and the angles those edges have left on it. That sentence is the whole of phiecs, and every type in the crate is one clause of it.

"The edges it stands in" is `Edges`, and each edge has a kind. The kind is what makes this a language. A tree has one kind of edge. A word can be `part-of` one word, `near` another, `unlike` a third, and all three edges are separate facts. RealityKit cannot say this. It has one kind of edge and one parent. Phiecs has up to 64 kinds, and each is a channel.

"The angles those edges have left on it" is `Phasor`. Each time two words are related under a kind, both move toward each other on that kind's channel, and the channel counts one more observation. Nothing else moves. After many relations, a word's phasor is a record of every condition it was seen under and how often. The angle is never a position. It only means something against another word's angle on the same channel. That is what `agreement` computes, and it returns `None` when either word has no evidence, because silence is not agreement.

The quest is the same idea turned into a procedure. If a word is its relations, then the way to learn a word is to ask what it relates to. The question has a fixed form: this word, which kinds, which other words, how strongly. The answer is parsed into relations and nothing else. Each relation moves two words. The words named in the answer are the next words to ask about. The quest stops when it has asked enough, and what it leaves behind is a world whose every edge can be traced to one line of one answer.

That traceability is the reason for the design. A trained language model holds what it knows in weights no one can read row by row. A quest holds what it learned in a table with four columns: `about`, `kind`, `other`, `weight`. The table can be read by a person, grepped, diffed against a second run, re-ingested into a fresh world with `Quest::from_tsv`, or handed to phiano. If the oracle said something wrong, the wrong row is there to be found.

The measurement in section 4 tests the last step of this: whether phiano, given the table, can put the answer back on the right channel. It can, at 0.594 MRR and 97.4% hit@5, once its byte phasor is allowed to take steps below one quantum. The one idea is the same at each stage. Ask for relations, keep only relations, move pairs, count.

## 6. The world, clause by clause

The lib.rs module comment lays out the map from RealityKit to phiecs as a table. It is worth walking row by row, because each row records a decision, and several record a fault in the Apple mirror that phiecs does not copy.

| RealityKit / reality-foundation | phiecs |
|---|---|
| `Entity` | `Word` |
| `EntityGenerator` | `Lexicon` |
| `Component` | `Aspect` |
| `Storage<T>` (linear scan) | `Dense<A>` (id-indexed) |
| `spawn` / `despawn` | `coin` / `forget` |
| `Transform` | `Phasor` |
| `Parent` / `Children` | `Edge` / `Edges` |
| `add_child` / `remove_child` | `relate` / `unrelate` |
| `GKStateMachine` | `Condition` |
| *(none)* | `Relation` |

Apple's `Entity` carries a generation so a recycled id can be told from a live one. Phiecs never recycles. The word module says the generation field is absent on purpose, "do not add it back." Apple's `Component` is a blanket impl over every `Send + Sync + 'static` type, which by coherence makes its `on_attach` and `on_detach` hooks impossible to override (reality-foundation `component.rs:21`). `Aspect` is implemented explicitly, so its hooks run. Apple's storage is a linear scan. `Dense<A>` indexes by word id. Apple's `despawn` leaves components behind for the next id, as its own comment at `world.rs:174` says. `World::forget` calls `forget` on every store and drops every edge that touched the word. The spelling stays interned, so an id already handed out still reads back.

The README's usage example is the smallest complete world:

```rust
let mut world = World::new();
let cat = world.coin("cat").bind(Phasor::seeded("cat")).word();
let mat = world.coin_bare("mat");
let on = world.declare("on").unwrap();
world.relate(cat, on, mat);
assert_eq!(world.calls_for(cat, on).first().map(|(w, _)| *w), Some(mat));
```

`World::new` has already declared the 16 named kinds, which occupy channels 0 to 15 in the order of `KIND_NAMES`. So `on` is new and gets channel 16. `declare` also interns `on` into the lexicon, so a kind is itself a word and can be related to. `mat` has no phasor when `relate` is called, so `relate_by` binds `Phasor::seeded("mat")`. Both words then move 0.08 of the way toward each other on channel 16, and each records one observation there. `calls_for(cat, on)` returns `mat` with weight 1.0.

Conditions follow the same rule. `CONDITION_NAMES` are asking, answered, grounded, contradicted and idle, and `Conditions::intern` makes each an ordinary word. Entering a condition is a comparison of two ids. The GameplayKit mirror found states by a linear scan over strings (`gameplay_kit/mod.rs:31`).

## 7. The phasor, and how two words move

`Phasor::couple` is eight lines. It computes the signed shortest angle `d` from the channel's angle to the target, in the range −π to π. It adds `rate * d` to the angle and wraps the result into 0 to 2π. It adds one to the channel's evidence, saturating. It sets the channel's amplitude to `ln(1 + evidence)`. The step is linear in `d`, so a pair keeps closing at a fixed fraction of its remaining gap.

`relate_by` reads both words' angles on the channel first, then couples each toward the other's old angle. The two move toward each other symmetrically. The phasor test in `phasor.rs` does the same thing by hand: `cat` and `mat`, seeded, coupled forty times on one channel at rate 0.2. After that, agreement is above 0.99, evidence on that channel is 40, resonance is above 0.99, and `kinds()` returns that one channel with a count of 40. Before the first coupling, agreement is `None`.

In a quest the rate is `COUPLE_RATE * weight`. A claim at weight 0.9 moves both words 0.08 times 0.9, which is 0.072 of their gap. A claim at 0.5 moves them 0.04. The edge weight accumulates separately: `Edges::relate` adds to an existing edge of the same `from`, `kind` and `to`. So a word the oracle names twice under the same kind moves twice and weighs twice. The quest module says so: an answer is evidence, counted.

The phiecs phasor holds angles as `f64`. Phiano's phasor holds each channel as one byte, 256 quanta of 2π. This difference is where the first measurement failed. Phiano's nudge is `0.06 · sin d`. Once `sin d` falls below 0.2, about 11°, the nudge is under half a quantum, nearest rounding writes back the byte it read, and the pair stops. Docs/83 found every one of the 256 part-of pairs stopped at the same `|sin d| = 0.195`, eight quanta short. The fix was dithered rounding, which rounds up with probability equal to the fractional quantum, so small steps add up in expectation. The coin is a hash of the phasor's bytes and the channel's evidence count, so runs stay reproducible.

Phiecs has no such deadband. Its angles are continuous, and its step does not shrink with `sin d`. That makes the phiecs world a faithful record of what the oracle said, and phiano's facet the compressed one. The compression is where the ceiling sits. Two hundred and fifty-six part-of targets on a 256-quantum circle collide, and a collision is a tie that the measurement scores against the truth (docs/83).

## 8. The quest loop, step by step

`Quest::new(seed)` builds a world, interns the five conditions, coins the seed, and puts `(seed, 0)` on the frontier. `with_depth` and `with_budget` set the two limits. `next_word` pops the frontier until it finds a word within depth that has not been asked, marks it asked, and returns it. `push` puts a word on the frontier only if its depth is within the limit and it has not been asked. The asked list is what stops a cycle in the language from becoming a loop in the program.

`ask_one` takes the next word and the list of every kind the world currently knows. It fills the relations template with both. It asks the oracle. On `Ok(text)` it parses the text into claims and records `Answer::Said(text)`. On an error it records `Answer::Nothing` with the error's message and keeps no claims. Then, for each claim, it declares the kind. A kind the oracle invented is declared too, up to the 64-channel bound. Past the bound, `declare` returns `None` and the claim is dropped. It is never put in some other bucket. It coins the other word, relates the pair with the claim's weight and rate, and keeps the triple. `step::opened` lists the kept words not yet asked, each once, and they go on the frontier at depth plus one.

A failed answer is a step. The word counts as asked and the quest goes on.

The test `a_quest_walks_outward_and_stops_at_its_budget` shows the walk on a scripted oracle. The quest starts at `cat` with depth 1 and budget 10.

| # | asked | depth | kept | opened |
|---|---|---|---|---|
| 1 | cat | 0 | is-a animal 0.9, has fur 0.8, does sleep 0.6 | animal, fur, sleep |
| 2 | animal | 1 | is-a thing 0.7 | none: thing would be depth 2 |
| 3 | fur | 1 | part-of cat 0.9 | none: cat was asked |
| 4 | sleep | 1 | does rest 0.5 | none: rest would be depth 2 |

The frontier is then empty and `run` returns 4, which the test asserts. The prose line in the `animal` answer, "not a relation line at all", has no pipe and is skipped. The test then reads the graph. `calls_for(cat, is-a)` returns `animal` first, and the phasors of `cat` and `animal` both have evidence on the is-a channel, so `agreement` returns a value.

A second test runs a quest against a scripted oracle that knows nothing. It asks once, records `Nothing`, and keeps nothing.

The quest never calls `World::step`. It adds no systems. Learning happens inside `relate_by`, at the moment each claim is written.

## 9. The template, the parser, and the oracle as a command

The template module opens with the sentence that explains the quest's choice of format: the format is the whole trick. A model asked for prose returns prose, and prose has to be parsed by another model. A model asked for one relation per line, pipe-separated, returns that, and it parses with `split('|')`. The template is a `Template` struct with a `name` and a `body`, so it can be edited without touching the loop, and each step records which template produced its prompt.

The `RELATIONS` body, filled for `cat`, reads in part:

```text
You are describing one word for a relation engine, not for a reader.
A word means nothing except what it relates to, under which condition.

Word: cat

List at most 8 relations this word stands in. One per line, exactly:

kind | other word | weight
```

It then asks for a kind from the listed kinds, a single lower-case common word, and a weight from 0.1 to 1.0, and forbids prose, numbering and blank lines. `Template::fill` replaces `{word}` and `{kinds}`, joining the kinds with commas.

`parse_relations` is built on the expectation that the model will not comply. It trims each line and strips a leading `-`, `*` or `•`. It splits on `|`. A line with fewer than two fields is skipped. `clean` lower-cases each field and keeps letters, digits, dashes and spaces. An empty kind, an empty other, or an other containing a space is skipped. A missing or unparsable weight becomes 1.0. The weight is clamped to 0 to 1, and a weight of 0 is skipped. The test `the_parser_survives_a_chatty_model` feeds it an answer that opens with "Sure! Here are the relations for `cat`:", wraps the lines in a code fence, adds a bullet, a prose line, a phrase, and a missing weight, and ends with "Hope this helps!". Four claims come out: is-a animal 0.9, has fur 0.8, does sleep 0.6, and near dog at 1.0.

The `CONDITIONS` template asks for at most 4 conditions under which a word relates to different things, one per line as `condition | other word | weight`. It is written. No code path fills it.

`Spawned` is how a model is reached. The prompt goes in on stdin and the answer comes back on stdout. The oracle module names three ways to reach Phi-4: `llama-cli -m phi-4.gguf -f /dev/stdin`, `ollama run phi4`, or a script of one's own. Errors are `Unreachable`, `Failed` or `Empty`.

The example binary takes seed, depth and budget, then either `synthetic` or a command. With neither, it runs a scripted oracle whose one key, `Word:`, matches every prompt. That dry run proves the loop and the files without a model.

## 10. The synthetic oracle, and what a quest writes

`Synthetic::new(n)` rounds `n` up to a multiple of eight. Word `i` is spelled `w{i}`. `truth(i)` gives each word five relations by arithmetic on its index:

| kind | other | weight |
|---|---|---|
| is-a | `f{i / 8}`, the family head | 0.9 |
| part-of | `w{(7i + 3) mod n}` | 0.6 |
| near | the next word in the family, wrapping | 0.7 |
| used-for | `t{i mod 5}`, one of five tools | 0.5 |
| unlike | `w{(i + n/2) mod n}` | 0.8 |

For `n` of 256 there are 32 families. Seven is coprime to 256, so part-of reaches every word, as do near and unlike. The targets are therefore the 256 words, 32 family heads and 5 tools, which is 293. Five rows per word gives 1,280. Both match docs/83. The oracle reads the subject from the prompt's `Word:` line. For a family head, a tool, or an index past `n`, it returns `Empty`. Those words are leaves. They are asked and recorded as steps with nothing kept.

Asked about `w0`, the oracle answers:

```text
is-a | f0 | 0.9
part-of | w3 | 0.6
near | w1 | 0.7
used-for | t0 | 0.5
unlike | w128 | 0.8
```

The read-backs in docs/83 check against this arithmetic. `w48 near → w49`: family 6, next word. `w232 unlike → w104`: 232 plus 128, mod 256. `w176 part-of → w211`: 7 times 176 plus 3 is 1,235, which is 211 mod 256. `w76 is-a → f9`: 76 divided by 8.

The command that produced the measured table is:

```bash
cargo run --release --example quest -- w0 12 600 synthetic 256
```

Output goes to the directory named by `PHIQUEST_OUT`, default `quest`. `Quest::write` makes three things. `steps/001-w0.md` is the first page. Its header line reads: depth 0, template `relations`, 5 relations kept, 5 words opened. It is followed by an Asked block, an Answered block holding the oracle's text, and a Kept table. `relations.tsv` holds one tab-separated row per kept claim, weight to three places, so the first row is `w0`, `is-a`, `f0`, `0.900`. `README.md` is the index, one row per step.

`parse_tsv` reads the table. It skips blank lines and any row whose weight does not parse, which covers a header, and nothing else. `Quest::from_rows` rebuilds a world by relating every row exactly as the oracle's claims were related, marks each `about` as asked, and clears the frontier. The test `round_trips_a_synthetic_quest` runs a 16-word quest, writes its rows, parses them, rebuilds, and checks that edges and lexicon size match. The rebuilt quest counts only words with edges as asked, because leaves left no rows.

## 11. Dependencies in the PML family

Phiecs depends on nothing. Its Cargo.toml has no dependencies. It declares one feature, `phiano-facet`, which is empty and off by default so the crate builds standalone. Docs/85 lists it as `phiace/crates/phiecs/`, "the entity-component world for language and the quest", 14 tests, and records its only link as phiano, via the TSV. The container copy read for this paper sits at `/home/claude/phiecs`.

Phinet is the parent engine of the family: tensor, tape, unitary FFT, `Fno`, joint training, HRR, the 64 hexagram gates and the sphere. Phiecs does not link it. Phinet reaches the quest only after phiano has ingested it. Docs/86 files each quest word under a hexagram with `phinet::hexagram::hexagram_of` and scores it with `phinet::liskov`'s better, worse and loss. The children of phinet, which are phifno 1.0.0, phical, phiton, phinum and the planned phiqlm, have no path to phiecs. Phiqlm has nothing built.

Phical has two meanings in the tree (docs/85). The crate `phi-cal` in `phiace/crates/phical` is geometry on the relation circle. It sits beside phiecs in the same crates folder and does not depend on it or on anything else. The paradigm phical is the sentence that opens this paper, and phiecs is that sentence as a data structure, in the words of lib.rs.

Phiano is the word-level application and the only consumer. `bin/quest_ingest` reads a phiecs `relations.tsv`, turns each row into the three-token document `about kind other`, lets `RelationVocab::learn` find the kinds as marker channels, and couples each pair with `Trainer::couple_relations`. It saves a `.chroma` beside the table, and `bin/quest_ask` puts the quest's own questions back to it through `Facet::under` and `calls_for_under` (docs/83). The phiano helix writes the same facet as strands of 64 codons, with PhiTon wavelength, band and hex per codon (docs/84). That is the one route by which PhiTon touches a quest.

The two crates number channels differently. In phiecs, is-a is channel 0 because it is first in `KIND_NAMES`. In phiano, is-a became channel 8, because phiano's marker learning assigns channels by how often a word stands between two others (docs/83). The TSV carries kind names, so the numbering does not need to agree.

Phiace is the UI workspace that holds the crate. Its `apple` mirrors, from reality-foundation and GameplayKit, are the source of the shape, and lib.rs and the module comments cite them by file and line. Phiber's ECS Wand workspace is meant to edit the phiecs graph as nodes. `System::name` is documented as feeding "the ECS Wand's node list". The README leaves this item unchecked. It is not built.

## 12. The next experiment

The next experiment is the first Phi-4 quest, measured by the same path as the synthetic one, against a control built from its own rows.

The run is the README's command, `cargo run --release --example quest -- cat 3 64 ollama run phi4`: seed `cat`, depth 3, budget 64, with `PHIQUEST_OUT` set to a fresh directory. It writes a page per question and a `relations.tsv`. The table then goes through `bin/quest_ingest` with the arm that worked on the synthetic quest, 64 epochs, negatives off and dither on. Phi-4 gives no truth to rank against, so the reading must use held-out rows. With `PHIANO_QUEST_HOLDOUT=0.2`, a fifth of Phi-4's own rows are withheld, and the question is whether the facet ranks them above chance on their kind's channel, by one hop and by two.

The control is the same table with its kind column shuffled among rows. That keeps every word, every weight and every kind count, and breaks the link between a pair and its kind. If the kind-channel reading on Phi-4's rows is no better than on the shuffled rows, the model's kinds carry nothing a channel can hold. The shuffle does not exist yet as a flag and would have to be written. The synthetic quest at matched size is the second reference. It shows what a fully consistent oracle scores on the same pipeline, so the Phi-4 number has a ceiling to be read against. The wrong-channel reading is the third check. It read at chance on every synthetic arm and should do so here.

Before any score, the transcript itself is data, and four counts should be reported. The first is how many of the 64 questions returned `Said` and how many `Nothing`. The second is how many lines per answer survived `parse_relations`. The third is how many kinds were declared past the 16 named ones, and whether the 64-channel bound was reached. The fourth is how many claims were dropped because the other word was a phrase. These say whether the strict format holds for this model. The parser test shows the format surviving a chatty answer, but that answer was written by hand.

A second run with a different seed word tests whether two quests agree where they overlap. Each is read back with `Quest::from_tsv`, and edges present in both are compared by kind and weight.

The Liskov scoring of docs/86 and the hexagram table apply unchanged. On the synthetic quest the table was flat, with mean loss 0.108 at a margin of 0.1. Docs/86 says the table exists for a corpus that is not flat, which is Phi-4's. Whether it is flat there is not measured.

None of this has been run.

## 13. What is wrong or missing

There is no Phi-4 run. Every number in this paper comes from an oracle that answers from arithmetic. The crate was built to ask a model, and it has not asked one.

The condition half of the paradigm is not in the quest. `CONDITIONS` is written and never filled. `Quest::new` interns the five starting conditions, and nothing enters them. No quest binds a `Condition` aspect to any word. The relation kind stands in for the condition throughout. The README lists condition-conditioned readings as unchecked.

The staged systems are unused by the quest. `World::step` and the five stages are tested on their own, and the quest never calls them.

The transcript is not fully verbatim. `ask_one` fills the prompt with the kinds known at the moment of asking. `write` fills it again, per step, with the kinds known when the quest ends. If an oracle invents a kind at step 5, the Asked block of steps 1 to 4 lists a kind those prompts never contained. The Answered block is verbatim. The Asked block is a reconstruction. The synthetic oracle never invents a kind. A model might. There is also a leftover `let _ = prompt;` in `ask_one`. `Step::template` is always `relations`.

The README index reports `lexicon.len()` as its word count. The lexicon also holds the five condition words and every declared kind name, so that count is larger than the number of words the oracle named.

`Scripted` matches a key the prompt contains, so `Word: cat` also matches `category`. A replay of a real transcript would need exact keys.

The parser keeps spaces inside a kind. An answer that writes `is a` instead of `is-a` declares a new kind on a new channel. Whether Phi-4 does this is not measured.

The doc comment on `Synthetic::truth` calls the last relation `opposite`. The code and `SYNTHETIC_KINDS` call it `unlike`.

The `phiano-facet` feature is declared in Cargo.toml with a comment that the bridge lives in `corpus::facet`. The `corpus` folder is empty and lib.rs declares no such module. Phiecs cannot read a `.chroma` lexicon. The README lists this as unchecked.

Evidence in both phasors is counted per channel, with no record of side. A word seen only as a source on a channel has evidence there and a seeded target angle, and nothing says it was never a target. Docs/83 shows `w0 is-a → w29 (1.00)` and `f0 is-a → w150` as this gap read from both ends. A per-side sub-quantum residue is recorded in docs/83 and not built.

On the phiano side, hit@1 is capped by collisions on the byte circle, dither is a switch that costs a corpus six points, and nothing generates from `Facet::under`.

## 14. How it was found

The shape came first. The module comments in phiecs cite the Apple mirror by file and line throughout: `components.rs:12` for `Transform`, `components.rs:113` for `Parent`, `world.rs:35` for the world's four responsibilities, `world.rs:174` for the `despawn` that leaves components behind, `world.rs:207` for `add_child`, `world.rs:13` for `EntityBuilder`, `component.rs:21` for the blanket impl, `world.rs:67` and `system.rs:93` for the step loop written twice, and GameplayKit's `gameplay_kit/mod.rs:31` for the string-keyed state machine. Each citation records a place where phiecs took the shape and left the fault. The stage names followed from the same reading. Apple's Input, PreUpdate, Update, PostUpdate and Render assume a 16 ms frame. A language engine advances per observation, so the stages became Ingest, PreCouple, Couple, PostCouple and Readout.

The quest came with the world. The oracle was left as a trait so a model could be reached by a command without linking a runtime. `Scripted` came first so the loop could be tested before a model was installed. `Synthetic` came after, for a reason its doc comment states: a recoverability measurement needs a known graph, and prose has no known graph.

Docs/83 is where the quest met phiano. The first run of `bin/quest_ingest` on the synthetic table scored MRR 0.057 at eight epochs and 0.082 at sixty-four, with every reading at 1.00 for the wrong word. A probe printed the same `|sin d| = 0.195` for all 256 part-of pairs, which located the byte deadband. The same note found that negative sampling was the other half of the ceiling on a graph with one target per source per channel. It ran the four arms, the per-kind table, the holdout and the two-hop reading. It then re-ran the persona attribution of docs/81 with dither and found the six-point cost, which set dither as a trainer choice. The same note added `Quest::from_tsv` on the phiecs side, `Facet::under` and `bin/quest_ask` on the phiano side, and the `.chroma` saved beside the table.

Docs/84 wrote the synthetic quest's facet as a helix and read `w0`'s five relations back as five codons. It listed the helix as the quest's own output format as something that remains.

Docs/85 took the inventory of the phi* crates and placed phiecs in the PML structure: in phiace/crates, 14 tests, depending on nothing, reached by phiano through the TSV.

Docs/86 scored the quest with Liskov's three operators and filed it by hexagram. The flat result, 61 of 64 hexagrams occupied, mean loss 0.108 at margin 0.1, was read as the byte ceiling again. The same note argued that `worse` should bite on a quest, where a word either has onward relations or has none. On text it did not.

## 15. What remains

The Phi-4 quest remains, with the shuffled-kind control of section 12. It is the only experiment that tests the second claim, and until it runs the quest has been measured on arithmetic alone. The command exists. The shuffle does not.

The conditions remain. The `CONDITIONS` template has to be walked. A condition would then become a word the quest enters, with a `Condition` aspect bound and readings taken under it. Today the kind carries the whole of the "under what conditions" in the paradigm, and a word cannot relate to different things in different settings under one kind. This is the part of the opening sentence the crate does not yet hold.

The transcript fix remains. Each step should store the prompt it actually sent, so the Asked block is verbatim when an oracle invents kinds. This matters before a model run and not after.

The direct bridge remains. The `phiano-facet` feature and `corpus::facet` are named and empty. With them, phiecs could read a `.chroma` lexicon directly, and a quest could start from what phiano already knows. The TSV is the bridge today.

The phasor change remains, on the phiano side. Per-side evidence, or a sub-quantum residue of two bits per channel, would let one rounding rule serve both a quest and a corpus. Docs/83 argues for it and does not make it.

Generation from marker channels remains. `Facet::under` reads a quest's relations by kind name. The generator and the dialogue read offset channels, and on the synthetic quest the offset reading was near zero. Until a generator reads marker channels, nothing a quest teaches reaches a sentence.

The ECS Wand remains: the phiber workspace that would edit the world as nodes. Only its hook exists, in `System::name`.

The helix as the quest's own output format remains, from docs/84. So do forcing at depth two on the quest, and the hexagram table on a Phi-4 quest, from docs/86.

The systems remain unused. A quest could be written as stages: ask at Ingest, relate at Couple, write pages at Readout. Then the ECS shape would carry the quest as well as the data.

Resonance remains unmeasured. It is the phiecs reading of whole-word similarity, and it could be scored on the synthetic quest, where family members should resonate on is-a and near. That would be the first measurement of phiecs's own f64 phasor.

In order: the Phi-4 run with its control, the verbatim prompt, the conditions walk, and then the rest.

## 16. Sources

Every path below was read with the Read tool or printed in full for this paper. No other file was used. There is no ARCHITECTURE.md in the container copy, and none is cited.

| path | what was taken from it |
|---|---|
| `/home/claude/phiano/docs/papers/phient/BRIEF.md` | the rules and the sixteen-section arc |
| `/home/claude/phiano/docs/papers/phient/PHIENT_FACTS.md` | the paradigm paragraph; the family; the quest, deadband, dither and Liskov numbers |
| `/home/claude/phiecs/README.md` | key types, usage, the three example commands, the bridge summary, the status list |
| `/home/claude/phiecs/Cargo.toml` | no dependencies; the empty `phiano-facet` feature and its comment |
| `/home/claude/phiecs/src/lib.rs` | the RealityKit map; the prelude |
| `/home/claude/phiecs/src/quest/mod.rs` | `Quest`, `DEFAULT_DEPTH` 3, `DEFAULT_BUDGET` 64, `next_word`, `push`; the smaller-thing statement |
| `/home/claude/phiecs/src/quest/template.rs` | `Template`, `RELATIONS`, `CONDITIONS`, `fill`, `parse_relations`, `Claim`, `clean`, the chatty-model test |
| `/home/claude/phiecs/src/quest/synthetic.rs` | `Synthetic`, `truth`, `SYNTHETIC_KINDS`, leaves, the hidden-structure test |
| `/home/claude/phiecs/src/quest/run.rs` | `ask_one`, `run`, `write`, `slug`; the scripted walk and silent-oracle tests |
| `/home/claude/phiecs/src/quest/oracle.rs` | `Oracle`, `OracleError`, `Spawned`, `Spawned::ollama`, `Scripted` |
| `/home/claude/phiecs/src/quest/step.rs` | `Step`, `Answer`, `to_markdown`, `to_tsv`, `opened` |
| `/home/claude/phiecs/src/quest/tsv.rs` | `Row`, `parse_tsv`, `from_rows`, `from_tsv`, the round-trip test |
| `/home/claude/phiecs/src/phasor.rs` | `Phasor`, `seeded`, `couple`, `agreement`, `resonance`, `kinds`; the coupling test |
| `/home/claude/phiecs/src/relation/mod.rs`, `edge.rs`, `set.rs` | `Relation`, `Relations`, `MAX_KINDS` 64, `Edge`, `Edges`, `calls_for`, `KIND_NAMES` |
| `/home/claude/phiecs/src/world/mod.rs`, `relate.rs`, `spawn.rs`, `schedule.rs` | `World`, `COUPLE_RATE` 0.08, `relate_by`, `declare`, `coin`, `forget`, `step` |
| `/home/claude/phiecs/src/condition/mod.rs`, `set.rs` | `Condition`, `Conditions`, `CONDITION_NAMES` |
| `/home/claude/phiecs/src/system/mod.rs`, `set.rs` | `System`, `FnSystem`, `Stage` and its order |
| `/home/claude/phiecs/src/aspect.rs`, `store/dense.rs`, `word/mod.rs`, `query/pair.rs` | `Aspect`, `Dense`, `Word`, `Related` |
| `/home/claude/phiecs/examples/quest.rs` | argument handling, oracle choice, `PHIQUEST_OUT`, the dry-run oracle |
| `/home/claude/phiano/docs/83_quest_ingest.md` | the bridge, the deadband probe, the four arms, per-kind and holdout tables, the dither cost |
| `/home/claude/phiano/docs/84_sphere_and_helix.md` | `w0`'s five codons; the helix as quest output, proposed |
| `/home/claude/phiano/docs/85_PML_STRUCTURE.md` | phiecs's place in the tree, 14 tests, depends on nothing |
| `/home/claude/phiano/docs/86_liskov_three_operators.md` | the quest scored by better, worse and loss, filed by hexagram |

Where PHIENT_FACTS.md and docs/83 give the same number, they agree: 0.082, 0.594, 34.8%, 97.4%, 0.747, 0.015, 0.005, 0.011, 0.095, 0.093, and 71% to 65%. The README rounds 97.4% to 97%. Numbers given here only in docs/83 are attributed to it where they appear. The count of 14 tests in docs/85 matches the `#[test]` attributes in the container copy of `src`.
