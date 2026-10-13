# Where Phient stands in September 2026: what is built, what is measured, what is not, and the next three experiments

*Phient papers, 16 of 16 · 2026-09-14 · Where Phient stands in September 2026: what is built, what is measured, what is not, and the next three experiments*

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Abstract

This is the last of sixteen papers. It takes stock. It says what is built, what has been measured and to what number, what has not been measured, and which three experiments come next. Its sources are the project's own notes, docs/75 to docs/86, the operator sweep table in `data/sweep/results.md`, the proposal to Anima Anandkumar in `docs/papers/18`, and the facts sheet this series cites.

What is built is an engine and an application. phinet is the engine: a complex tensor, a tape for gradients, a unitary FFT, a Fourier neural operator, sixty-four hexagram gates and a spherical tie. It agrees with a PyTorch reference on 62 of 62 parity tests. phiano is the application: a lexicon of two-sided phasors, a trainer, a generator, personas, a bridge that reads quests, and a helix file format. The phiano library's 226 tests pass. They passed in the build container first, and they have now also been compiled and passed on the user's Windows machine.

What is measured is small and specific. On three Austen novels the best next-word operator ranks the true word at 30.7× chance on validation and 30.3× on test, with hit@10 0.348. An anisotropic spherical tie with 4,776 parameters reads 16.0× and 15.6×, above the circle's 14.7× and 14.6× at 8,360 parameters. Persona attribution on held-out paragraphs reads 71% where chance is 33%. A synthetic quest read back through a facet reaches MRR 0.594 and hit@5 97.4% once the byte deadband is dithered.

What is not measured is larger. No Phi-4 run has been made. The quest has only met a synthetic oracle whose answers come from arithmetic. The sphere has not been run at the softmax, wide and deep setting where the circle reaches 30.7×. The latency benchmark for learn, recall, correct and unlearn cannot run, because its inputs are absent from `data/`. The server's chat path is a template. There is no weather work of any kind, and no partnership has been agreed with anyone.

The three experiments are named in section 10 and given controls in section 12. The first points the quest at Phi-4, with the synthetic oracle as its control. The second runs the anisotropic sphere at the shipped operator's width and objective, against the circle at equal parameters. The third trains one operator on the shared persona lexicon and decodes the three-persona dialogue from it, against the trigram-decoded dialogue it would replace. Each of the three uses a harness that already exists. None of the three needs a new idea to run.

## 2. The claim

The claim Phient makes is narrow, and it has three parts that can each be checked.

The first part is about storage. A word can be kept as nothing but its relations, and that is enough to rank what comes next. In phiano a word is one `SpectralPhasor`: 64 channels, each with a source angle, a target angle and a 32-bit evidence count. No point in a space is stored for it. On three Austen novels the lexicon alone ranks the true next word at 7.6× chance, with hit@10 0.117 (docs/75). On WikiText-2 the directed ranking reads 15.5× chance. These are small numbers, above chance on text the lexicon did not train on.

The second part is about the operator. A Fourier neural operator, the construction Anima Anandkumar's group introduced for fields governed by equations, can take a word's 64-channel function as its input space and learn to move it toward the next word's function. On the Austen pool of 1,228 words, with chance MRR 0.0063, the identity operator ranks at 7.9× chance. The shipped hinge operator reads 14.7× on validation and 14.6× on test. The softmax operator at width 16 and three layers, trained for eight epochs, reads 30.7× and 30.3×. The operator is doing work the lexicon cannot do alone, and the amount is measured.

The third part is about learning from an oracle. Relations asked of an oracle one word at a time, and written as a table of `about`, `kind`, `other` and `weight`, can be put on a named channel of the facet and read back. On a synthetic oracle with a known answer this reads MRR 0.594 against chance 0.021. The wrong channel reads 0.015. The kind channel carries the kind.

The paper's own claim is about its record. Every number in it comes from a file in the tree, and each can be re-run with a command given in its source. Where two sources give different numbers for the same thing, this paper says so and uses the later one. Where something was proposed and not built, it says "not built". Where something was built and never run, it says "not measured".

The claim does not include fluent generation. The generator's best text is still trigram-stitched, and its best perplexity comes from the sampler. It does not include any result from a large model. Phi-4's weights are pointer files on this disk. It does not include weather, which Phient has never worked on and does not claim. It does not include the helix as biology. The helix is a file format whose 64-fold code lines up with three others by count.

## 3. The mechanism

A word in phiano is a `SpectralPhasor`. It holds 64 channels. Each channel has a source angle, a target angle and an evidence count, 64 × 12 B per word in the v6 format (docs/79). A channel's angle is one byte: 256 quanta of 2π, 0.0245 rad each (docs/83). A relation has a direction, so there are two sides. `on(a, b, k)` reads how well word `a`'s source side on channel `k` meets word `b`'s target side.

Channels get their meaning from the corpus. `relational.rs` keys a pair of words by the rarest marker word standing between them, and by their offset when there is none. `RelationVocab::learn` counts the words that stand between others and gives the most frequent of them marker channels, 8 to 31, up to `MARKER_CHANNELS_MAX` 24. So in the persona lexicon `of` is channel 10, and in the synthetic quest `is-a` is channel 8, because `is-a` is the word most often found between two others there.

