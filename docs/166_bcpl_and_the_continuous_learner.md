# 166 · BCPL, pulled in: what a 65-word language teaches Phiano's continuous learner

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. What was pulled in

Martin Richards' BCPL Cintcode distribution, now at `gemphi/ext/bcpl` with a `PROVENANCE.txt` like the other reference libraries. The Cambridge host refuses this sandbox, so the copy is the GitHub mirror `8l/bcpl` (commit bad6eec, 2015-10-18; `TGZDATE` 27 July 2015; README revised June 2013). The live `bcpl.tgz` on Richards' page (last modified 31 March 2022) is newer; the user's friend will have that one, and the differences are small. 77 MB without `.git`: 675 BCPL source files, 382,467 lines — the compiler, the Cintcode interpreter in C, 157 commands under `cintcode/com`, 473 programs under `bcplprogs` (demos, dynamic programming, cryptography, an EDSAC simulator, games), the 64-bit and native-code variants, the manual's sources.

## 2. How small it is, read from its own tables

The compiler's reserved-word table (`bcplfe.b`, `declsyswords`) has **65 words**: AND ABS BE BITSPERBCPLWORD BREAK BY CASE DO DEFAULT EQ EQV ELSE ENDCASE FALSE FINISH FIX FLOAT FOR GOTO GE GR GLOBAL GET IF INTO LET LV LE LS LOGOR LOGAND LOOP LSHIFT MANIFEST MOD NE NEEDS NEQV NOT OF OR RESULTIS RETURN REM RSHIFT RV REPEAT REPEATWHILE REPEATUNTIL SECTION SLCT STATIC SWITCHON TO TEST TRUE THEN TABLE UNLESS UNTIL VEC VALOF WHILE XOR. The parse tree has **162 node kinds** (`bcplfecg.h`); the Cintcode interpreter dispatches on **138 opcodes** (`cinterp.c`, 1,005 lines of C); the whole compiler — front end, translator, code generator — is **6,644 lines** of BCPL and compiles itself. One type: the machine word. Two operators do what a type system does elsewhere — `!` (the word at) and `@` (the address of) — so every program is words relating to words by position. That is our paradigm said in 1967.

## 3. What is measured

The same reader on BCPL and on Emma — every line a sentence, `phiplm::prose::tokens` cutting it, one tenth held out by hash — with three numbers on the held-out part: how many of its word-pairs were already told in training (the *told* stratum of docs/152–157), the top-1 hit rate of the plain bigram count (the ladder's rung 2), and Kneser-Ney perplexity (`phiplm::baseline`). `examples/bcpl.rs`.

| corpus | train tokens | vocabulary | held-out pairs already told | bigram top-1 | KN perplexity |
|---|---|---|---|---|---|
| BCPL, the whole distribution | 2,657,127 | 23,878 | **97.9%** | 31.4% | **11.4** |
| BCPL, the compiler's commands only (size-matched to Emma) | 158,725 | 3,342 | 95.9% | 35.3% | 9.4 |
| English, *Emma* | 180,344 | 6,942 | 76.4% | 17.9% | 110.6 |

The open stratum — the pairs a reader has never been told — is **2% of BCPL and 24% of English**, at matched size. That is the whole difference between the two regimes this system has lived in: on prose, docs/157 found the open stratum the standing problem and the real-world prior the only lever; on BCPL the open stratum nearly does not exist, the bigram alone names the next token a third of the time, and perplexity is a tenth of English's. The friend's impression — that an old, simple language is unusually learnable — is a measurement, and this is it.

## 4. The ideas that work, for the continuous learner

**O-code was Row.** BCPL's portability came from compiling to a tiny intermediate code and interpreting it everywhere (O-code in 1967, Cintcode now, 138 ops). Phiano already does this: prose, Rust, the kit, a quest's answers all compile to `Row{about, kind, other, weight}` and one loop (`World::tell`, `Counts`) interprets them. BCPL is the fifty-year-old proof that the design holds; its Cintcode listing is also a second, lower trace to learn — the op sequence under the source — and `next` over opcodes is a relation like `next` over words.

**The compiler is a free, instant oracle.** `code_student` and `code_write` (docs/120–130) learned Rust by writing it and asking rustc, which is slow and vast. `cintsys` is a thousand lines of C: compile-and-run is milliseconds, deterministic, local, and needs no phi-4. The continuous loop — write a program from the counts, compile, run, tell the rows of what passed, repeat — closes without a model in it. 473 programs with runnable demos are a held-out set with execution as the judge, which no prose corpus can offer.

**The student can read its teacher.** The compiler is 6,644 lines in the language it compiles. A reader that learns BCPL from the distribution learns, among its 382k lines, the definition of BCPL — the language's own grammar as text in the language. Nothing in the English corpus corresponds to that.

**Sixty-five words is one table.** The reserved words number 65; `phiecs::relation::MAX_KINDS` is 64, and the circus, the race and the sets all run over one table of that size. A BCPL reader whose kinds *are* the keywords — `LET`, `FOR`, `VALOF`, `RESULTIS`, `!`, `@` — fits the registry with one word to spare; the 162 tree kinds fit three tables; the 138 opcodes two. The language is the size of the system's unit of structure, which is why it feels learnable: a closed vocabulary of relations over an open vocabulary of names, exactly the shape of `relations.tsv`.

**GLOBAL is a world.** BCPL programs share one global vector of numbered slots (`GLOBAL { start:1; rdch:38 … }`) that separately compiled sections bind into by number, not by linker magic. That is a `World` with named channels, and `GET "libhdr"` is a persona's `inherit`.

## 5. What we can achieve, in order

1. **A BCPL reader into rows** (a week's work in `phiplm::code`, the way Rust is read by `syn`): token stream + the 65-keyword grammar → `within`, `says`, `calls`, `declares`, `next`; the sets folder and the Lisp then run on BCPL as they run on Emma. The measurement above says the counting reader will be strong here from the first day.
2. **Generation judged by execution.** Resurrect `code_write`'s loop against `cintsys`: write → compile → run → tell. The first honest number is the share of generated programs that compile, then the share whose output matches a held-out program's. Both are measurable in an afternoon once the build runs — on the user's machine, since this sandbox may not build downloaded code.
3. **Cintcode as the second trace.** Read the `.cin` listings beside the `.b` sources: two relations, source `next` and opcode `next`, and the compiler as the map between them — a learned X ⇒ Y the operator (`phiplm::operator`) was built for.
4. **A persona that speaks BCPL.** A `bcpl` persona whose `kinds` are the keywords and whose quests are the 473 programs; `(held bcpl let)` in the Lisp is then the set of everything ever bound.

## 6. Status

Pulled in and measured 2026-10-24; `examples/bcpl.rs` and this doc synced. The build-and-run proof (`make` in `cintcode/`, then `cintsys`) is the user's to run.

Sources: [BCPL — Martin Richards' page](https://www.cl.cam.ac.uk/~mr10/BCPL.html) · [The BCPL Cintsys and Cintpos User Guide](https://www.cl.cam.ac.uk/~mr10/bcplman.pdf) · [GitHub mirror 8l/bcpl](https://github.com/8l/bcpl) · [History of BCPL — Software Preservation Group](https://softwarepreservation.computerhistory.org/BCPL/)
