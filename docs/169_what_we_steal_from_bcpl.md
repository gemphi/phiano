# 169 · What we steal from BCPL (not its binary — its approach)

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The build, for the record

The downloaded Cintcode system **builds from its C sources and boots** in this sandbox: `gcc -DforLinux -fcommon` on `cintsys.c cinterp.c kblib.c cfuncs.c fasterp.c extfn.c glfn.c sdlfn.c nrastlib.c` links a 75 KB `bin/cintsys` that prints `BCPL 32-bit Cintcode System (30 May 2013)` and takes commands. (Modern gcc's `-fno-common` default is why the stock `make` fell over; `-fcommon` is the one-flag fix. The shipped *compiler image* in `cin/` is 32-bit Cintcode and segfaults when run on a 64-bit host — the 64-bit image lives in `cin64/`, built by `make sys64`, which is the clean way to run it and the user's to do on their machine.) The friend's claim is confirmed from the inside: the whole system is small enough that a stranger can build and boot it in minutes.

But a booting binary was never the prize. BCPL's binary is a 1967 virtual machine; we are not going to run Phiano on it. What we take is the **approach**, and every piece of it is something we either already do or now do because of it.

## The five things we steal

1. **One tiny intermediate, interpreted everywhere.** BCPL compiles to O-code / Cintcode — ~138 ops — and one small interpreter runs it on any machine. We compile *everything* — prose, Rust, BCPL, the kit, a quest, a scene from the eye — to one intermediate, `Row{about, kind, other, weight}`, and one loop (`World::tell`, `Tally`) interprets it. BCPL is the fifty-year proof that a tiny universal intermediate beats a big native target. **Already ours; now cited.**

2. **A closed vocabulary of structure over an open vocabulary of names.** 65 keywords, unlimited identifiers. That ratio is exactly `MAX_KINDS = 64` relations over an unbounded lexicon — and it is *why* BCPL's open stratum is 2% where English's is 24% (docs/166). The lesson we steal: **keep the kinds closed and small, let the names run free.** It is the shape of `relations.tsv` and the reason the counting model is strong on code and starved on prose.

3. **The compiler is written in itself.** 6,644 lines of BCPL compile BCPL. The student can read its teacher. We steal this directly: `phiplm::code::bcpl` reads the compiler's own source into rows, so `(held bcpl calls)` is the compiler describing itself (docs/168 §5). No English corpus contains its own grammar; a language does.

4. **Execution is the judge.** You do not score BCPL by perplexity — you run it. This is the loop we build next and the one thing a prose model can never have: write → compile → run → keep what executed. The oracle is not a 9 GB model, it is a 75 KB interpreter that answers in milliseconds, deterministically, offline. **The approach that removes the model from the learning loop.**

5. **Typeless: one type, two operators.** BCPL has one type (the word) and `!`/`@` (the value at / the address of). Meaning is position and reference, nothing else — which *is* the Phical paradigm (`w` is only what it relates to, under which `c`). BCPL is the existence proof that a whole systems language needs no type system, only relation. We steal the nerve to keep Phiano typeless in the same sense: a word is its relations, not a class.

## What this unlocks, concretely

- **Now**: BCPL is a persona told from its own source; it races, talks, and answers from counts (docs/168 §5). The reader, the sets folder, the Lisp and the Tally all run on it with no new code.
- **Next (user's machine)**: `make sys64` in `ext/bcpl/cintcode`, then the write→compile→run loop with `cintsys64` as the judge — the first learning loop here with no language model inside it.
- **The principle, kept**: closed kinds, open names; one intermediate; self-description; execution as truth. Four lines of method, stolen clean.

## Status

cintsys built and booted in-sandbox (32-bit image; `cin64` is the run path). The approach is extracted and already wired where it can be. Nothing here needs the binary to proceed.