Training moves angles. `Trainer::couple_relations` takes a pair on a channel and nudges the head's source side toward the other word's target side by `RELATION_RATE · sin d = 0.06 · sin d`. It is the only objective of six that writes the target side. `phasor::Rounding::Dither`, set with `Trainer::with_rounding`, rounds the new angle up with probability equal to its fractional quantum. The coin is a hash, so runs stay reproducible.

The operator lives in phinet. `Fno` is configured by `FnoConfig { space 64, width, layers, modes, margin, lr }` with a hinge or a softmax objective. At width 8, two layers and 12 modes it has 8,360 complex parameters. At width 16, three layers and 16 modes it has 50,016. Candidates rank by `cosine_table(operator(ctx), table)`. `FnoConfig::tie = Tie::Sphere` reads the 64 channels as real spherical harmonics `Y_l^m` with `l < 8`, index `k = l² + l + m`. It skips the FFT and learns one complex weight per degree. `Tape::tie` expands eight weights to sixty-four and sums the gradient back. `Tie::Anisotropic` learns one weight per `(l, |m|)`, 36 slots.

Decoding uses the operator through `Decode` in phiano. `OperatorGuide::nearest_forcing` implements `Decode::Forcing { k, lambda }` with `phinet::liskov`'s three operators. `better(pred, a, b)` is `cos(pred, a) − cos(pred, b)`. `worse` is how far a word's best continuation falls below its fit. `loss(pred, target, decoy, margin)` is `max(0, margin − better)`. `forcing_choice` picks the largest `cos − λ·worse`.

`Generator::with_persona` conditions one shared lexicon with a sector prior, clipped at ±2, and the persona's own bigram evidence.

The quest bridge is `bin/quest_ingest`. Each row of a phiecs `relations.tsv` becomes the three-token document `about kind other`, repeated by weight. The helix is `bin/helix write <facet.chroma> <dir>`. `codon(k)` is `k` in base four over `ACGT`.

## 4. What was measured, and the numbers

Every number below stands as in the facts sheet, the sweep table or the doc named. The Austen setting throughout is *Emma*, *Persuasion* and *Sense and Sensibility*, a pool of 1,228 words seen 20 or more times, 204,592 training examples, and chance MRR 0.0063.

| what | setting | number | source |
|---|---|---|---|
| phinet vs PyTorch | every operation, gradient, one Adam step | 62/62 parity tests | facts, docs/79 |
| identity operator | Austen ranking | 7.9× chance | facts |
| hinge circle, 8,360 params | Austen, 3 epochs | 14.7× valid / 14.6× test, hit@10 0.216, 142 s | sweep |
| softmax β=8 circle, 8,360 | Austen, 3 epochs | 18.0× / 18.8×, hit@10 0.246, 395 s | sweep |
| softmax wide+deep, Adam, 50,016 | Austen, 8 epochs | 30.7× / 30.3×, hit@10 0.348, 3189 s | sweep |
| sphere tie, 1,192 | Austen hinge, 3 epochs | 12.8× / 13.1×, hit@10 0.182, 89 s | sweep |
| anisotropic tie, 4,776 | Austen hinge, 3 epochs | 16.0× / 15.6×, hit@10 0.254, 85 s | sweep |
| degree filter | 200 items, 40 epochs | circle 0.662 (identity 0.646), sphere 1.000 | docs/84 |
| `phinet::sphere` vs scipy `sph_harm_y` | 64 harmonics, 8 × 16 grid | max difference 2.4e-15 | docs/84 |
| decoding, KN perplexity | 40 prompts × 30 tokens | sampler 13.3, operator 66.9 (was 97.2) | facts |
| forcing | k = 32 | λ=0.05 69.7, λ=0.25 68.5, λ=1 107.4 | docs/86 |
| persona attribution | three writers, 4-sentence paragraphs | 71% (74/65/86), chance 33% | docs/83 |
| dialogue | 18 replies | 44% sector reader, 72% KN panel | facts |
| quest, shipped trainer | 1,280 rows, 293 targets | MRR 0.082 | docs/83 |
| quest, attraction-only + dither | same | MRR 0.594, hit@1 34.8%, hit@5 97.4%, within kind 0.747 | docs/83 |
| quest held out 20% | two hops | is-a 0.095, used-for 0.093, all 0.045 | docs/83 |
| helix | 18,496 words | 1.9 s, 75 MB | facts |

The sources disagree in four places. docs/75 records 55 of 55 parity checks. docs/79 and the facts sheet record 62 of 62; the later count is used. The shared 4-grams across prompts read 26% and 10% in docs/79, 26.2% and 9.9% in docs/86, and 25.7% and 8.7% in the facts sheet; this paper uses the facts sheet. The helix wrote in 0.6 s in docs/84 and in 1.9 s in the facts sheet. Attribution read 64% in docs/75 and 71% in docs/83.

## 5. The one idea, restated

Everything in section 4 is one idea measured in different places. A word is what it relates to, under a condition. The condition is a channel. The relation is a pair of angles on that channel, one on each side. The count says how often the pair has been played out. Nothing else is stored.

