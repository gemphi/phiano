# 130 · The story asks: the chain of requests for prose, and a faster cast

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. What the story does not know

docs/124 §3 gave the code student its chain of requests: a word told in fewer than three functions is not known, and the student asks Phi-4 to show it in use. The same for prose (`prose::unknown`, `story --in text/story --unknown words.tsv`): a word of the story said in fewer than three paragraphs of the whole tree — *Emma* and the story together — cannot be told from the paragraph it came in, because its counts are that paragraph's. Of the story's seventy-eight words, eight: `presumptuous`, `pointing`, `struggling`, `boss`, `investment`, `demand`, `ceo`, `ai`. Each is listed with the words around it, and `story --uses words.tsv ollama run phi4-builder` asks the teacher for three sentences using the word as it was used there, saved as `data\text\uses\<word>.txt` — one more paragraph in the tree, told with the rest the next time the tree is read. A second round asks for what is still unknown (with a stub teacher that answers, the second round finds nothing left). `scripts\phi4_story.ps1` runs three rounds unattended and writes the story going on, before and after, to `data\text\story_go.md`. Nothing is hard-coded: what is asked comes from the counts, what is learned comes from the files.

## 2. A faster cast

docs/129's one ask took 405 ms at five thousand turns because the cast built every child scope's whole `says` distribution (150,000 words and pairs) to score it. The score needs only the asked conditions' counts, so the cast now reads those — `(n_c + base_c) / (total + 1)` against the base, exactly what `dist` would give at that index — and builds the whole distribution only for the scopes it enters, which become their children's base. `Counts::values_and_prior` keeps each kind's sorted values and prior between readings and drops them when more is learned. Same answers (10/10, 20/20, 20/20 at every length, both seeds); one ask 2 ms at 25 turns, 10 ms at 400, 22 ms at 2,000, 33 ms at 5,000.

## 3. Waiting on the teacher

Three runs now need the machine with Phi-4, each one unattended script: `phi4_code.ps1` (the code student's uses rounds and the three tests), `phi4_sepia.ps1` (the needle on the teacher's side, paper 24 §7), `phi4_story.ps1` (this). docs/131 is what they return.

`story data\text --in text\story --unknown words.tsv` · `story data\text --uses words.tsv ollama run phi4-builder` · `scripts\phi4_story.ps1`
