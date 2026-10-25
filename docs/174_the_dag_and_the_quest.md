# 174 · The DAG, and the quest that completes it

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The rung

docs/172 ended on the finding: tense constructions and collocations are different AXES of the same sentence, so forcing each sentence into one discovered group makes the axes fight. This rung builds the DAG: a sentence belongs to EVERY recurring construction family it shows.

**New operators** (`prose/family.rs`): `Knowing::families(at_least)` — each adjacent known-word pair shown by at least two paragraphs, with its member paragraphs (the DAG membership); `Knowing::unsure(at_least)` — the constructions shown exactly once: real enough to have been said, not yet a family, and therefore exactly what the quest asks the teacher for more uses of (`family_prompt`, the builders' move from docs/130 applied to pairs instead of words). This is the interest that grows: each answer either founds a family or stays a one-off.

## Conditions of satisfaction, fixed in advance

(a) coverage: every one of Murphy's four units is the majority unit of at least one family; (b) member-weighted purity ≥ 85%; (c) holding or improving from round 1 (the book alone) to round 2 (after the teacher's twenty extra uses).

## Measured (student, lesson 0)

| round | families | coverage | purity |
|---|---|---|---|
| 1 — book alone (21 sentences) | 6 | 3/4 | 67% |
| 2 — after the teacher's uses (41) | 22 | **4/4** | **85%** |

Round 1 does NOT meet the condition — six families, the present-perfect axis not yet the majority anywhere, purity 67%. Round 2 meets all three: coverage 4/4, purity 85%, both improved. The largest families in round 2 are the tense constructions and the collocations side by side — (did not)×3, (to the)×3, (do not)×2, (does not)×2, (a lot)×2 — each on its own axis, no longer fighting.

The point is not that round 1 fails; it is WHY it fails and what fixes it. Twenty-one sentences cannot make most constructions recur, so the student is `unsure` of 18 of them — and the quest is precisely the request for the uses that turn one-offs into families. The teacher's answers (never the unit names, only uses) are what carried every condition. Discovery is not a pass over a corpus; it is the loop: listen → find families → name what you are unsure of → ask → listen again. On the machine the asking goes to ollama; in the sandbox the SHOWN bank answers it.

## Where Q2 stands

Resolution under given concepts: met (docs/172, the practice table). Discovery: met at the DAG rung WITH the quest loop, on a small curriculum. Next rung: scale the loop live — wire `unsure` → `family_prompt` → ollama (`quest ask` on the machine), tell the answers back with `Knowing::tell`, and watch families, coverage and purity as the loop runs unattended; condition: purity holds ≥ 85% while families grow past 100.

## Status

2026-10-24. `prose/family.rs` (+ tests), student lesson 0 rewritten to the DAG, student.rs at 201 lines, workspace tests green.