Each part of the machine is a way of reading that idea. The lexicon reads it directly, through `on(a, b, k)`. The operator reads it as a function. A word's 64 channels are a function on the circle, or on the sphere, and the operator learns a map from the context's function to the next word's. The persona reads it as a histogram of conditions. A writer is which conditions that writer puts words under, and how often. The quest reads it as a question. It asks an oracle what `w0` is, what it is near, what it is unlike, and writes each answer on the channel its kind names. The helix reads it as a file. Each channel is a codon, and a word's strand is 64 lines of source byte, target byte and evidence.

The idea also shapes the measurement. A word has no score of its own, only a rank among other words under a condition. So every number in this series is a rank against a pool: MRR against chance, hit@k, attribution against three writers, a Kneser-Ney perplexity from a model that never saw the text. `phinet::liskov` makes the same shape explicit. `better` compares two words under one prediction. `worse` compares a word with what can follow it. `loss` is the hinge over `better`. None of the three gives a word a value on its own.

The idea has a cost, and the measurements show it. A condition that is one byte can hold only as many distinct answers as its quanta allow, and fewer when nudges fall under half a quantum. A lexicon that reads only offset channels cannot see relations written on marker channels. A tokenizer that deletes punctuation cannot put a comma under any condition at all. These are limits of the current encoding. Sections 9 and 13 list them.

This is why the sphere fits. The sum of `2l + 1` for `l` from 0 to 7 is 64. A word's channels can be read as a band-limited function on the sphere with no change to the stored bytes. Whether that helps is measured. On Austen the anisotropic tie says it does, by 16.0× against 14.7×, at about half the parameters. At the softmax, wide and deep setting it has not been asked.

## 6. What is built

The tree has two engines and several applications, and each can be run from a command.

phinet is its own crate. It holds `Tensor` as `[b, w, s]` complex, `Tape` for reverse-mode gradients, a unitary FFT, `Fno`, joint training, holographic reduced representations, sixty-four hexagram gates and `phinet::sphere`. `phinet/examples/torch_reference.rs` dumps every operation, every gradient and one Adam step. `phinet/torch/phinet_torch.py` is the PyTorch side, and the two agree on 62 of 62 checks. `phinet/torch/phinet_colab.ipynb` runs the checks and trains the README's operator in torch from MRR 0.073 to 1.000 (docs/75). `phinet::sphere` gives orthonormal real harmonics on a Gauss–Legendre by equiangular grid, exact on the band to 1e-9, and agrees with scipy to 2.4e-15. phifno 1.0.0 is the child crate that names the spherical operator; its `examples/filter.rs` is the degree-filter task.

phiano is the application crate. docs/82 counted it mechanically: 541 source files, 29,670 lines and 50 binaries. The binaries that carry the measurements are `bin/operator_sweep`, which trains the operator arms and checkpoints every quarter epoch with the optimiser state; `bin/decode_eval`, which compares the trigram sampler with decoding from the function space; `bin/persona_eval` and `bin/impersonate_eval`; `bin/persona_ingest`; `bin/liskov`; `bin/quest_ingest` and `bin/quest_ask`; `bin/helix`; and `bin/bpe_lexicon`. The tokenizer in `src/bpe.rs` reads Phi-4's real `vocab.json` and `merges.txt` and matches Hugging Face `tokenizers` on 10,535 of 10,535 sentences of *Emma*.

phiecs is the entity-component world. Its quest walks from one word outward and writes `relations.tsv`. `phiecs::quest::Synthetic` is an oracle with a hidden structure of 256 words in families of eight standing in five relations. `Quest::from_tsv` reads a table back into a world so a quest can resume.

The helix is a way to write a lexicon to disk. From docs/84:

```
cat codons/AGG/words.tsv                              # `of`: who carries it (channel 10)
awk -F'\t' '$4>0' strands/ca/cat.dna                  # a word's active codons
grep -l $'^AGG\t.*\t[1-9]' strands/*/*.dna | wc -l    # every word with `of` evidence
```

On the synthetic quest's helix, `w0`'s active codons are its five relations plus offset+1: `AGA` is-a, `AGC` near, `AGG` unlike, `AGT` part-of, `ATA` used-for.

The test count has moved over the month. docs/75 recorded 219 library tests in phiano and 15 in phinet. docs/79 recorded 220 in phiano and 29 in phinet. The proposal's attachment list records phinet at 37 tests. The phiano library now has 226 tests, and all 226 have been compiled and passed on the user's Windows machine as well as in the container. That is the first record of the library building off the container.

## 7. What is measured, in depth

The operator sweep is the centre of the record: one binary, `bin/operator_sweep`, run in arms on one Austen pool. `data/sweep/results.md` keeps every arm with its wall-clock.

