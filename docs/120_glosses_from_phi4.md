# 120 · Glosses from Phi-4: what a word says in code

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

**Where docs/119 left it.** Writing by proportion reached 25% working functions, against Phi-4's 90%.

**Trials since docs/119, all measured on the same 248 tasks and checks:**

- **Chaining two word edits** (`count_even` → `sum_odd`): 25% → 25%. No task needed it, given the families we have.
- **A vote among routes**, where the body most proportions arrive at goes first: 25% → 25%.
- **All literals kept as themselves in the writer** (`"aeiouAEIOU"`, `5`, `' '`, not only `0`, `1`, `2`): 25% → 25%. It changes what the string families can say, but no string family has a proportion to say it with.

**Where the 75% fail.** The failures were read family by family; three causes stand out.

1. **The word has no partner pair** (`below` when `count_below` is held out). The edit `above` → `below` cannot be learned from two answers, because no second pair has those words.
2. **The teacher's own slip propagates.** Phi-4's `sum_odd` squares the odd numbers before summing, so `sum_even` written by proportion from it does too.
3. **The word occurs nowhere else** (`caesar`, `palindrome`, `lcm`, `anagram`): no relation to borrow.

**This trial: glosses** (`code_quest --glosses`, `code_write/gloss.rs`).

- **The question.** Phi-4 is asked one question per word of the task names (72 words): which code token says it. The answer is one of `+ - * / % < > <= >= == != && || ! abs max min sum product count filter rev len none`.
- **The writer.** A learned answer whose name differs from the new one in glossed words is said again with each word's token exchanged: `count_above`'s `y > x` becomes `count_below`'s `y < x`.
- **Phient terms.** A gloss is one relation of the word, `w ⇒ token`, taught by the teacher under the condition "in code". It is the first relation the student asks for because it lacks it.

**A check of the code path, not a result.** With a gloss file written by hand for two words only (`above ⇒ >`, `below ⇒ <`), the gloss row goes from 62 to 66 working, and `count_below` is written right in every type. Those glosses were not Phi-4's. The real row needs Phi-4's 72 answers.

**Run:** `scripts\phi4_code.ps1`. The 253 answers are kept; it asks the 72 gloss questions (short replies), then writes and scores.

## Result (Phi-4's 72 glosses, run 2026-10-03)

| writer (each family held out; Phi-4's other answers only) | compiles | **works** |
|---|---|---|
| family resemblance, else beam | 89% | 6% (16) |
| proportion, else family resemblance, else beam | 91% | 25% (61) |
| **+ Phi-4's glosses** | 91% | **28% (70)** |
| **Phi-4 (the teacher)** | – | **89% (221)** |

**What Phi-4 glossed** (`data/phi4_code/glosses.tsv`):

- **Useful pairs:**
  - `above ⇒ >` and `below ⇒ <`;
  - `max ⇒ max` and `min ⇒ min`;
  - `sum ⇒ sum` and `product ⇒ product`;
  - `equal ⇒ ==`, `square ⇒ *`, `abs ⇒ abs`.
- **Useless pairs:** `even ⇒ %` and `odd ⇒ %` are the *same* token, so no exchange can tell them apart. `positive ⇒ +` and `negative ⇒ -` are signs, while the bodies say `x > 0` and `x < 0`.
- **No gloss:** 21 words got `none`, including `zero`, `palindrome`, `anagram` and `caesar`.

**Reading.** Glosses add 9 working functions (61 → 70), most from `above`/`below`. A one-token gloss is too narrow: the meaning of `even`, `odd`, `positive`, `negative` or `zero` is a short *expression* (`x % 2 == 0`, `x > 0`), not a token. The next quest asks Phi-4 for each word's expression over `x` and exchanges expressions instead of tokens. That is the same relation, `w ⇒ code`, at the size the word actually has.
