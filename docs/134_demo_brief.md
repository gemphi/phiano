# 134 · Demo brief: the tree of knowing in ten minutes, no GPU

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

**Run** (from `C:\Users\phiac\Workspace\gemphi\phinet\crates\phiano`): `powershell -ExecutionPolicy Bypass -File scripts\demo_tree.ps1` (about 4 minutes; `-Quick` skips step 4). phiano is now a crate of the phinet workspace (docs/135); the script finds the workspace's target directory itself. Everything it shows is told from files on disk and read by counts; nothing is trained, nothing calls Phi-4. The first build takes a minute; run it once before the demo.

## The one sentence

A word is a seed: what it relates to, under what conditions, grows from it. We keep the conditions as a tree — story, chapter, paragraph; crate, file, function; session, turn — and every word's relations with their counts under every level of its path. Telling is reading rows in; going on is reading them back under the path the words are drawn into. Nothing is summarised, nothing is retrained, and every step can be shown as a row.

## Step by step

**1. The story as a tree** (`data\story\ch1.txt`). Two paragraphs. The rows the reader makes of one word are printed — `suddenly@story/ch1/p2#1` is `within` the story, the chapter, the paragraph; it has the word `suddenly`; its `pair` is `she suddenly`; its `next` is `had`. Then the student goes on from two words. Say: *"She suddenly"* gives the second paragraph back; *"He"* gives the first and crosses into the second at "she was going", where the words draw the reading out of paragraph one — the log line under the text says where it was drawn and which conditions posted. Point at that line: that is the whole mechanism, visible.

**2. The language in the tree** (`data\text`: *Emma*, 55 chapters, 158,000 words, and the story under one root; 2.97 million rows read in eight seconds, 444 MB). The story is gone on past where it ends: its own text first, then Austen's language under the story's conditions, with the story's phrases drawn back in. Then, inside Emma's first chapter, the student says the chapter back word for word — counts over one telling must. Then what the story does not yet know: eight of its words are said in fewer than three paragraphs of the whole tree (`presumptuous`, `boss`, `investment`, `ceo`, `ai`, …). Say: these are what it would ask the teacher for, and `teach.ps1` is the loop that asks and tells for hours.

**3. Our own context tests** (`context --turns 400,2000`). A session of two thousand turns written as a tree — a file per turn, a hundred turns per block — with ten codes and twenty keys told at random turns, every key set again later. Asked after everything else: 10 of 10 codes, 20 of 20 keys, 20 of 20 latest values, in about ten milliseconds an ask. Say: a Context Language Model (Meta, September 2026) edits its context file to keep a needle alive and trains a policy for it; here nothing was dropped because nothing is kept as context — the latest value wins because the scope told later stands first among equals, which is phiecs's `Condition`, now over was.

**4. The code student on the base** (`code_student --ask`, from Phi-4's 253 saved answers). `count_odd` in `i128`, a type it never saw: the type sibling said again, compiles. `count_zero` in `u64`: the literal goes with the type (it would be `0.0` for `f64`). Then `max_odd`: it compiles and is wrong — it says `max` back and does not hear `odd` under it. Show this one on purpose. Say: this is the honest limit of a student that has read 253 answers, and the chain of requests — ask the teacher to show `odd` in more company — is how it is fixed, not a rule.

## Numbers you may quote (all measured, all in docs/124–133)

- A word from one pointing: the builders' game student keeps a colour-word taught once at 100% over 50 orders; one shown answer of a code family lifts the student from 15% to 61% of its siblings.
- Code, a new type of a known task: 75% working against Phi-4's own 90%, same audited checks.
- Context: 10/10 · 20/20 · 20/20 at 25, 100, 400, 2,000 and 5,000 turns, two seeds; one ask 2–16 ms.
- Store: 2.8 KB a word; a night of teaching (a million words) is about 3 GB.

## What not to say

- Not "it beats Phi-4" or "it beats the CLM". It is behind Phi-4 on code (75 vs 90) and has no number on the CLM's benchmarks; it does a different thing (keeps without choosing).
- Not "it understands". It reads what was told, in the words it was told in; a paraphrased needle is not found yet.
- Not "4% of Phi-4". No measurement supports a ratio.
- Not "the phasors do this". What this demo shows is the paradigm's *count*; the phasor operator is built (phinet) and is not yet on the tree (docs/132 §2). If asked where the Fourier operators are: in the relation board as the comparison line, and next on the tree.

## If something breaks

Every step reads files; if a step prints nothing, the path is wrong (`data\text\emma` must have 55 chapter files; `data\phi4_code` the answer families). `cargo build --release --bin story --bin context --bin code_student` rebuilds everything the demo needs.