| arm | params | best epoch | valid × | test × | hit@10 | s |
|---|---|---|---|---|---|---|
| hinge (shipped) | 8360 | 3 | 14.7× | 14.6× | 0.216 | 142 |
| softmax β=8 | 8360 | 3 | 18.0× | 18.8× | 0.246 | 395 |
| softmax wide+deep, AdamW+cosine | 50016 | 3 | 24.5× | 24.3× | 0.311 | 984 |
| softmax wide+deep, Adam | 50016 | 8 | 30.7× | 30.3× | 0.348 | 3189 |
| hinge, sphere tie | 1192 | 3 | 12.8× | 13.1× | 0.182 | 89 |
| hinge, anisotropic tie | 4776 | 3 | 16.0× | 15.6× | 0.254 | 85 |

The objective matters: softmax over the table lifts the circle from 14.7× to 18.0× at the same 8,360 parameters. Width, depth and epochs matter more: 50,016 parameters reach 30.7× at eight epochs. The cosine schedule did not help: 24.5× at three epochs. The sphere at one weight per degree keeps 88–90% of the circle's hinge ranking with a seventh of the parameters; at softmax β=8 it reads only 13.2×. Eight weights per channel pair is a ceiling. The anisotropic tie, which lets cosine and sine orders share one weight per `(l, |m|)`, reads 16.0× and 15.6× at 4,776 parameters in 85 s. It beats the circle's hinge arm on every column shown.

`bin/decode_eval` decodes 40 held-out prompts of 30 tokens, judged by a Kneser-Ney model that never saw the output. The trigram sampler reads perplexity 13.3. Decoding from the operator's nearest words reads 66.9, down from 97.2 before the sweep's operator. The sampler repeats itself across prompts: 25.7% of its 4-grams are shared across prompts, against 8.7% for the operator. The sampler is fluent because it stitches seen trigrams.

Attribution by `bin/persona_ingest` on held-out paragraphs reads 71%: Chesterton 74%, Austen 65%, Carroll 86%. Generation conditioned on a persona, `bin/persona_eval`, reads 53% as the persona asked for with bigram conditioning, against 33% unconditioned and 43% with the sector prior alone. From docs/79, prompt *chapter 11 the*, bigram conditioning. Carroll: "little golden key and hurried her into the garden wall just as well go back to the same thing with you said the queen". Austen: "very first subject on which the sweet sounds of her mother was sincere it must be more advisable". The idiom moves with the persona's pairs. The sentences are still stitched.

The 18-reply dialogue reads 44% as its persona under the sector reader and 72% under a Kneser-Ney panel.

## 8. The quest and the byte

The quest is the one path where the answer is known exactly, and where the encoding's limit was found. The commands, from docs/83:

```
cargo run -p phiecs --release --example quest -- w0 12 600 synthetic 256
cargo run --release --bin quest_ingest -- <dir>/relations.tsv 64 out.md
```

The quest walks from `w0` to depth 12 on 600 questions: 1,280 rows naming 293 target words. `quest_ingest` turns each row into `about kind other`, so `RelationVocab::learn` gives each kind a marker channel. The measure is the rank of the true `other` when `on(about, ·, k)` is read over the pool of 293 on the kind's channel, where chance MRR is 0.021.

The shipped trainer read MRR 0.057 after eight epochs and 0.082 after sixty-four. A probe printed the same number for every one of the 256 `part-of` pairs: `|sin d| = 0.195`. Every pair had stopped eight quanta short. The nudge is `0.06 · sin d`. Once `sin d < 0.2`, about 11°, the nudge is under half a quantum, `round()` writes back the byte it read, and the pair never moves again. 
Dither fixes the mean movement. Negatives were the other half: a kick of `0.5 · rate · sin`, as large as the attraction within 30°, makes the pair jitter. The four arms of docs/83:

| negatives | rounding | kind MRR | hit@1 | hit@5 | within kind |
|---|---|---|---|---|---|
| on (shipped) | nearest (shipped) | 0.082 | 0.5% | 6.2% | 0.303 |
| on | dither | 0.177 | 3.8% | 29.0% | 0.404 |
| off | nearest | 0.092 | 1.0% | 7.0% | 0.330 |
| off | dither | 0.594 | 34.8% | 97.4% | 0.747 |

Read back by name with `bin/quest_ask`, the facet gives `w0 · is-a ⇒ f0`, `w0 · near ⇒ w1`, `w0 · unlike ⇒ w128`, `w176 · part-of ⇒ w211`. Those four are the truth. It also gives `f0 · is-a ⇒ w150`, and `f0` was only ever a target. Evidence is not kept per side, so a word that was only a source on a channel has a random target angle there with evidence behind it.

Dither is not free. On the Austen corpus it cost attribution six points, 71% to 65%, and Carroll twelve. On a corpus the deadband acts as a low-pass filter. Dither ships off by default and on for a quest.

Held out at 20%, one hop reads 0.011, below chance, as the truth ties every unevidenced word. Two hops recover `is-a` at 0.095 and `used-for` at 0.093, the two kinds shared across a family. `part-of`, `near` and `unlike` have one target per word, and stay near chance.

## 9. What is not built, not measured, and not runnable

