# 165 · The language machine: a Lisp over Russell's algebra over the counts

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. What they put forward

Leibniz asked for a *characteristica universalis* and a *calculus ratiocinator* — a script for every concept and a calculus to settle disputes by computing. Frege wrote the first piece (the *Begriffsschrift*, 1879) and split a word's sense from its reference. Peirce and De Morgan worked out the logic of relatives; Russell and Whitehead gathered it into *Principia* — relations as sets of pairs, the theory of types — and Russell's logical atomism asked for an ideal language mirroring facts one to one. Tarski showed truth is defined from outside the language. Wittgenstein pushed the ideal language to its limit (*Tractatus*) and then withdrew it (*Investigations* §43: meaning is use; a language is a family of games). Church's lambda calculus was the calculus; McCarthy's Lisp (1958–60) made it a machine, and his *Advice Taker* (1959) was the proposed language machine: a program holding sentences in a formal language and reasoning with them. Lisp's gift was homoiconicity — a program is a list, data is a list, the machine reads and writes its own sentences.

## 2. The machine

`phinet::sys::lisp`: a reader, `eval`, and the special forms (`quote if define lambda let begin and or`, with `pairs` and `class` as data constructors), 46 primitives, 520 lines in four files, no dependencies. An unbound symbol is a word and stands for itself, so `(held russell is-a)` reads as a row does. The primitives are the whole of Russell's algebra from docs/164 — `union meet difference compose converse closure ancestral power domain range field image relata referents restrict-domain restrict-range product identity included? transitive? symmetric? reflexive?` — plus the stories' sets (`next`, `says`, `scopes` over a sets folder), the personas' three moves (`held`, `ask`, `tell`, with `told` and `beings`), and the little a Lisp needs (`list car cdr cons length = < + - * print str count members`). `Machine::new().with_scopes(…).with_beings(…)`; `examples/lisp.rs` is the REPL, loading `--text` and any number of `--personas` rings.

Nothing in it infers. Composition and closure produce pairs no one was told — a syllogism — and the only form that changes a world is `tell`, which turns pairs into rows under a kind. That is Leibniz's calculator on Wittgenstein's counts: the machine computes over the sets, and the sets come from use.

## 3. A session, measured

Over `data/text` (2,373 scopes) and both rings (256 personas, stand-in quests):

```lisp
(count (next "scope:text/emma"))                                   ; 62568 — Emma's "is followed by"
(count (closure (next "scope:text/emma/ch01/p1")))                 ; 1269 — reachable in the first paragraph
(members (relata (next "scope:text/emma/ch01") emma))              ; what follows "emma" in chapter 1: was, woodhouse, smiled, knows …
(count (meet (next "scope:text/emma/ch01") (next "scope:text/emma/ch02")))  ; 321 pairs the two chapters share
(define degree (lambda (r w) (count (relata r w))))
(degree (next "scope:text/emma") the)                              ; 1525 distinct words follow "the"
(define syllogism (difference (closure (held bertrand-russell is-a)) (held bertrand-russell is-a)))
(count syllogism)                                                  ; 97 pairs Russell never said, that follow from what he did
(told whitehead) (tell whitehead syllogism is-a) (told whitehead)  ; 166 → 263: Whitehead now holds them, as rows
(transitive? (next "scope:text/story/ch1"))                        ; #f — a story's order is not an order
```

## 4. Training, meanwhile

The operator over Emma (`examples/operator_run`, resumed) is at epoch 5, hinge loss 0.0229 → 0.0200, ~3 min/epoch; its read-backwards measurement at the last checkpoint: told pairs 15.0% top-1 · 0.215 MRR against frequency's 7.4% · 0.163; never-told pairs 0.1% · 0.010 against frequency's 1.1% · 0.042 — the open stratum of docs/157 again, from the operator's side. Numbers to the log when it ends.

## 5. Status

Done 2026-10-13: `sys::lisp` (3 tests), `examples/lisp.rs`, the session above. Next: `(tell …)` from the REPL written back to a quest folder (so a session persists), a `load`/`save` pair, and the Lisp as the personas' own voice — a persona's answers as S-expressions the machine can run.
