# 186 · The person — your condition of satisfaction, measured

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The condition, in the user's words

Learn a person from a little of their words, on the fly; pick up their word-choices and preferences; keep learning as more is fed, with no retraining; and be able to talk to it. Measured by speed of learning, not benchmarks.

## The run: examples/person.rs

Any corpus, read in chunks; before each chunk is told, every next word in it is predicted (prequential — nothing is scored on what it was trained on); after each chunk the learner ANSWERS prompts in the voice learned so far. Two arena readers on the same positions: counts, and concepts (the family, backing off). No model anywhere; 176,000 words in two seconds.

### Raymond Murphy — English Grammar in Use, the whole book (22,197 sentences, 4,797-word vocabulary, floor 0.02%)

After the FIRST chunk (17,000 words): you can → "you can use the present perfect continuous and simple"; she has been → "she has been working here since 2010"; it is not → "it is not necessary to do something"; i have → "i have to go to the party but i don[t]". Unseen next-word: concepts 19.5% from chunk 1, 22.6% by chunk 2, plateau near 22%, 18.2% on the last chunk — which is the answer key, a different register. Counts 16–19% on the same positions; concepts above counts in every chunk. Verdict printed: PARTIAL — pickup immediate and far above floor, concepts ≥ counts, but first→last does not rise because the book ends in another register. Reported as measured.

### Tom Crick — the BCPL compiler dissertation (34,000 words, 5 chunks)

the front end → "the front end for bcpl using the gnu compiler collection gcc"; the compiler → "the compiler is a good example from the c front [end]". Unseen next-word 12.1% → 21.0% chunk by chunk (+9 pts): rising, as the condition asks, on a corpus a fifth the size.

### The dictionary voice (glosses, ~3,000 lines)

a person who → "a person who goes before or announces the coming of another" — a herald, defined from counts. Unseen next-word 40–42%.

## What is met and what is not

Met: a little is enough (one chunk), the voice is recognisably the person's, it keeps learning with nothing retrained, and it talks back. Not met as stated: the Murphy curve does not rise end to end, because that book is one register until its key; Crick's does. The honest reading is that pickup is fast and then the person has been learned — the plateau is the corpus running out of new word-choices, not the learner. The remaining distance to the friend's demo is conversation rather than continuation (ask it questions, get answers in the voice), and that is the circus and the quest on the user's machine.

## Status

2026-10-25. examples/person.rs; the book and the dissertation as text under crates/phiano/data/person/ so the run reproduces on the machine: cargo run --release -p phiano --example person -- crates/phiano/data/person/murphy.txt --chunks 10 --prompt "you can".