Four things are not built. The first is distillation from Phi-4. docs/75 writes out the path, Phi-4's next-token distribution restricted to Phiano's pool, KL against `softmax(β · cosine_table)`, then `Fno::from_bytes`, and says it is not written. `refs/Phi-4-multimodal-instruct/` holds pointer files of 133–135 bytes where the weights should be, and the build machine cannot reach huggingface.co. The second is per-side evidence, or a sub-quantum residue of two bits per channel, which docs/83 argues for and records as not built. The third is a generator that reads marker channels. `Facet::under` and `calls_for_under` exist, and nothing generates from them. The fourth is a shared-lexicon operator. `data/austen_operator.fno` is Austen's, so the three-persona dialogue decodes from the trigram tables. phiqlm is planned and nothing of it is built.

Several things are built and not measured. The sphere at softmax and width 16 has not been run; the anisotropic tie at that width and objective is named in docs/84 as the next run. Forcing at depth two on the quest has not been run. The hexagram table of docs/86 has only been filled by the synthetic quest, where it is flat: 256 words over 61 of 64 hexagrams, mean better −0.008, mean worse −0.008, mean loss 0.108 against a margin of 0.1. Torch-harmonics parity, `phinet::sphere` against `th.RealSHT`, is proposed in docs/papers/18 and not measured. Beam search, `decode_beam` in `src/generate/beam.rs`, is implemented and never called. `SectorIndex`, written to make retrieval sub-linear, is called only from its own test. The drift monitor in `lifelong` is never called.

Some paths cannot run. Four inputs are missing from `data/`: `chunks/`, `dictionary_corpus.txt`, `stop_words.txt` and `searle_markers.json`. `ChunkStore::load_all` returns an empty vector with no warning, so every definition lookup finds nothing. `PhiConfig::is_stop_word` is false for every word, so the stop-word filter in every fingerprint ever computed did nothing. `SpeechActAgent` classifies every input as `Assertive`. docs/82 says ten binaries cannot run for want of the first two inputs and then lists eleven: `latency`, `bootstrap_facet`, `roles`, `topdown`, `types`, `context`, `conception`, `relations`, `defstats`, `forgetting`, `ingest`. docs/79 had counted twelve. `latency` is the one that matters. It is the benchmark for learn, recall, correct and unlearn at p50 and p99, which is the project's central claim, and it has never run on this tree.

Some text is not generated. `/api/chat`'s general answer is `WordSelector::synthesize`, "{} concerns {} and {}.", with the manifold supplying only the nouns. An explanation with a Wikipedia hit returns the extract verbatim. `compose` and `impersonate` were fixed in docs/80 and now generate. Chat was not fixed.

Nothing on weather has been done. No partnership has been agreed. The proposal to Professor Anandkumar says in its opening note that nothing in it has been sent or agreed.

## 10. The next three experiments

Each of the three uses a binary that already runs. Each has an outcome that would change what Phient does next.

The first experiment is the quest against Phi-4. The command exists. docs/83 records that the same `quest` example, given `ollama run phi4` in place of `synthetic`, asks a model through the `Spawned` oracle. The rows it writes go through the same `quest_ingest` and the same `quest_ask`. What it measures is whether a model's answers, one word at a time, carry structure a channel can hold. The synthetic quest showed the path from table to facet can carry a known structure at MRR 0.594. It did not show that a model's answers have one. The hexagram table of docs/86 is waiting for this run. On the synthetic quest it is flat. The facts sheet states plainly that there is no Phi-4 run yet. This experiment is the first honest measurement of what a model's answers look like on a channel.

The second experiment is the anisotropic sphere at the shipped setting. The circle reaches 30.7× only at width 16, three layers, softmax and eight epochs. The anisotropic tie has been run only at the hinge setting, where it beats the circle by 16.0× to 14.7×. `bin/operator_sweep` can run it at the shipped width and objective with a change of `FnoConfig::tie`. This is also the joint experiment the proposal in docs/papers/18 would put to Professor Anandkumar: the spherical operator at FourCastNet 3's settings, anisotropic per `(l, |m|)`, softmax, width 16, against the circle at equal parameters. The sweep's cost numbers give its price. The wide softmax arm took 3189 s over eight epochs on this machine.

The third experiment is a shared-lexicon operator for the dialogue. docs/82 ranks it third of the things worth doing before a demo. The dialogue today decodes from trigram tables, and docs/78 measured decoding from the function space at a fraction of the sampler's cross-prompt repetition. The run is to train one operator on `personas.chroma`, the lexicon all three writers share, and decode the 18 replies from it with persona conditioning. The measures are the ones docs/81 already uses: the per-writer Kneser-Ney panel's reading, 72% today, and the sector reader's, 44% today, with shared 4-grams from `decode_eval`.

None of the three needs new code in the phasor. The first needs a machine that can serve phi4 through ollama. The second needs a run the size of the wide softmax arm, which took 3189 s. The third needs a training run of the size the sweep has already made.

## 11. What it depends on, and what depends on it

The family is called PML, Phi machine learning. Every experiment in section 10 touches more than one member of it.

phinet is the parent. It holds `Tensor` in `[b, w, s]` complex, `Tape` autograd, the unitary FFT, `Fno`, joint training, HRR, the 64 hexagram gates and the sphere. Every operator number in this paper is a phinet `Fno` trained through a phinet `Tape`. The 62 parity tests belong to it. The second experiment changes one field of `FnoConfig` and runs through phinet unchanged. `phinet::liskov` supplies `better`, `worse` and `loss`, and `phinet::hexagram::hexagram_of` files each quest word under one of 64 hexagrams.

The children read the same 64 channels their own way. phical is the geometry of the relation circle: sphere, Möbius, Q64, winding and heat, with Q64 double-quantisation loss under 0.01 (docs/75). phifno 1.0.0 is the spherical operator, one complex weight per degree, and its `examples/filter.rs` is the degree-filter task where the sphere reads 1.000. phiton holds light as units, `LightQuantum`, `SpectralBand` and `PhitonColor`; the helix writes a PhiTon wavelength, band and hex colour per codon. phinum is named as a child; inside phiano, `src/phinum` is a second Searle stack of about 1,500 lines that nothing calls (docs/79). phiqlm is planned for quantum work. Nothing of it is built.

phiano is the word application and depends on phinet by path. It holds `Facet`, `Trainer`, generation, personas, the quest bridge and the helix. The first and third experiments run in phiano binaries. Its word-level `phinet` module carries `joint::write_back`, which is where evidence counts are overwritten with learned magnitudes.

phiecs is the entity-component world and the quest. It writes `relations.tsv`, and phiano reads it. phiecs does not depend on phiano. The first experiment starts in phiecs and ends in phiano. phiace is the application workspace in which phiecs sits as a crate.

The dependencies also fix what can break. A change in phinet's `Fno` serialisation would reach every saved `.fno`; the sphere tie was added with `#[serde(default)]` so every saved operator still loads (docs/84). A change in phiano's `SpectralPhasor`, such as per-side evidence, would reach the `.chroma` format, the helix and the quest bridge together. The helix round trip is byte-exact on both sides and tested, so a format change there has a test that would catch it. The facts sheet's family line and docs/84's family paragraph agree on this structure.

Nothing outside the family is a runtime dependency of the measurements. The PyTorch side of parity and the scipy side of the sphere check are references, run separately. Phi-4 appears only as a tokenizer, as model source to read, and as the oracle of the first experiment.

## 12. The next experiment, and its control

Each of the three experiments has a named control. The first is set out in full.

The experiment is a quest against Phi-4. The treatment is `cargo run -p phiecs --release --example quest -- w0 12 600` with the `Spawned` oracle running `ollama run phi4`, the same depth of 12 and the same budget of 600 as the synthetic run. The rows go through `quest_ingest` with attraction only and dithered rounding, the arm that read 0.594, and 64 epochs. The measures are kind-channel MRR against the pool of target words, hit@1, hit@5, the within-kind reading, the wrong-channel reading and the offset reading. The per-hexagram table of `better`, `worse` and `loss` is written alongside.

The control is the synthetic oracle at the same settings. It is `phiecs::quest::Synthetic` with 256 words in families of eight: MRR 0.594, wrong channel 0.015, offset 0.005, 61 of 64 hexagrams, mean loss 0.108. It fixes what the pipeline can recover when the structure is known. A second control is internal to the treatment. The wrong channel, the next kind's, must read at chance on Phi-4's rows as it did on the synthetic rows. If it does not, the channels are not separating kinds, and any kind-channel number is suspect. A third control is a held-out fifth, `PHIANO_QUEST_HOLDOUT=0.2`, where one hop must read near chance and two hops show whether the model's answers share structure across words the way `is-a` and `used-for` did.

There are three outcomes. Phi-4's rows read near the synthetic control, and the wrong channel reads chance: the model's answers are consistent enough for a channel, and the hexagram table is the first one that is not flat. Phi-4's rows read far below the control with the wrong channel at chance: the model's answers are inconsistent across words, and the quest needs a filter on answers before ingest. The wrong channel reads above chance: the kind words the model uses are not separating, and the parser or the kind list is the fault.

For the second experiment the control is the circle at equal parameters, named in the proposal. The anisotropic tie has 4,776 parameters at the hinge setting. The circle's width is set so its count matches the tie's at width 16, and both run softmax for the same epochs on the same pool. The existing 30.7× arm is a reference and not the control, because its parameter count differs.

For the third experiment the control is the dialogue as it stands: the same 18 replies, the same prompts, the same persona conditioning, decoded from the trigram tables. Both are judged by the same KN panel and sector reader, and their shared 4-grams are counted by the same code.

## 13. What is wrong or missing

The generator's fluency comes from trigrams. Operator decoding reads KN perplexity 66.9 against the sampler's 13.3. The operator repeats itself less, 8.7% shared 4-grams against 25.7%, and it reads worse to a trigram judge. Forcing did not help. At λ = 1 it read 107.4 and pulled toward function words. The operator path exists and has not yet produced text as fluent as the sampler's.

The tokenizer deletes punctuation, case, apostrophes and hyphens. `split_sentences` splits on every `.`, so "Mr. Darcy" is two sentences. A persona cannot be told by its commas. This limits every persona number in the paper, and it is not fixed.

Five of six objectives write only the source side. `train_predictive`, `train_sentence`, `Descent::settle`, `Coherence::align`, `Conception::compose_anchored` and `phinet::joint::write_back` leave the target side where it was. Each pass that improves the manifold desynchronises the directed reading that generation and the dialogue use.

`joint::write_back` overwrites evidence counts with exponentiated learned magnitudes, and `as u32` can saturate silently. After one write-back the count is no longer a count.

Evidence is not kept per side. The quest showed the cost: `f0 · is-a ⇒ w150`, a reading from a side that never saw a row. The byte has a ceiling from both ends. The deadband stops nudges under half a quantum, and 256 `part-of` targets on a 256-quantum circle collide.

The persona reader is not discriminative. It weights sectors by how often a writer uses them, and reads 44% where a Kneser-Ney panel reads 72%. The three fingerprints overlap; Austen and Chesterton have similarity 0.98 (docs/79).

The sphere tie at one weight per degree is weaker than the circle on Austen. 12.8× against 14.7× at hinge, 13.2× against 18.0× at softmax. Only the anisotropic tie is ahead, and only at the hinge setting.

The record has some mismatches that are not explained in the sources. docs/75 records the Austen operator at 16.1× chance. The sweep's shipped hinge arm reads 14.7× valid and 14.6× test. docs/82 counts ten non-runnable binaries and lists eleven.

The codebase is wider than the system. docs/82 found 112 public items with no reference anywhere and 162 used only inside their module. 33 constants in `config` have no reader, among them `ROTOR_DIMS` and `META_LR_MULT_MIN`. Six of them went dead the week docs/82 was written, when `Impersonator` was rewritten. `Generator::new` loads Rust-Book weights whatever corpus is loaded.

The engine is allocation-bound. A 1 MB tensor op takes 2–5 ms, and the profile shows the FFT is not the cost. There is no streaming corpus reader and no sampled softmax. docs/79 extrapolates a WikiText-103 lexicon pass at about a day of CPU, and labels that as an extrapolation.

## 14. How it was found

The state in this paper was reached by a sequence of notes, each correcting the last.

docs/64 left a collapse that the v6 phasor format closed (docs/75). docs/69 measured the two-sided lexicon: WikiText-2 directed ranking 15.5× chance, and directed perplexity 142.8 against one-sided 134.2, so two sides still trail on the tail. docs/72 kept source and target apart for good. docs/73 built the learned scorer and took Austen's candidate perplexity from 12.94 to 10.95. docs/74 settled the pair key, measured the Liskov substitution at 238 against 254 random, and found that the hexagram gates do not predict: next-hexagram 0.9% against chance 1.6%.

docs/75 was the first stock-taking. It found the Phi-4 clone to be pointer files, built the BPE tokenizer to 10,535 of 10,535 parity, found it a wash on the relation circle, 7.6× words against 8.0× pieces, and named decoding from the operator as the one change every open number waited on. It recorded 55 parity checks.

docs/76 recorded the Austen–Chesterton fingerprint similarity of 0.98. docs/78 ran the operator sweep to 30.7× and measured decoding from the function space. docs/79 was the demo brief: two read-throughs of all 524 source files, the 62 parity checks, `Generator::with_persona` and 53% persona-conditioned generation, and the list of what is not done. docs/80 made `compose` and `impersonate` generate, 39% to 56% read as asked, KN perplexity 4,478 to 24. docs/81 built the persona dialogue and measured the 44% sector reader against the 72% KN panel.

docs/82 replaced the hand-written inventory of docs/79 with a mechanical one: the compiler with every `#[allow(dead_code)]` stripped, and a reference count for each public item. It found the four silent inputs and the 112 unreferenced items.

docs/83 built the bridge from phiecs. Its first run failed, at 0.082. A probe printed one number, `|sin d| = 0.195`, and the byte deadband was found. Dither and attraction-only took the quest to 0.594, and the persona re-run showed dither costing a corpus six points, so it ships off by default. The held-out run and `bin/quest_ask` followed in the same note.

docs/84 put the word on the sphere and wrote the lexicon as a helix. The first sphere test assumed a per-degree operator could rotate the sphere. It cannot, and the shipped test says so. The isotropic tie came in below the circle, and the anisotropic tie, run later the same day, came in above it. docs/86 built Liskov's three operators and found forcing does not beat greedy on text, and that the quest's hexagram table is flat.

docs/papers/15 recorded the Austen persona run. docs/papers/17 profiled Anima Anandkumar, and docs/papers/18 drafted the proposal. This series of sixteen papers is the latest step.

## 15. What remains

The three experiments of section 10 come first. The sources name a longer list after them.

The phasor needs per-side evidence, or a two-bit sub-quantum residue per channel, which docs/83 argues for and does not build. It would let dither serve a quest without costing a corpus, and stop a side that never saw a row from answering. The `.chroma` format, the helix and the bridge change with it.

The generator needs to read marker channels. `Facet::under` exists and nothing generates from it. Until the generator reads them, a quest's relations are invisible to generation, because `calls_for` reads offset channels only.

The tokenizer needs punctuation and case. Until then a persona lives at the word-bag level whatever else improves, and `split_sentences` keeps breaking "Mr. Darcy".

Every objective needs to write the target side, and `joint::write_back` needs to stop overwriting evidence counts.

Chat needs to generate. docs/82 ranked it first of three things to act on before a demo, because it is the only remaining place where the system says something it did not compose. The other two were the missing `stop_words.txt` and `searle_markers.json`, and the shared-lexicon operator.

The missing inputs need to be supplied or the paths that read them need to fail loudly. `ChunkStore::load_all` should not return an empty vector with no warning. The latency benchmark needs its inputs so the central claim can be measured at p50 and p99.

Large corpora need a streaming reader, sampled softmax, the engine's allocation fix, and `SectorIndex` in retrieval. docs/79 lists these as what learning on large data takes, and none is written.

The sphere needs placing by relation kind. docs/84 names it: put a word's phasors on the sphere by kind, rather than by channel index, which is where degree would start to mean scale of relation. The proposal asks Professor Anandkumar's view on exactly this.

Forcing needs to be tried at depth two on the quest, where a word either has onward relations or does not. The training loop should expose `better`, `worse` and `loss` by name. Beam search should be called or removed. The 112 unreferenced items should be shrunk one module at a time, with the reference count as the test, as docs/82 recommends. 
The helix should become the quest's own output format, so a quest writes strands directly. The three missing Austen novels named in docs/75 should be added.

The proposal to Professor Anandkumar has not been sent. Its joint experiment is the second experiment of this paper. The torch-harmonics parity it offers is not yet measured. Phient has no weather results, and the proposal says so in its own words.

## 16. Sources

Every path below was read for this paper. No source file was changed.

| path | what it supplied |
|---|---|
| `/home/claude/phiano/docs/papers/phient/BRIEF.md` | the form of this series and its rules |
| `/home/claude/phiano/docs/papers/phient/PHIENT_FACTS.md` | the paradigm paragraph, the family, the measured numbers this series may cite, and what is not done or claimed |
| `/home/claude/phiano/docs/75_where_we_are.md` | the first stock-taking by layer, 219 and 15 tests, 55 parity checks, the Phi-4 pointer files, the BPE tokenizer and its 10,535 of 10,535 parity, the words-against-pieces table, and the order of completion |
| `/home/claude/phiano/docs/79_demo_brief.md` | what can be shown, what is not done in four groups, `Generator::with_persona` and the 53% table, the parameter and cost tables, the large-data extrapolation, 220 tests and 29 phinet tests, and the `chapter 11 the` samples |
| `/home/claude/phiano/docs/82_missing_and_unused.md` | the mechanical inventory: 541 files, 29,670 lines, 50 binaries, the four silent inputs, the eleven listed binaries, 112 unreferenced items, the 33 constants, the ranked capability list and the three actions before a demo |
| `/home/claude/phiano/docs/83_quest_ingest.md` | the quest bridge, the byte deadband and its probe, dithered rounding, the four arms and the per-kind table, dither's cost on attribution, reading by name, the held-out table and the commands |
| `/home/claude/phiano/docs/84_sphere_and_helix.md` | the family paragraph, `phinet::sphere`, `Tie::Sphere` and the anisotropic tie, the degree filter, the scipy parity, the helix layout, the codon queries and `w0`'s codons, and what remains on the sphere |
| `/home/claude/phiano/docs/86_liskov_three_operators.md` | `better`, `worse` and `loss`, `forcing_choice`, the forcing table on `decode_eval`, and the quest filed by hexagram |
| `/home/claude/phiano/data/sweep/results.md` | the seven operator arms on Austen with parameters, objective, optimiser, best epoch, valid and test ranking, hit@10 and wall-clock |
| `/home/claude/phiano/docs/papers/18_PROPOSAL_ANANDKUMAR.md` | what Phient would put in front of Professor Anandkumar, the joint experiment and its controls, torch-harmonics parity, what is not asked, the draft message, and the phinet test count of 37 in its attachments |
| `/home/claude/phiano/docs/papers/phient/08_phiecs_and_the_quest.md` | the voice and the front-matter form of an earlier paper in this series; no number was taken from it |

One fact comes from outside these files. The request that commissioned this paper states that the phiano library's 226 tests have now also been compiled and passed on the user's Windows machine, after passing in the container. Sections 1 and 6 cite it as that statement.

Doc numbers cited by name and not read for this paper, docs/64, 69, 72, 73, 74, 76, 78, 80 and 81, and docs/papers/15 and 17, are cited only through what the files above say about them. Where a number is attributed to one of them, it is the number the reading file gives.

## Addendum, 2026-09-28

The run this paper names as next has been made. The anisotropic tie at softmax, width 16, 3 layers, 3 epochs, with 28,512 parameters, reaches 16.5× valid and 16.6× test, hit@10 0.257, in 912 s. The circle at the same shape and epochs reached 27.6× (`data/sweep/results_3ep.md`, doc 78). The sphere ties saturate and the circle stays the shipped operator. The likely cause is that the circle's FFT runs along the channel index, whose order is meaningful in phiano (offsets +1 to +8, then marker relations by frequency), and reading the 64 channels as spherical harmonics discards it. The three-epoch 27.6× row is in `results_3ep.md`, not `results.md`, which resolves the discrepancy noted above.
